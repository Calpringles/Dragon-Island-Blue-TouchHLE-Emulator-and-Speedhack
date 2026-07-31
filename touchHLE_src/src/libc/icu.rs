/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `libicucore.dylib`: the parts of ICU that iOS exposes as C functions.
//!
//! Only the regular-expression API (`uregex.h`) and `u_strlen` are implemented,
//! because that is what RegexKitLite — which apps of this era commonly compile
//! in — calls. RegexKitLite itself is guest code; only these escape to us.
//!
//! ICU works in UTF-16 and every index it reports is a UTF-16 code unit offset,
//! while the Rust regex engine works in UTF-8. [RegexState] keeps the subject
//! string in both forms plus the offset mapping between them, so the boundary
//! conversion happens in exactly one place.

use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::{ConstPtr, GuestUSize, MutPtr, MutVoidPtr};
use crate::Environment;
use regex_lite::Regex;
use std::collections::HashMap;

#[allow(non_camel_case_types)]
type UChar = u16;
#[allow(non_camel_case_types)]
type UErrorCode = i32;
#[allow(non_camel_case_types)]
type UBool = bool;

const U_ZERO_ERROR: UErrorCode = 0;
const U_ILLEGAL_ARGUMENT_ERROR: UErrorCode = 1;
const U_INDEX_OUTOFBOUNDS_ERROR: UErrorCode = 8;
const U_BUFFER_OVERFLOW_ERROR: UErrorCode = 15;
const U_REGEX_RULE_SYNTAX: UErrorCode = 66562;

// Option flags accepted by uregex_open().
const UREGEX_CASE_INSENSITIVE: u32 = 2;
const UREGEX_COMMENTS: u32 = 4;
const UREGEX_DOTALL: u32 = 32;
const UREGEX_MULTILINE: u32 = 8;

/// The handles we hand back as `URegularExpression *`. They are never
/// dereferenced by the guest, only passed back to us.
#[allow(non_camel_case_types)]
pub type URegularExpression = MutVoidPtr;

/// Handles are allocated in guest memory so that each is a distinct, non-null
/// pointer value, and stay alive until `uregex_close()`.
#[derive(Default)]
pub struct State {
    regexes: HashMap<MutVoidPtr, RegexState>,
}
impl State {
    fn get_mut(env: &mut Environment) -> &mut Self {
        &mut env.libc_state.icu
    }
}

struct RegexState {
    regex: Regex,
    /// The subject string set by `uregex_setText()`, in UTF-8.
    text: String,
    /// UTF-16 index for each UTF-8 byte offset in `text`, plus a final entry for
    /// the end of the string.
    utf16_index: Vec<i32>,
    /// UTF-8 byte offset for each UTF-16 index, plus a final entry.
    utf8_offset: Vec<usize>,
    /// Capture spans of the most recent successful match, in UTF-16 indices.
    /// Group 0 is the whole match; a group that did not participate is [None].
    groups: Vec<Option<(i32, i32)>>,
    /// Where the next `uregex_findNext()` starts, in UTF-16 indices.
    next_start: i32,
    /// Where `uregex_appendReplacement()` last stopped copying, in UTF-16
    /// indices.
    append_position: i32,
}

impl RegexState {
    fn set_text(&mut self, text: String) {
        // Build both directions of the offset map once per subject string.
        let mut utf16_index = Vec::with_capacity(text.len() + 1);
        let mut utf8_offset = Vec::new();
        let mut u16_pos: i32 = 0;
        for (offset, c) in text.char_indices() {
            let width = c.len_utf8();
            for _ in 0..width {
                utf16_index.push(u16_pos);
            }
            for _ in 0..c.len_utf16() {
                utf8_offset.push(offset);
            }
            u16_pos += c.len_utf16() as i32;
        }
        utf16_index.push(u16_pos);
        utf8_offset.push(text.len());

        self.text = text;
        self.utf16_index = utf16_index;
        self.utf8_offset = utf8_offset;
        self.groups.clear();
        self.next_start = 0;
        self.append_position = 0;
    }

    fn utf16_len(&self) -> i32 {
        *self.utf16_index.last().unwrap_or(&0)
    }

    fn to_utf16(&self, byte_offset: usize) -> i32 {
        self.utf16_index
            .get(byte_offset)
            .copied()
            .unwrap_or_else(|| self.utf16_len())
    }

