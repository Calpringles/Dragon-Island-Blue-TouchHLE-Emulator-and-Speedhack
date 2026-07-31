/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Isolated automation harness: framebuffer screenshots + scripted touch input.
//!
//! This module is entirely opt-in via environment variables and has exactly two
//! call sites (`capture_if_due` in the compositor, `due_taps` in
//! `Window::poll_for_events`). To remove it: delete this file, its `mod harness;`
//! line in lib.rs, and those two call sites. It never touches the guest app.
//!
//! Env vars:
//!   TOUCHHLE_CAPTURE_DIR    directory to write frame_NNNN_<ms>ms.ppm into
//!   TOUCHHLE_CAPTURE_EVERY  capture 1 of every N composited frames (default 30)
//!   TOUCHHLE_INPUT_SCRIPT   path to a tap script; lines:
//!                             `TAP <t_ms> <x> <y>` where x,y are WINDOW pixel
//!                             coords (as seen in a screenshot). Each TAP
//!                             becomes a down then an up 120ms later.
//!                             `DUMP <t_ms>` dumps the cocos2d scene graph
//!                             (class names + window-space positions) to the log,
//!                             which is how to see what is on screen and what is
//!                             tappable without looking at a screenshot.
//!                             `TAPTEXT <t_ms> <text>` taps the on-screen label
//!                             whose text contains <text>, so a script survives
//!                             timing drift and does not need coordinates. If the
//!                             label is not on screen yet it keeps looking for a
//!                             few seconds, and later TAPTEXTs wait their turn.

use crate::frameworks::core_graphics::{CGPoint, CGRect, CGSize};
use crate::objc::{id, msg, msg_class, nil, ObjC};
use crate::Environment;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

/// One injected touch phase, in window pixel coordinates.
pub struct DueTap {
    pub down: bool,
    pub x: f32,
    pub y: f32,
}

struct ScriptedTap {
    t_ms: u64,
    down: bool,
    x: f32,
    y: f32,
}

struct HarnessState {
    start: Instant,
    capture_dir: Option<String>,
    capture_every: u64,
    frame_counter: u64,
    capture_index: u64,
    taps: Vec<ScriptedTap>,
    next_tap: usize,
    dumps: Vec<u64>,
    next_dump: usize,
    tap_texts: Vec<(u64, String)>,
    next_tap_text: usize,
    /// Don't resolve another `TAPTEXT` before this time, so a queue that has
    /// bunched up (because one step waited for its label) still plays back as
    /// distinct taps the game can react to.
    tap_text_hold_until: u64,
    /// Taps produced at runtime by `TAPTEXT`, once the label has been located.
    injected: Vec<ScriptedTap>,
}

type Script = (Vec<ScriptedTap>, Vec<u64>, Vec<(u64, String)>);

fn parse_script(path: &str) -> Script {
    let mut taps = Vec::new();
    let mut dumps = Vec::new();
    let mut tap_texts = Vec::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        echo!("harness: could not read input script {:?}", path);
        return (taps, dumps, tap_texts);
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 4 && parts[0].eq_ignore_ascii_case("TAP") {
            if let (Ok(t), Ok(x), Ok(y)) = (
                parts[1].parse::<u64>(),
                parts[2].parse::<f32>(),
                parts[3].parse::<f32>(),
            ) {
                taps.push(ScriptedTap { t_ms: t, down: true, x, y });
                taps.push(ScriptedTap { t_ms: t + 120, down: false, x, y });
                continue;
            }
        }
        if parts.len() == 2 && parts[0].eq_ignore_ascii_case("DUMP") {
            if let Ok(t) = parts[1].parse::<u64>() {
                dumps.push(t);
                continue;
            }
        }
        if parts.len() >= 3 && parts[0].eq_ignore_ascii_case("TAPTEXT") {
            if let Ok(t) = parts[1].parse::<u64>() {
                tap_texts.push((t, parts[2..].join(" ")));
                continue;
            }
        }
        echo!("harness: ignoring malformed script line: {:?}", line);
    }
    taps.sort_by_key(|t| t.t_ms);
    dumps.sort_unstable();
    tap_texts.sort_by_key(|&(t, _)| t);
    (taps, dumps, tap_texts)
}

