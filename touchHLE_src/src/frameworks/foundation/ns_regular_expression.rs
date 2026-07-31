/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSRegularExpression` and `NSTextCheckingResult`.
//!
//! Backed by the same `regex_lite` engine as the ICU shim in
//! [crate::libc::icu]. As there, every index Foundation reports is a UTF-16
//! offset while the engine works in UTF-8, so [Utf16Map] does the conversion in
//! one place.

use super::ns_string::{from_rust_string, to_rust_string};
use super::{ns_array, NSRange, NSUInteger};
use crate::objc::{
    autorelease, id, msg, msg_class, nil, objc_classes, ClassExports, HostObject, NSZonePtr,
};
use crate::mem::MutPtr;
use crate::Environment;
use regex_lite::Regex;

pub type NSRegularExpressionOptions = NSUInteger;
const NSRegularExpressionCaseInsensitive: NSUInteger = 1 << 0;
const NSRegularExpressionAllowCommentsAndWhitespace: NSUInteger = 1 << 1;
const NSRegularExpressionIgnoreMetacharacters: NSUInteger = 1 << 2;
const NSRegularExpressionDotMatchesLineSeparators: NSUInteger = 1 << 3;
const NSRegularExpressionAnchorsMatchLines: NSUInteger = 1 << 4;

/// Mapping between UTF-8 byte offsets and UTF-16 offsets for one subject string.
struct Utf16Map {
    utf16_index: Vec<NSUInteger>,
    utf8_offset: Vec<usize>,
}

impl Utf16Map {
    fn new(text: &str) -> Self {
        let mut utf16_index = Vec::with_capacity(text.len() + 1);
        let mut utf8_offset = Vec::new();
        let mut u16_pos: NSUInteger = 0;
        for (offset, c) in text.char_indices() {
            for _ in 0..c.len_utf8() {
                utf16_index.push(u16_pos);
            }
            for _ in 0..c.len_utf16() {
                utf8_offset.push(offset);
            }
            u16_pos += c.len_utf16() as NSUInteger;
        }
        utf16_index.push(u16_pos);
        utf8_offset.push(text.len());
        Utf16Map {
            utf16_index,
            utf8_offset,
        }
    }

    fn utf16_len(&self) -> NSUInteger {
        *self.utf16_index.last().unwrap_or(&0)
    }

    fn to_utf16(&self, byte_offset: usize) -> NSUInteger {
        self.utf16_index
            .get(byte_offset)
            .copied()
            .unwrap_or_else(|| self.utf16_len())
    }

    fn to_utf8(&self, utf16_index: NSUInteger) -> Option<usize> {
        self.utf8_offset.get(utf16_index as usize).copied()
    }
}

struct NSRegularExpressionHostObject {
    regex: Option<Regex>,
    pattern: String,
}
impl HostObject for NSRegularExpressionHostObject {}

#[derive(Default)]
struct NSTextCheckingResultHostObject {
    /// Capture spans in UTF-16 offsets. Group 0 is the whole match; a group that
    /// did not participate is [None].
    groups: Vec<Option<NSRange>>,
}
impl HostObject for NSTextCheckingResultHostObject {}

/// `NSNotFound`, as Foundation reports it for a group that did not participate.
const NOT_FOUND_RANGE: NSRange = NSRange {
    location: super::NSNotFound as NSUInteger,
    length: 0,
};

/// Translate Foundation's options into inline flags for the engine.
fn pattern_with_options(pattern: &str, options: NSRegularExpressionOptions) -> String {
    let pattern = if options & NSRegularExpressionIgnoreMetacharacters != 0 {
        regex_lite::escape(pattern)
    } else {
        pattern.to_string()
    };
    let mut inline = String::new();
    if options & NSRegularExpressionCaseInsensitive != 0 {
        inline.push('i');
    }
    if options & NSRegularExpressionAllowCommentsAndWhitespace != 0 {
        inline.push('x');
    }
    if options & NSRegularExpressionDotMatchesLineSeparators != 0 {
        inline.push('s');
    }
    if options & NSRegularExpressionAnchorsMatchLines != 0 {
        inline.push('m');
    }
    if inline.is_empty() {
        pattern
    } else {
        format!("(?{}){}", inline, pattern)
    }
}

/// Run `regex` over the part of `text` selected by `range`, returning each
/// match's capture spans in UTF-16 offsets.
fn find_matches(regex: &Regex, text: &str, map: &Utf16Map, range: NSRange) -> Vec<Vec<Option<NSRange>>> {
    let start = map.to_utf8(range.location).unwrap_or(text.len());
    let end = map
        .to_utf8(range.location + range.length)
        .unwrap_or(text.len());
    if start > end {
        return Vec::new();
    }
    // Search a slice so that `range` really limits the match, then shift the
    // reported offsets back into the whole string's coordinates.
    let slice = &text[start..end];
    let mut out = Vec::new();
    for captures in regex.captures_iter(slice) {
        let groups = (0..captures.len())
            .map(|i| {
                captures.get(i).map(|m| {
                    let location = map.to_utf16(start + m.start());
                    NSRange {
                        location,
                        length: map.to_utf16(start + m.end()) - location,
                    }
                })
            })
            .collect();
        out.push(groups);
    }
    out
}

fn new_text_checking_result(env: &mut Environment, groups: Vec<Option<NSRange>>) -> id {
    let class = env
        .objc
        .get_known_class("NSTextCheckingResult", &mut env.mem);
    let host_object = Box::new(NSTextCheckingResultHostObject { groups });
    let result = env.objc.alloc_object(class, host_object, &mut env.mem);
    autorelease(env, result)
}

/// Borrow the compiled regex and the subject string together, since every
/// matching method needs both.
fn regex_and_text(env: &mut Environment, this: id, string: id) -> Option<(Regex, String)> {
    let regex = env
        .objc
        .borrow::<NSRegularExpressionHostObject>(this)
        .regex
        .clone()?;
    let text = to_rust_string(env, string).to_string();
    Some((regex, text))
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSRegularExpression: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NSRegularExpressionHostObject {
        regex: None,
        pattern: String::new(),
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)regularExpressionWithPattern:(id)pattern // NSString *
                           options:(NSRegularExpressionOptions)options
                             error:(MutPtr<id>)error {
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithPattern:pattern options:options error:error];
    autorelease(env, new)
}

+ (id)escapedPatternForString:(id)string { // NSString *
    let string = to_rust_string(env, string).to_string();
    let escaped = regex_lite::escape(&string);
    let res = from_rust_string(env, escaped);
    autorelease(env, res)
}

- (id)initWithPattern:(id)pattern // NSString *
              options:(NSRegularExpressionOptions)options
                error:(MutPtr<id>)error {
    let pattern_string = to_rust_string(env, pattern).to_string();
    let translated = pattern_with_options(&pattern_string, options);
    match Regex::new(&translated) {
        Ok(regex) => {
            let host_object: &mut NSRegularExpressionHostObject = env.objc.borrow_mut(this);
            host_object.regex = Some(regex);
            host_object.pattern = pattern_string;
            if !error.is_null() {
                env.mem.write(error, nil);
            }
            this
        }
        Err(e) => {
            // Foundation returns nil and writes an NSError; the pattern being
            // rejected by a different engine than ICU is worth saying out loud.
            log!(
                "NSRegularExpression: could not compile pattern {:?}: {}",
                pattern_string,
                e
            );
            if !error.is_null() {
                let domain = from_rust_string(env, "NSCocoaErrorDomain".to_string());
                let userinfo: id = msg_class![env; NSDictionary dictionary];
                let err: id = msg_class![env; NSError alloc];
                let err: id = msg![env; err initWithDomain:domain code:2048i32 userInfo:userinfo];
                let err = autorelease(env, err);
                env.mem.write(error, err);
            }
            nil
        }
    }
}

- (id)pattern {
    let pattern = env.objc.borrow::<NSRegularExpressionHostObject>(this).pattern.clone();
    let res = from_rust_string(env, pattern);
    autorelease(env, res)
}

- (NSUInteger)numberOfCaptureGroups {
    let host_object = env.objc.borrow::<NSRegularExpressionHostObject>(this);
    host_object.regex.as_ref().map_or(0, |r| r.captures_len() as NSUInteger - 1)
}

- (NSUInteger)numberOfMatchesInString:(id)string // NSString *
                              options:(NSUInteger)_options
                                range:(NSRange)range {
    let Some((regex, text)) = regex_and_text(env, this, string) else {
        return 0;
    };
    let map = Utf16Map::new(&text);
    find_matches(&regex, &text, &map, range).len() as NSUInteger
}

- (id)firstMatchInString:(id)string // NSString *
                 options:(NSUInteger)_options
                   range:(NSRange)range {
    let Some((regex, text)) = regex_and_text(env, this, string) else {
        return nil;
    };
    let map = Utf16Map::new(&text);
    let Some(groups) = find_matches(&regex, &text, &map, range).into_iter().next() else {
        return nil;
    };
    new_text_checking_result(env, groups)
}

- (NSRange)rangeOfFirstMatchInString:(id)string // NSString *
                             options:(NSUInteger)_options
                               range:(NSRange)range {
    let Some((regex, text)) = regex_and_text(env, this, string) else {
        return NOT_FOUND_RANGE;
    };
    let map = Utf16Map::new(&text);
    find_matches(&regex, &text, &map, range)
        .first()
        .and_then(|groups| groups.first().copied().flatten())
        .unwrap_or(NOT_FOUND_RANGE)
}

- (id)matchesInString:(id)string // NSString *
              options:(NSUInteger)_options
                range:(NSRange)range {
    let Some((regex, text)) = regex_and_text(env, this, string) else {
        let empty: id = msg_class![env; NSArray array];
        return empty;
    };
    let map = Utf16Map::new(&text);
    let matches = find_matches(&regex, &text, &map, range);
    let results: Vec<id> = matches
        .into_iter()
        .map(|groups| {
            let result = new_text_checking_result(env, groups);
            // from_vec takes ownership of a reference each.
            msg![env; result retain]
        })
        .collect();
    let array = ns_array::from_vec(env, results);
    autorelease(env, array)
}

- (id)stringByReplacingMatchesInString:(id)string // NSString *
                               options:(NSUInteger)_options
                                 range:(NSRange)range
                          withTemplate:(id)template_ { // NSString *
    let Some((regex, text)) = regex_and_text(env, this, string) else {
        return string;
    };
    let template_string = to_rust_string(env, template_).to_string();
    let map = Utf16Map::new(&text);
    let matches = find_matches(&regex, &text, &map, range);

    // Rebuild the string by hand rather than using the engine's own replace, so
    // that a range narrower than the whole string is honoured.
    let mut out = String::new();
    let mut copied_to: usize = 0;
    for groups in &matches {
        let Some(whole) = groups[0] else { continue };
        let start = map.to_utf8(whole.location).unwrap_or(text.len());
        let end = map
            .to_utf8(whole.location + whole.length)
            .unwrap_or(text.len());
        out.push_str(&text[copied_to..start]);
        out.push_str(&expand_template(&template_string, groups, &text, &map));
        copied_to = end;
    }
    out.push_str(&text[copied_to..]);

    let res = from_rust_string(env, out);
    autorelease(env, res)
}

@end

@implementation NSTextCheckingResult: NSObject

- (NSRange)range {
    let host_object = env.objc.borrow::<NSTextCheckingResultHostObject>(this);
    host_object.groups.first().copied().flatten().unwrap_or(NOT_FOUND_RANGE)
}

- (NSUInteger)numberOfRanges {
    let host_object = env.objc.borrow::<NSTextCheckingResultHostObject>(this);
    host_object.groups.len() as NSUInteger
}

- (NSRange)rangeAtIndex:(NSUInteger)idx {
    let host_object = env.objc.borrow::<NSTextCheckingResultHostObject>(this);
    host_object
        .groups
        .get(idx as usize)
        .copied()
        .flatten()
        .unwrap_or(NOT_FOUND_RANGE)
}

@end

};

/// Expand `$1`-style references in a replacement template against one match.
fn expand_template(
    template: &str,
    groups: &[Option<NSRange>],
    text: &str,
    map: &Utf16Map,
) -> String {
    let group_text = |n: usize| -> String {
        let Some(Some(range)) = groups.get(n) else {
            return String::new();
        };
        let start = map.to_utf8(range.location).unwrap_or(text.len());
        let end = map
            .to_utf8(range.location + range.length)
            .unwrap_or(text.len());
        text[start..end].to_string()
    };

    let mut out = String::new();
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                // A backslash escapes the next character, including '$'.
                if let Some(next) = chars.next() {
                    out.push(next);
                }
            }
            '$' => {
                let mut digits = String::new();
                while let Some(d) = chars.peek().filter(|d| d.is_ascii_digit()) {
                    digits.push(*d);
                    chars.next();
                }
                if digits.is_empty() {
                    out.push('$');
                } else {
                    out.push_str(&group_text(digits.parse().unwrap()));
                }
            }
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_offsets_are_not_utf8_offsets() {
        // "é" is two bytes in UTF-8 but one UTF-16 code unit.
        let map = Utf16Map::new("aéb");
        assert_eq!(map.to_utf16(0), 0);
        assert_eq!(map.to_utf16(1), 1);
        assert_eq!(map.to_utf16(3), 2);
        assert_eq!(map.utf16_len(), 3);
        assert_eq!(map.to_utf8(2), Some(3));
    }

    #[test]
    fn range_limits_the_search() {
        let regex = Regex::new("a").unwrap();
        let text = "aaa".to_string();
        let map = Utf16Map::new(&text);
        let all = find_matches(&regex, &text, &map, NSRange { location: 0, length: 3 });
        assert_eq!(all.len(), 3);
        let limited = find_matches(&regex, &text, &map, NSRange { location: 1, length: 1 });
        assert_eq!(limited.len(), 1);
        let location = limited[0][0].unwrap().location;
        assert_eq!(location, 1);
    }

    #[test]
    fn template_expands_groups_and_escapes() {
        let regex = Regex::new("(a)(b)").unwrap();
        let text = "ab".to_string();
        let map = Utf16Map::new(&text);
        let m = find_matches(&regex, &text, &map, NSRange { location: 0, length: 2 });
        assert_eq!(expand_template("$2$1", &m[0], &text, &map), "ba");
        assert_eq!(expand_template("\\$1", &m[0], &text, &map), "$1");
        assert_eq!(expand_template("$9", &m[0], &text, &map), "");
    }
}