    fn to_utf8(&self, utf16_index: i32) -> Option<usize> {
        if utf16_index < 0 {
            return None;
        }
        self.utf8_offset.get(utf16_index as usize).copied()
    }

    /// Run the regex from `start` (a UTF-16 index), recording the capture spans.
    fn find_from(&mut self, start: i32) -> bool {
        let Some(byte_start) = self.to_utf8(start) else {
            self.groups.clear();
            return false;
        };
        let Some(captures) = self.regex.captures_at(&self.text, byte_start) else {
            self.groups.clear();
            return false;
        };
        let spans: Vec<Option<(usize, usize)>> = captures
            .iter()
            .map(|group| group.map(|m| (m.start(), m.end())))
            .collect();
        self.groups = spans
            .iter()
            .map(|span| span.map(|(s, e)| (self.to_utf16(s), self.to_utf16(e))))
            .collect();
        let (_, whole_end) = self.groups[0].unwrap();
        let (whole_start, _) = self.groups[0].unwrap();
        // An empty match must still make progress, or findNext() would spin.
        self.next_start = if whole_end == whole_start {
            whole_end + 1
        } else {
            whole_end
        };
        true
    }

    /// The matched text for a group, or [None] if it did not participate.
    fn group_text(&self, group: usize) -> Option<&str> {
        let (start, end) = (*self.groups.get(group)?)?;
        let start = self.to_utf8(start)?;
        let end = self.to_utf8(end)?;
        self.text.get(start..end)
    }
}

fn set_status(env: &mut Environment, status: MutPtr<UErrorCode>, code: UErrorCode) {
    if !status.is_null() {
        env.mem.write(status, code);
    }
}

/// Whether the guest has already reported a failure through this status
/// out-parameter. ICU functions are no-ops once that has happened.
fn status_is_failure(env: &Environment, status: MutPtr<UErrorCode>) -> bool {
    !status.is_null() && env.mem.read(status) > U_ZERO_ERROR
}

fn read_utf16(env: &Environment, chars: ConstPtr<UChar>, length: i32) -> String {
    if chars.is_null() {
        return String::new();
    }
    let length = if length < 0 {
        u_strlen_inner(env, chars)
    } else {
        length
    };
    let units: Vec<u16> = (0..length as GuestUSize)
        .map(|i| env.mem.read(chars + i))
        .collect();
    String::from_utf16_lossy(&units)
}

/// Translate ICU's option flags into the inline flags the Rust engine takes.
/// Returns [None] for a flag we cannot honour, since quietly ignoring one would
/// silently change what the pattern matches.
fn pattern_with_flags(pattern: &str, flags: u32) -> Option<String> {
    let mut inline = String::new();
    if flags & UREGEX_CASE_INSENSITIVE != 0 {
        inline.push('i');
    }
    if flags & UREGEX_MULTILINE != 0 {
        inline.push('m');
    }
    if flags & UREGEX_DOTALL != 0 {
        inline.push('s');
    }
    if flags & UREGEX_COMMENTS != 0 {
        inline.push('x');
    }
    let known = UREGEX_CASE_INSENSITIVE | UREGEX_MULTILINE | UREGEX_DOTALL | UREGEX_COMMENTS;
    if flags & !known != 0 {
        return None;
    }
    if inline.is_empty() {
        Some(pattern.to_string())
    } else {
        Some(format!("(?{}){}", inline, pattern))
    }
}

#[allow(non_snake_case)]
fn u_strlen(env: &mut Environment, s: ConstPtr<UChar>) -> i32 {
    u_strlen_inner(env, s)
}

fn u_strlen_inner(env: &Environment, s: ConstPtr<UChar>) -> i32 {
    if s.is_null() {
        return 0;
    }
    let mut length: GuestUSize = 0;
    while env.mem.read(s + length) != 0 {
        length += 1;
    }
    length as i32
}

