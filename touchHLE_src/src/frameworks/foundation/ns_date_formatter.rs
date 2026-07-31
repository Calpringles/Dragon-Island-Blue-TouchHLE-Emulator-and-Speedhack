/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSDateFormatter`.
//!
//! Resources:
//! - Apple's [Introduction to Data Formatting Programming Guide For Cocoa](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/DataFormatting/DataFormatting.html)
//! - [Unicode Technical Standard #35](https://unicode.org/reports/tr35/tr35-10.html#Date_Format_Patterns)

use crate::frameworks::core_foundation::time::CFAbsoluteTimeGetGregorianDate;
use crate::frameworks::foundation::{ns_string, NSInteger, NSTimeInterval};
use crate::objc::{autorelease, id, msg, nil, objc_classes, ClassExports, HostObject, NSZonePtr};

/// `NSDateFormatterStyle`: NoStyle=0, Short=1, Medium=2, Long=3, Full=4.
type NSDateFormatterStyle = NSInteger;

struct NSDateFormatterHostObject {
    date_format: Option<id>,
    date_style: NSDateFormatterStyle,
    time_style: NSDateFormatterStyle,
}
impl HostObject for NSDateFormatterHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSDateFormatter: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NSDateFormatterHostObject {
        date_format: None,
        date_style: 0,
        time_style: 0,
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (())setDateFormat:(id)format { // NSString *
    let date_format: id = msg![env; format copy];
    env.objc.borrow_mut::<NSDateFormatterHostObject>(this).date_format = Some(date_format);
}

- (())setDateStyle:(NSDateFormatterStyle)style {
    env.objc.borrow_mut::<NSDateFormatterHostObject>(this).date_style = style;
}
- (NSDateFormatterStyle)dateStyle {
    env.objc.borrow::<NSDateFormatterHostObject>(this).date_style
}
- (())setTimeStyle:(NSDateFormatterStyle)style {
    env.objc.borrow_mut::<NSDateFormatterHostObject>(this).time_style = style;
}
- (NSDateFormatterStyle)timeStyle {
    env.objc.borrow::<NSDateFormatterHostObject>(this).time_style
}

- (id)stringFromDate:(id)date {
    let &NSDateFormatterHostObject {
        date_format,
        date_style,
        time_style,
    } = env.objc.borrow(this);
    // An explicit format string takes precedence; otherwise synthesise one from
    // the date/time styles, using only tokens the substitution below handles.
    let mut format = if let Some(date_format) = date_format {
        ns_string::to_rust_string(env, date_format).to_string()
    } else {
        let date_part = if date_style != 0 { "yyyy-MM-dd" } else { "" };
        let time_part = match time_style {
            0 => "",
            1 | 2 => "HH:mm",
            _ => "HH:mm:ss",
        };
        match (date_part.is_empty(), time_part.is_empty()) {
            (false, false) => format!("{date_part} {time_part}"),
            (false, true) => date_part.to_string(),
            (true, false) => time_part.to_string(),
            (true, true) => String::new(),
        }
    };
    log_dbg!("date_format before: {:?}", format);

    let ti: NSTimeInterval = msg![env; date timeIntervalSinceReferenceDate];
    let greg_date = CFAbsoluteTimeGetGregorianDate(env, ti, nil);
    let year = greg_date.year;
    let month = greg_date.month;
    let day = greg_date.day;
    let hour = greg_date.hours;
    let minute = greg_date.minutes;
    let second = greg_date.seconds;

    format = format.replace("yyyy", format!("{year:04}").as_str());
    format = format.replace("YYYY", format!("{year:04}").as_str());
    format = format.replace("MM", format!("{month:02}").as_str());
    format = format.replace("dd", format!("{day:02}").as_str());
    format = format.replace("HH", format!("{hour:02}").as_str());
    format = format.replace("mm", format!("{minute:02}").as_str());
    format = format.replace("ss", format!("{second:02}").as_str());

    for c in format.chars() {
        if let pattern @ ('A'..='Z' | 'a'..='z') = c {
            unimplemented!("date string contains unsubstituted format pattern: {pattern}");
        }
    }
    log_dbg!("date_format after: {:?}", format);

    let res = ns_string::from_rust_string(env, format);
    autorelease(env, res)
}

@end

};