fn state() -> &'static Mutex<HarnessState> {
    static STATE: OnceLock<Mutex<HarnessState>> = OnceLock::new();
    STATE.get_or_init(|| {
        let capture_dir = std::env::var("TOUCHHLE_CAPTURE_DIR").ok().filter(|s| !s.is_empty());
        let capture_every = std::env::var("TOUCHHLE_CAPTURE_EVERY")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .filter(|&n| n > 0)
            .unwrap_or(30);
        let (taps, dumps, tap_texts) = std::env::var("TOUCHHLE_INPUT_SCRIPT")
            .ok()
            .filter(|s| !s.is_empty())
            .map(|p| parse_script(&p))
            .unwrap_or_default();
        if let Some(dir) = &capture_dir {
            let _ = std::fs::create_dir_all(dir);
            echo!("harness: capturing every {} frames to {:?}", capture_every, dir);
        }
        if !taps.is_empty() {
            echo!("harness: loaded {} scripted tap phases", taps.len());
        }
        Mutex::new(HarnessState {
            start: Instant::now(),
            capture_dir,
            capture_every,
            frame_counter: 0,
            capture_index: 0,
            taps,
            next_tap: 0,
            dumps,
            next_dump: 0,
            tap_texts,
            next_tap_text: 0,
            tap_text_hold_until: 0,
            injected: Vec::new(),
        })
    })
}

/// Called once per composited frame with framebuffer 0 bound. Dumps a PPM
/// screenshot every Nth frame when TOUCHHLE_CAPTURE_DIR is set.
pub fn capture_if_due(
    gles: &mut dyn crate::gles::GLES,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    let mut s = state().lock().unwrap();
    if s.capture_dir.is_none() {
        return;
    }
    s.frame_counter += 1;
    if s.frame_counter % s.capture_every != 0 {
        return;
    }
    let idx = s.capture_index;
    s.capture_index += 1;
    let ms = s.start.elapsed().as_millis();
    let dir = s.capture_dir.clone().unwrap();
    let path = format!("{dir}/frame_{idx:04}_{ms}ms.ppm");
    drop(s); // release lock before the (slow) glReadPixels
    crate::debug::dump_framebuffer(&path, x, y, width, height, gles);
}

/// Selector trace filter, from TOUCHHLE_TRACE_SEL (comma-separated substrings).
/// Empty (the default) disables tracing entirely.
fn trace_filters() -> &'static Vec<String> {
    static FILTERS: OnceLock<Vec<String>> = OnceLock::new();
    FILTERS.get_or_init(|| {
        let list: Vec<String> = std::env::var("TOUCHHLE_TRACE_SEL")
            .ok()
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        if !list.is_empty() {
            echo!("harness: tracing selectors containing {:?}", list);
        }
        list
    })
}

/// Guest LR ranges to trace, from TOUCHHLE_TRACE_LR: comma-separated
/// `start-end` pairs of hex addresses, e.g. "0xcf978-0xcfe60". This traces every
/// message sent *by* a given game method, which is how you find out what a
/// decompiled method actually asks the runtime for.
fn trace_lr_ranges() -> &'static Vec<(u32, u32)> {
    static RANGES: OnceLock<Vec<(u32, u32)>> = OnceLock::new();
    RANGES.get_or_init(|| {
        let parse = |s: &str| u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok();
        let list: Vec<(u32, u32)> = std::env::var("TOUCHHLE_TRACE_LR")
            .ok()
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(',')
                    .filter_map(|p| {
                        let (a, b) = p.split_once('-')?;
                        Some((parse(a)?, parse(b)?))
                    })
                    .collect()
            })
            .unwrap_or_default();
        if !list.is_empty() {
            echo!("harness: tracing sends from LR ranges {:x?}", list);
        }
        list
    })
}

/// Whether this send's return address falls in a traced range.
pub fn trace_wants_lr(lr: u32) -> bool {
    // The LR of a BLX has the Thumb bit set; mask it before comparing.
    let lr = lr & !1;
    trace_lr_ranges().iter().any(|&(a, b)| lr >= a && lr <= b)
}