#[allow(non_snake_case)]
fn uregex_open(
    env: &mut Environment,
    pattern: ConstPtr<UChar>,
    pattern_length: i32,
    flags: u32,
    _parse_error: MutVoidPtr, // UParseError *
    status: MutPtr<UErrorCode>,
) -> URegularExpression {
    if status_is_failure(env, status) {
        return MutVoidPtr::null();
    }
    let pattern_string = read_utf16(env, pattern, pattern_length);
    let Some(with_flags) = pattern_with_flags(&pattern_string, flags) else {
        log!(
            "uregex_open: unsupported option flags {:#x} for pattern {:?}",
            flags,
            pattern_string
        );
        set_status(env, status, U_ILLEGAL_ARGUMENT_ERROR);
        return MutVoidPtr::null();
    };
    let regex = match Regex::new(&with_flags) {
        Ok(regex) => regex,
        Err(e) => {
            log!("uregex_open: could not compile {:?}: {}", with_flags, e);
            set_status(env, status, U_REGEX_RULE_SYNTAX);
            return MutVoidPtr::null();
        }
    };

    // A distinct non-null pointer the guest can hold on to. One byte is enough:
    // the guest never looks inside it.
    let handle = env.mem.alloc(1);
    State::get_mut(env).regexes.insert(
        handle,
        RegexState {
            regex,
            text: String::new(),
            utf16_index: vec![0],
            utf8_offset: vec![0],
            groups: Vec::new(),
            next_start: 0,
            append_position: 0,
        },
    );
    set_status(env, status, U_ZERO_ERROR);
    handle
}

#[allow(non_snake_case)]
fn uregex_close(env: &mut Environment, regex: URegularExpression) {
    if State::get_mut(env).regexes.remove(&regex).is_some() {
        env.mem.free(regex);
    }
}

#[allow(non_snake_case)]
fn uregex_setText(
    env: &mut Environment,
    regex: URegularExpression,
    text: ConstPtr<UChar>,
    text_length: i32,
    status: MutPtr<UErrorCode>,
) {
    if status_is_failure(env, status) {
        return;
    }
    // ICU does not copy the text, it borrows the caller's buffer. Copying is
    // safe here because callers do not mutate the buffer while it is set.
    let text = read_utf16(env, text, text_length);
    let Some(state) = State::get_mut(env).regexes.get_mut(&regex) else {
        set_status(env, status, U_ILLEGAL_ARGUMENT_ERROR);
        return;
    };
    state.set_text(text);
    set_status(env, status, U_ZERO_ERROR);
}

#[allow(non_snake_case)]
fn uregex_reset(
    env: &mut Environment,
    regex: URegularExpression,
    index: i32,
    status: MutPtr<UErrorCode>,
) {
    if status_is_failure(env, status) {
        return;
    }
    let Some(state) = State::get_mut(env).regexes.get_mut(&regex) else {
        set_status(env, status, U_ILLEGAL_ARGUMENT_ERROR);
        return;
    };
    if index < 0 || index > state.utf16_len() {
        set_status(env, status, U_INDEX_OUTOFBOUNDS_ERROR);
        return;
    }
    state.groups.clear();
    state.next_start = index;
    state.append_position = index;
    set_status(env, status, U_ZERO_ERROR);
}

#[allow(non_snake_case)]
fn uregex_find(
    env: &mut Environment,
    regex: URegularExpression,
    start_index: i32,
    status: MutPtr<UErrorCode>,
) -> UBool {
    if status_is_failure(env, status) {
        return false;
    }
    let Some(state) = State::get_mut(env).regexes.get_mut(&regex) else {
        set_status(env, status, U_ILLEGAL_ARGUMENT_ERROR);
        return false;
    };
    // A negative start index means "continue from the last match", which is what
    // uregex_findNext() does.
    let start = if start_index < 0 {
        state.next_start
    } else {
        start_index
    };
    if start > state.utf16_len() {
        set_status(env, status, U_INDEX_OUTOFBOUNDS_ERROR);
        return false;
    }
    if start_index >= 0 {
        state.append_position = start_index;
    }
    let found = state.find_from(start);
    set_status(env, status, U_ZERO_ERROR);
    found
}

#[allow(non_snake_case)]
fn uregex_findNext(
    env: &mut Environment,
    regex: URegularExpression,
    status: MutPtr<UErrorCode>,
) -> UBool {
    uregex_find(env, regex, -1, status)
}

#[allow(non_snake_case)]
fn uregex_groupCount(
    env: &mut Environment,
    regex: URegularExpression,
    status: MutPtr<UErrorCode>,
) -> i32 {
    if status_is_failure(env, status) {
        return 0;
    }
    let Some(state) = State::get_mut(env).regexes.get_mut(&regex) else {
        set_status(env, status, U_ILLEGAL_ARGUMENT_ERROR);
        return 0;
    };
    // ICU does not count group 0 (the whole match).
    let count = state.regex.captures_len() as i32 - 1;
    set_status(env, status, U_ZERO_ERROR);
    count
}

/// Shared by `uregex_start()` and `uregex_end()`; `end` picks which side of the
/// span to report.
fn group_bound(
    env: &mut Environment,
    regex: URegularExpression,
    group: i32,
    status: MutPtr<UErrorCode>,
    end: bool,
) -> i32 {
    if status_is_failure(env, status) {
        return -1;
    }
    let Some(state) = State::get_mut(env).regexes.get_mut(&regex) else {
        set_status(env, status, U_ILLEGAL_ARGUMENT_ERROR);
        return -1;
    };
    if group < 0 || group as usize >= state.groups.len() {
        // No match yet, or no such group.
        set_status(env, status, U_INDEX_OUTOFBOUNDS_ERROR);
        return -1;
    }
    set_status(env, status, U_ZERO_ERROR);
    let state = State::get_mut(env).regexes.get(&regex).unwrap();
    match state.groups[group as usize] {
        // A group that did not participate reports -1 for both bounds.
        None => -1,
        Some((start, stop)) => {
            if end {
                stop
            } else {
                start
            }
        }
    }
}

#[allow(non_snake_case)]
fn uregex_start(
    env: &mut Environment,
    regex: URegularExpression,
    group: i32,
    status: MutPtr<UErrorCode>,
) -> i32 {
    group_bound(env, regex, group, status, /* end: */ false)
}

#[allow(non_snake_case)]
fn uregex_end(
    env: &mut Environment,
    regex: URegularExpression,
    group: i32,
    status: MutPtr<UErrorCode>,
) -> i32 {
    group_bound(env, regex, group, status, /* end: */ true)
}