/// Whether any tracing at all is active (cheap gate for the dispatch hook).
pub fn trace_any_enabled() -> bool {
    !trace_filters().is_empty() || !trace_lr_ranges().is_empty()
}

/// Whether to log touch coordinate conversions (TOUCHHLE_TRACE_TOUCH=1).
pub fn touch_trace_enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var("TOUCHHLE_TRACE_TOUCH").is_ok_and(|v| !v.is_empty()))
}

/// Log a touch coordinate at some stage of the window -> view conversion, so the
/// guest's view of a tap can be compared against the pixel that was injected.
pub fn trace_touch(stage: &str, x: f32, y: f32) {
    if !touch_trace_enabled() {
        return;
    }
    let ms = state().lock().unwrap().start.elapsed().as_millis();
    echo!("HTOUCH {}ms {} ({}, {})", ms, stage, x, y);
}

/// Whether any selector tracing is active (cheap gate for the dispatch hook).
pub fn trace_enabled() -> bool {
    !trace_filters().is_empty()
}

/// Whether this selector matches the trace filter.
pub fn trace_wants(selector: &str) -> bool {
    trace_filters().iter().any(|f| selector.contains(f.as_str()))
}

/// Log one traced message send, with the guest return address so the caller can
/// be mapped back to a game method via tools/ghidra/class_methods.json.
pub fn trace_msg(
    class_name: &str,
    selector: &str,
    receiver: id,
    lr: u32,
    args: [u32; 2],
    arg0_class: Option<&str>,
) {
    let ms = state().lock().unwrap().start.elapsed().as_millis();
    // Under the armv7 softfp ABI a CGFloat/CGPoint argument arrives in the core
    // registers, so show each word both ways and let the reader pick.
    echo!(
        "HTRACE {}ms [{}@0x{:08x} {}] lr=0x{:08x} a0=0x{:08x}({}{}) a1=0x{:08x}({})",
        ms,
        class_name,
        receiver.to_bits(),
        selector,
        lr,
        args[0],
        f32::from_bits(args[0]),
        arg0_class.map_or(String::new(), |c| format!(", {}", c)),
        args[1],
        f32::from_bits(args[1]),
    );
}

/// Whether to log guest file opens (TOUCHHLE_TRACE_FILES=1).
fn file_trace_enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var("TOUCHHLE_TRACE_FILES").is_ok_and(|v| !v.is_empty()))
}

/// Log one guest file open. This is the only way to see which of an app's asset
/// variants it actually loads.
pub fn trace_file_open(path: &str) {
    if !file_trace_enabled() {
        return;
    }
    let ms = state().lock().unwrap().start.elapsed().as_millis();
    echo!("HFILE {}ms {}", ms, path);
}

/// Whether a `DUMP` line's time has arrived (consumes it).
fn dump_due() -> bool {
    let mut s = state().lock().unwrap();
    if s.next_dump >= s.dumps.len() {
        return false;
    }
    let elapsed = s.start.elapsed().as_millis() as u64;
    if s.dumps[s.next_dump] > elapsed {
        return false;
    }
    // Skip any that are also overdue, so one late iteration doesn't dump twice.
    while s.next_dump < s.dumps.len() && s.dumps[s.next_dump] <= elapsed {
        s.next_dump += 1;
    }
    true
}

/// Walk the running cocos2d scene and log every node with its class, its origin
/// in *window* pixels, and its size. This is the eyes of the automation loop: it
/// says what screen the game is on and where the tappable things are, without
/// anyone having to look at a screenshot.
///
/// Called once per run-loop iteration; a no-op unless a `DUMP` is due.
pub fn dump_scene_if_due(env: &mut Environment) {
    resolve_due_tap_texts(env);
    if !dump_due() {
        return;
    }
    // The harness must never be able to take down the emulator. There is no
    // cocos2d in, for example, the app picker's environment.
    if !env.objc.class_is_known("CCDirector") {
        echo!("HSCENE: no CCDirector in this environment");
        return;
    }
    let director: id = msg_class![env; CCDirector sharedDirector];
    if director == nil {
        echo!("HSCENE: no CCDirector");
        return;
    }
    let scene: id = msg![env; director runningScene];
    if scene == nil {
        echo!("HSCENE: no running scene");
        return;
    }
    let ms = state().lock().unwrap().start.elapsed().as_millis();
    echo!("HSCENE {}ms ==== running scene ====", ms);
    dump_node(env, scene, 0);
    // The director's notification node is visited after the running scene but is
    // not part of it. Dragon Island Blue puts every popup and menu there, so
    // omitting it would hide most of the interactive UI.
    let notification_node: id = msg![env; director notificationNode];
    if notification_node != nil {
        echo!("HSCENE {}ms ==== notification node ====", ms);
        dump_node(env, notification_node, 0);
    }
    echo!("HSCENE {}ms ==== end ====", ms);
}