/// Expand `$0`..`$9` and `\` escapes in a replacement string against the last
/// match, the way ICU does.
fn expand_replacement(state: &RegexState, replacement: &str) -> String {
    let mut out = String::new();
    let mut chars = replacement.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(escaped) = chars.next() {
                    out.push(escaped);
                }
            }
            '$' => {
                let mut number = String::new();
                while let Some(&digit) = chars.peek() {
                    if !digit.is_ascii_digit() {
                        break;
                    }
                    // ICU takes the longest group number that actually exists.
                    let candidate: usize =
                        format!("{}{}", number, digit).parse().unwrap_or(usize::MAX);
                    if candidate >= state.groups.len() && !number.is_empty() {
                        break;
                    }
                    number.push(digit);
                    chars.next();
                }
                match number.parse::<usize>() {
                    Ok(group) => out.push_str(state.group_text(group).unwrap_or("")),
                    Err(_) => out.push('$'),
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// Write `text` into the `destBuf`/`destCapacity` pair that ICU's append
/// functions use, advancing both. Returns the number of UTF-16 units the text
/// needed, whether or not they all fit — that is what ICU returns, and callers
/// use it to size a second attempt.
fn append_to_dest(
    env: &mut Environment,
    text: &str,
    dest_buf: MutPtr<MutPtr<UChar>>,
    dest_capacity: MutPtr<i32>,
    status: MutPtr<UErrorCode>,
) -> i32 {
    let units: Vec<u16> = text.encode_utf16().collect();
    let needed = units.len() as i32;
    if dest_buf.is_null() || dest_capacity.is_null() {
        set_status(env, status, U_ILLEGAL_ARGUMENT_ERROR);
        return needed;
    }
    let mut buf = env.mem.read(dest_buf);
    let capacity = env.mem.read(dest_capacity);
    if needed > capacity {
        set_status(env, status, U_BUFFER_OVERFLOW_ERROR);
        return needed;
    }
    for (i, &unit) in units.iter().enumerate() {
        env.mem.write(buf + i as GuestUSize, unit);
    }
    buf += needed as GuestUSize;
    env.mem.write(dest_buf, buf);
    env.mem.write(dest_capacity, capacity - needed);
    needed
}

#[allow(non_snake_case)]
fn uregex_appendReplacement(
    env: &mut Environment,
    regex: URegularExpression,
    replacement_text: ConstPtr<UChar>,
    replacement_length: i32,
    dest_buf: MutPtr<MutPtr<UChar>>,
    dest_capacity: MutPtr<i32>,
    status: MutPtr<UErrorCode>,
) -> i32 {
    if status_is_failure(env, status) {
        return 0;
    }
    let replacement = read_utf16(env, replacement_text, replacement_length);
    let Some(state) = State::get_mut(env).regexes.get_mut(&regex) else {
        set_status(env, status, U_ILLEGAL_ARGUMENT_ERROR);
        return 0;
    };
    let Some(&Some((match_start, _))) = state.groups.first() else {
        set_status(env, status, U_ILLEGAL_ARGUMENT_ERROR);
        return 0;
    };
    // Everything between the last append position and this match, then the
    // expanded replacement.
    let prefix = match (state.to_utf8(state.append_position), state.to_utf8(match_start)) {
        (Some(from), Some(to)) => state.text.get(from..to).unwrap_or("").to_string(),
        _ => String::new(),
    };
    let expanded = expand_replacement(state, &replacement);
    state.append_position = state.next_start;
    append_to_dest(
        env,
        &format!("{}{}", prefix, expanded),
        dest_buf,
        dest_capacity,
        status,
    )
}

#[allow(non_snake_case)]
fn uregex_appendTail(
    env: &mut Environment,
    regex: URegularExpression,
    dest_buf: MutPtr<MutPtr<UChar>>,
    dest_capacity: MutPtr<i32>,
    status: MutPtr<UErrorCode>,
) -> i32 {
    if status_is_failure(env, status) {
        return 0;
    }
    let Some(state) = State::get_mut(env).regexes.get_mut(&regex) else {
        set_status(env, status, U_ILLEGAL_ARGUMENT_ERROR);
        return 0;
    };
    let tail = match state.to_utf8(state.append_position) {
        Some(from) => state.text.get(from..).unwrap_or("").to_string(),
        None => String::new(),
    };
    state.append_position = state.utf16_len();
    append_to_dest(env, &tail, dest_buf, dest_capacity, status)
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(u_strlen(_)),
    export_c_func!(uregex_open(_, _, _, _, _)),
    export_c_func!(uregex_close(_)),
    export_c_func!(uregex_setText(_, _, _, _)),
    export_c_func!(uregex_reset(_, _, _)),
    export_c_func!(uregex_find(_, _, _)),
    export_c_func!(uregex_findNext(_, _)),
    export_c_func!(uregex_groupCount(_, _)),
    export_c_func!(uregex_start(_, _, _)),
    export_c_func!(uregex_end(_, _, _)),
    export_c_func!(uregex_appendReplacement(_, _, _, _, _, _)),
    export_c_func!(uregex_appendTail(_, _, _, _)),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn state_for(pattern: &str, text: &str) -> RegexState {
        let mut state = RegexState {
            regex: Regex::new(pattern).unwrap(),
            text: String::new(),
            utf16_index: vec![0],
            utf8_offset: vec![0],
            groups: Vec::new(),
            next_start: 0,
            append_position: 0,
        };
        state.set_text(text.to_string());
        state
    }

    #[test]
    fn indices_are_utf16_not_utf8() {
        // "é" is two UTF-8 bytes but one UTF-16 unit; "😀" is four and two.
        let mut state = state_for("world", "é😀 world");
        assert_eq!(state.utf16_len(), 9);
        assert!(state.find_from(0));
        assert_eq!(state.groups[0], Some((4, 9)));
    }

    #[test]
    fn groups_that_did_not_participate_are_none() {
        let mut state = state_for("(a)|(b)", "b");
        assert!(state.find_from(0));
        assert_eq!(state.groups[1], None);
        assert_eq!(state.groups[2], Some((0, 1)));
        assert_eq!(state.group_text(2), Some("b"));
    }

    #[test]
    fn empty_matches_still_advance() {
        let mut state = state_for("x*", "ab");
        assert!(state.find_from(0));
        assert_eq!(state.groups[0], Some((0, 0)));
        assert_eq!(state.next_start, 1);
    }

    #[test]
    fn replacement_expands_groups_and_escapes() {
        let mut state = state_for("(\\w+)@(\\w+)", "user@host");
        assert!(state.find_from(0));
        assert_eq!(expand_replacement(&state, "$2/$1"), "host/user");
        // A group number that does not exist is not consumed as one.
        assert_eq!(expand_replacement(&state, "$1$9"), "user");
        assert_eq!(expand_replacement(&state, "\\$literal"), "$literal");
    }
}