/// The window is 480x320 with a top-left origin; cocos2d's world space is
/// bottom-left. Flip so the logged coordinates can be pasted straight into a
/// `TAP` line.
fn to_window(p: CGPoint) -> CGPoint {
    CGPoint {
        x: p.x,
        y: 320.0 - p.y,
    }
}

fn dump_node(env: &mut Environment, node: id, depth: usize) {
    if depth > 12 {
        return;
    }
    let class = ObjC::read_isa(node, &env.mem);
    let class_name = env.objc.get_class_name(class).to_string();

    let size: CGSize = msg![env; node contentSize];
    let origin: CGPoint = msg![env; node convertToWorldSpace:(CGPoint { x: 0.0, y: 0.0 })];
    let far: CGPoint = msg![env; node convertToWorldSpace:(CGPoint { x: size.width, y: size.height })];
    let visible: bool = msg![env; node visible];
    // CGPoint/CGSize are `repr(packed)`, so copy the fields out before use.
    let (ox, oy) = (origin.x, origin.y);
    let (fx, fy) = (far.x, far.y);
    let centre = to_window(CGPoint {
        x: (ox + fx) / 2.0,
        y: (oy + fy) / 2.0,
    });
    let (cx, cy) = (centre.x, centre.y);

    // A label's text is the single most useful thing for identifying a screen.
    let text = describe_text(env, node);

    echo!(
        "HSCENE {:indent$}{} tap=({:.0},{:.0}) size=({:.0}x{:.0}){}{}",
        "",
        class_name,
        cx,
        cy,
        (fx - ox).abs(),
        (fy - oy).abs(),
        if visible { "" } else { " HIDDEN" },
        text,
        indent = depth * 2,
    );

    let children: id = msg![env; node children];
    if children == nil {
        return;
    }
    let count: u32 = msg![env; children count];
    for i in 0..count {
        let child: id = msg![env; children objectAtIndex:i];
        if child != nil {
            dump_node(env, child, depth + 1);
        }
    }
}

/// Turn any `TAPTEXT` whose time has arrived into a real tap, by finding the
/// label on screen. The notification node is searched first: when a popup is up
/// it is what the player can actually touch, and its labels often duplicate ones
/// still present in the scene behind it.
fn resolve_due_tap_texts(env: &mut Environment) {
    loop {
        let wanted = {
            let s = state().lock().unwrap();
            if s.next_tap_text >= s.tap_texts.len() {
                return;
            }
            let elapsed = s.start.elapsed().as_millis() as u64;
            let (t_ms, ref text) = s.tap_texts[s.next_tap_text];
            if t_ms > elapsed || s.tap_text_hold_until > elapsed {
                return;
            }
            (elapsed, t_ms, text.clone())
        };
        let (now, t_ms, text) = wanted;

        let director: id = msg_class![env; CCDirector sharedDirector];
        let notification_node: id = if director == nil {
            nil
        } else {
            msg![env; director notificationNode]
        };
        let scene: id = if director == nil {
            nil
        } else {
            msg![env; director runningScene]
        };

        let mut found = None;
        for root in [notification_node, scene] {
            if root != nil {
                found = find_label(env, root, &text);
                if found.is_some() {
                    break;
                }
            }
        }

        let Some(point) = found else {
            // The label may not exist yet (a scene transition, a fade-in, an
            // animation still playing). Keep looking until the deadline rather
            // than failing the step and desynchronising the whole script.
            if now < t_ms + TAP_TEXT_TIMEOUT_MS {
                return;
            }
            echo!("HTAPTEXT {}ms NOT FOUND {:?} (gave up)", now, text);
            state().lock().unwrap().next_tap_text += 1;
            continue;
        };
        let (x, y) = (point.x, point.y);
        echo!("HTAPTEXT {}ms {:?} -> ({:.0}, {:.0})", now, text, x, y);
        let mut s = state().lock().unwrap();
        s.next_tap_text += 1;
        s.tap_text_hold_until = now + 400;
        s.injected.push(ScriptedTap { t_ms: now, down: true, x, y });
        s.injected.push(ScriptedTap { t_ms: now + 120, down: false, x, y });
        s.injected.sort_by_key(|t| t.t_ms);
        return;
    }
}

/// How long a `TAPTEXT` keeps looking for its label before giving up.
const TAP_TEXT_TIMEOUT_MS: u64 = 6000;

/// Depth-first search for a visible label containing `text`, returning where to
/// tap it in window pixels.
fn find_label(env: &mut Environment, node: id, text: &str) -> Option<CGPoint> {
    let visible: bool = msg![env; node visible];
    if !visible {
        return None;
    }
    if let Some(node_text) = label_text(env, node) {
        if node_text.contains(text) {
            let size: CGSize = msg![env; node contentSize];
            let (w, h) = (size.width, size.height);
            let centre: CGPoint = msg![env; node convertToWorldSpace:(CGPoint { x: w / 2.0, y: h / 2.0 })];
            return Some(to_window(centre));
        }
    }
    let children: id = msg![env; node children];
    if children == nil {
        return None;
    }
    let count: u32 = msg![env; children count];
    for i in 0..count {
        let child: id = msg![env; children objectAtIndex:i];
        if child != nil {
            if let Some(found) = find_label(env, child, text) {
                return Some(found);
            }
        }
    }
    None
}

/// The node's label text, if it has one.
fn label_text(env: &mut Environment, node: id) -> Option<String> {
    let sel = env.objc.lookup_selector("string")?;
    let responds: bool = msg![env; node respondsToSelector:sel];
    if !responds {
        return None;
    }
    let string: id = msg![env; node string];
    if string == nil {
        return None;
    }
    Some(crate::frameworks::foundation::ns_string::to_rust_string(env, string).to_string())
}

/// The node's label text, if it has one, as a ` "..."` suffix.
fn describe_text(env: &mut Environment, node: id) -> String {
    label_text(env, node).map_or(String::new(), |s| format!(" {:?}", s))
}

/// Log a message sent to nil while a trace is running. Objective-C defines this
/// to evaluate to 0 silently, which is how a failed dictionary lookup becomes a
/// convincing-looking `0` several frames later rather than an error at the
/// source. If a traced method is asking nil for something, that is the bug.
pub fn trace_nil_msg(selector: &str, lr: u32) {
    let ms = state().lock().unwrap().start.elapsed().as_millis();
    echo!("HTRACE {}ms [nil {}] -> 0  lr=0x{:08x}", ms, selector, lr);
}

/// Returns any scripted taps whose scheduled time has arrived. Coordinates are
/// in window pixels; the caller transforms them into guest coords.
pub fn due_taps() -> Vec<DueTap> {
    let mut s = state().lock().unwrap();
    if s.taps.is_empty() && s.injected.is_empty() {
        return Vec::new();
    }
    let elapsed = s.start.elapsed().as_millis() as u64;
    let mut out = Vec::new();
    while s.next_tap < s.taps.len() && s.taps[s.next_tap].t_ms <= elapsed {
        let t = &s.taps[s.next_tap];
        out.push(DueTap { down: t.down, x: t.x, y: t.y });
        s.next_tap += 1;
    }
    // TAPTEXT taps are resolved during the run, so this queue is drained rather
    // than indexed.
    let mut i = 0;
    while i < s.injected.len() {
        if s.injected[i].t_ms <= elapsed {
            let t = s.injected.remove(i);
            out.push(DueTap { down: t.down, x: t.x, y: t.y });
        } else {
            i += 1;
        }
    }
    out
}
