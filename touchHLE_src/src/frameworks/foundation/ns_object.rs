/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSObject`, the root of most class hierarchies in Objective-C.
//!
//! Resources:
//! - Apple's [Advanced Memory Management Programming Guide](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/MemoryMgmt.html)
//!   explains how reference counting works. Note that we are interested in what
//!   it calls "manual retain-release", not ARC.
//! - Apple's [Key-Value Coding Programming Guide](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/KeyValueCoding/SearchImplementation.html)
//!   explains the algorithm `setValue:forKey:` should follow.
//!
//! See also: [crate::objc], especially the `objects` module.

use super::ns_string::{from_rust_string, to_rust_string};
use super::{NSTimeInterval, NSUInteger};
use crate::frameworks::foundation::ns_run_loop::{add_perform_request, cancel_perform_requests};
use crate::frameworks::foundation::ns_thread::detach_new_thread_inner;
use crate::libc::semaphore::{host_destroy_semaphore, sem_wait};
use crate::mem::{ConstVoidPtr, MutVoidPtr, Ptr};
use crate::Environment;
use crate::objc::{
    autorelease, id, msg, msg_class, msg_send, msg_send_no_type_checking, nil, objc_classes,
    retain, Class, ClassExports, NSZonePtr, ObjC, TrivialHostObject, SEL,
};


/// Split an Objective-C type encoding into its top-level type tokens, dropping
/// the offset digits and qualifier prefixes. For a method, the tokens are the
/// return type followed by each argument type, so a setter's value is token 3
/// (after the return type, `self` and `_cmd`).
fn type_tokens(encoding: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = encoding.chars().peekable();
    while let Some(&c) = chars.peek() {
        // Offsets between tokens, and the qualifiers that may prefix one.
        if c.is_ascii_digit() || "rnNoORV".contains(c) {
            chars.next();
            continue;
        }
        let mut token = String::new();
        let mut depth = 0usize;
        while let Some(c) = chars.next() {
            token.push(c);
            match c {
                '{' | '[' | '(' => depth += 1,
                '}' | ']' | ')' => depth -= 1,
                '"' => {
                    // A quoted class or ivar name, e.g. @"NSString".
                    for q in chars.by_ref() {
                        token.push(q);
                        if q == '"' {
                            break;
                        }
                    }
                }
                _ => {}
            }
            if depth == 0 && !matches!(chars.peek(), Some('"')) {
                break;
            }
        }
        if !token.is_empty() {
            tokens.push(token);
        }
    }
    tokens
}

/// The declared type of `key` on `class`, from its property attribute string
/// (which starts with `T` followed by the type encoding).
fn property_type(env: &mut Environment, class: Class, key: &str) -> Option<String> {
    let name = env.mem.alloc_and_write_cstr(key.as_bytes());
    let property = crate::objc::class_getProperty(env, class, name.cast_const());
    env.mem.free(name.cast_void());
    if property.is_null() {
        return None;
    }
    let attributes = crate::objc::property_getAttributes(env, property);
    let attributes = env.mem.cstr_at_utf8(attributes).ok()?;
    let first = attributes.split(',').next()?;
    first.strip_prefix('T').map(|t| t.to_string())
}

/// Send `sel` to `this` with `value`, unwrapping an NSNumber when the receiving
/// method takes a scalar. KVC does this because a scalar property can only be
/// set through the generic API by boxing the value first.
fn send_setter(env: &mut Environment, this: id, sel: SEL, value: id, arg_type: Option<String>) {
    let arg_type = arg_type.unwrap_or_else(|| "@".to_string());
    match arg_type.chars().next() {
        // An object-typed setter takes the box itself.
        Some('@') | Some('#') | None => () = msg_send(env, (this, sel, value)),
        Some('B') | Some('c') | Some('C') => {
            let v: bool = msg![env; value boolValue];
            () = msg_send(env, (this, sel, v))
        }
        Some('f') => {
            let v: f32 = msg![env; value floatValue];
            () = msg_send(env, (this, sel, v))
        }
        Some('d') => {
            let v: f64 = msg![env; value doubleValue];
            () = msg_send(env, (this, sel, v))
        }
        Some('q') => {
            let v: i64 = msg![env; value longLongValue];
            () = msg_send(env, (this, sel, v))
        }
        Some('Q') => {
            let v: u64 = msg![env; value unsignedLongLongValue];
            () = msg_send(env, (this, sel, v))
        }
        Some(_) => {
            // The remaining integer widths all pass in one register.
            let v: i32 = msg![env; value intValue];
            () = msg_send(env, (this, sel, v))
        }
    }
}

/// Write `value` into the ivar at `ivar_ptr`, unwrapping an NSNumber when the
/// ivar holds a scalar rather than an object reference.
fn write_ivar(env: &mut Environment, ivar_ptr: ConstVoidPtr, value: id, ivar_type: Option<String>) {
    let ivar_type = ivar_type.unwrap_or_else(|| "@".to_string());
    match ivar_type.chars().next() {
        Some('@') | Some('#') | None => {
            retain(env, value);
            env.mem.write(ivar_ptr.cast().cast_mut(), value);
        }
        Some('B') | Some('c') | Some('C') => {
            let v: bool = msg![env; value boolValue];
            env.mem.write(ivar_ptr.cast().cast_mut(), v as u8);
        }
        Some('f') => {
            let v: f32 = msg![env; value floatValue];
            env.mem.write(ivar_ptr.cast().cast_mut(), v);
        }
        Some('d') => {
            let v: f64 = msg![env; value doubleValue];
            env.mem.write(ivar_ptr.cast().cast_mut(), v);
        }
        Some('q') => {
            let v: i64 = msg![env; value longLongValue];
            env.mem.write(ivar_ptr.cast().cast_mut(), v);
        }
        Some('Q') => {
            let v: u64 = msg![env; value unsignedLongLongValue];
            env.mem.write(ivar_ptr.cast().cast_mut(), v);
        }
        Some('s') | Some('S') => {
            let v: i32 = msg![env; value intValue];
            env.mem.write(ivar_ptr.cast().cast_mut(), v as i16);
        }
        Some(_) => {
            let v: i32 = msg![env; value intValue];
            env.mem.write(ivar_ptr.cast().cast_mut(), v);
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSObject

+ (id)alloc {
    msg![env; this allocWithZone:(MutVoidPtr::null())]
}
+ (id)allocWithZone:(NSZonePtr)_zone { // struct _NSZone*
    log_dbg!("[{:?} allocWithZone:]", this);
    env.objc.alloc_object(this, Box::new(TrivialHostObject), &mut env.mem)
}

+ (id)new {
    let new_object: id = msg![env; this alloc];
    msg![env; new_object init]
}

+ (Class)class {
    this
}
+ (bool)isSubclassOfClass:(Class)class {
    env.objc.class_is_subclass_of(this, class)
}

// Class-method sibling of methodForSelector: (see instance methods below).
// Here `this` is the class itself, and we look up one of its instance methods.
+ (MutVoidPtr)instanceMethodForSelector:(SEL)selector {
    if let Some(imp) = env.objc.class_lookup_guest_imp(this, selector) {
        MutVoidPtr::from_bits(imp.addr_with_thumb_bit())
    } else {
        log!(
            "TODO: instanceMethodForSelector: returned NULL for {} (host-implemented or unknown selector)",
            selector.as_str(&env.mem)
        );
        MutVoidPtr::null()
    }
}

// See the instance method section for the normal versions of these.
+ (id)retain {
    this // classes are not refcounted
}
+ (())release {
    // classes are not refcounted
}
+ (())autorelease {
    // classes are not refcounted
}

+ (bool)instancesRespondToSelector:(SEL)selector {
    env.objc.class_has_method(this, selector)
}

+ (())cancelPreviousPerformRequestsWithTarget:(id)target selector:(SEL)selector object:(id)arg {
    let run_loop: id = msg_class![env; NSRunLoop currentRunLoop];
    cancel_perform_requests(env, run_loop, target, selector, arg);
}

+ (bool)accessInstanceVariablesDirectly {
    true
}

+ (id)description {
    let name = env.objc.get_class_name(this);
    let str = from_rust_string(env, name.to_string());
    autorelease(env, str)
}

+ (id)debugDescription {
    msg![env; this description]
}

+ (id)instanceMethodSignatureForSelector:(SEL)sel {
    // TODO: support `host` method signatures
    let sig = *env.objc.class_get_method_signature(this, sel).unwrap();
    log_dbg!("instanceMethodSignatureForSelector: '{}' -> {:?}", sel.as_str(&env.mem), env.mem.cstr_at_utf8(sig));
    msg_class![env; NSMethodSignature signatureWithObjCTypes:sig]
}

+ (())initialize {
    // Do nothing
}

- (id)init {
    this
}

- (NSUInteger)retainCount {
    env.objc.get_refcount(this).into()
}

- (id)retain {
    log_dbg!("[{:?} retain]", this);
    env.objc.increment_refcount(this);
    this
}
- (())release {
    log_dbg!("[{:?} release]", this);
    if env.objc.decrement_refcount(this) {
        () = msg![env; this dealloc];
    }
}
- (id)autorelease {
    () = msg_class![env; NSAutoreleasePool addObject:this];
    this
}

- (())dealloc {
    log_dbg!("[{:?} dealloc]", this);
    env.objc.dealloc_object(this, &mut env.mem)
}

- (Class)class {
    ObjC::read_isa(this, &env.mem)
}
- (bool)isMemberOfClass:(Class)class {
    let this_class: Class = msg![env; this class];
    class == this_class
}
- (bool)isKindOfClass:(Class)class {
    let this_class: Class = msg![env; this class];
    env.objc.class_is_subclass_of(this_class, class)
}

- (NSUInteger)hash {
    this.to_bits()
}

// To not confuse with isEqualTo:, which is
// a category of NSWhoseSpecifier!
// Reference https://nshipster.com/equality
- (bool)isEqual:(id)other {
    this == other
}

// TODO: Instance description and debugDescription.
// This is not hard to add, but before adding a fallback implementation of it,
// we should make sure all the Foundation classes' overrides of it are there,
// to prevent weird behavior.
// TODO: localized description methods also? (not sure if NSObject has them)

// Helper for NSCopying
- (id)copy {
    msg![env; this copyWithZone:(MutVoidPtr::null())]
}

// Helper for NSMutableCopying
- (id)mutableCopy {
    msg![env; this mutableCopyWithZone:(MutVoidPtr::null())]
}

// NSKeyValueCoding
// https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/KeyValueCoding/SearchImplementation.html
- (())setValue:(id)value
       forKey:(id)key { // NSString*
    let key_string = to_rust_string(env, key); // TODO: avoid copy?
    assert!(key_string.is_ascii()); // TODO: do we have to handle non-ASCII keys?
    let camel_case_key_string = format!("{}{}", key_string.as_bytes()[0].to_ascii_uppercase() as char, &key_string[1..]);

    let class = msg![env; this class];

    // TODO: If value is nil, the target ivar/method argument type must be
    // checked. If it's non-object type, invoke setNilValueForKey:
    assert!(value != nil);

    // A boxed value (NSNumber/NSValue) is how a scalar property gets set through
    // this API at all, so it is unwrapped according to the *target's* declared
    // type -- see send_setter and write_ivar.

    // Look for the first accessor named set<Key>: or _set<Key>, in that order.
    // If found, invoke it with the input value (or unwrapped value, as needed)
    // and finish.
    for prefix in ["set", "_set"] {
        let Some(sel) = env.objc.lookup_selector(&format!("{prefix}{camel_case_key_string}:"))
        else {
            continue;
        };
        if !env.objc.class_has_method(class, sel) {
            continue;
        }
        // The setter's own signature says whether it wants the box or a scalar.
        let arg_type = env
            .objc
            .class_get_method_signature(class, sel)
            .copied()
            .and_then(|types| env.mem.cstr_at_utf8(types).ok().map(|t| t.to_string()))
            .and_then(|types| type_tokens(&types).get(3).cloned());
        send_setter(env, this, sel, value, arg_type);
        return;
    }

    // If no simple accessor is found, and if the class method
    // accessInstanceVariablesDirectly returns YES, look for an instance
    // variable with a name like _<key>, _is<Key>, <key>, or is<Key>,
    // in that order.
    // If found, set the variable directly with the input value
    // (or unwrapped value) and finish.
    let sel = env.objc.lookup_selector("accessInstanceVariablesDirectly").unwrap();
    let accessInstanceVariablesDirectly = msg_send(env, (class, sel));
    if accessInstanceVariablesDirectly {
        if let Some(ivar_ptr) = env.objc.object_lookup_ivar(&env.mem, this, &format!("_{key_string}"))
            .or_else(|| env.objc.object_lookup_ivar(&env.mem, this, &format!("_is{camel_case_key_string}")))
            .or_else(|| env.objc.object_lookup_ivar(&env.mem, this, &format!("{key_string}")))
            .or_else(|| env.objc.object_lookup_ivar(&env.mem, this, &format!("is{camel_case_key_string}"))
        ) {
            let ivar_type = property_type(env, class, &key_string);
            write_ivar(env, ivar_ptr.cast_const().cast(), value, ivar_type);
            return;
        }
    }

    // Upon finding no accessor or instance variable,
    // invoke setValue:forUndefinedKey:.
    // This raises an exception by default, but a subclass of NSObject
    // may provide key-specific behavior.
    let sel = env.objc.lookup_selector("setValue:forUndefinedKey:").unwrap();
    () = msg_send(env, (this, sel, value, key));
}

- (())setValue:(id)_value
forUndefinedKey:(id)key { // NSString*
    // TODO: Raise NSUnknownKeyException
    let class: Class = ObjC::read_isa(this, &env.mem);
    let class_name_string = env.objc.get_class_name(class).to_owned(); // TODO: Avoid copying
    let key_string = to_rust_string(env, key);
    panic!("Object {:?} of class {:?} ({:?}) does not have a setter for {} ({:?})\
        \nAvailable selectors: {}\nAvailable ivars: {}",
        this, class_name_string, class, key_string, key,
        env.objc.debug_all_class_selectors_as_strings(&env.mem, class).join(", "),
        env.objc.debug_all_class_ivars_as_strings(class).join(", "));
}

- (())willChangeValueForKey:(id)_key { // NSString *
    log_once!("TODO: NSObject willChangeValueForKey:");
}
- (())didChangeValueForKey:(id)_key { // NSString *
    log_once!("TODO: NSObject didChangeValueForKey:");
}

- (bool)respondsToSelector:(SEL)selector {
    env.objc.object_has_method(&env.mem, this, selector)
}

- (bool)conformsToProtocol:(id)protocol { // Protocol*
    // Answering "does not conform" unconditionally is NOT safe: apps use this
    // to decide whether to configure an object at all. Dragon Island Blue's
    // RendererFactory does `if ([node conformsToProtocol:@protocol(...)])
    // [node setupForRenderable:context:]`, so a blanket false left every map
    // node unconfigured -- towns became unenterable, with no error anywhere.
    let protocol: ConstVoidPtr = Ptr::from_bits(protocol.to_bits());
    let Some(name) = env.objc.protocol_name_of(protocol, &env.mem) else {
        return false;
    };
    let class = ObjC::read_isa(this, &env.mem);
    let conforms = env.objc.class_conforms_to_protocol(class, &name, &env.mem);
    if !conforms {
        // Host classes don't declare their protocols, so this can still be a
        // false negative; log it, since the failure mode is silent misbehavior.
        log_dbg!(
            "conformsToProtocol: {} does not conform to {:?}",
            env.objc.get_class_name(class),
            name
        );
    }
    conforms
}

- (MutVoidPtr)methodForSelector:(SEL)selector {
    let class = ObjC::read_isa(this, &env.mem);
    if let Some(imp) = env.objc.class_lookup_guest_imp(class, selector) {
        MutVoidPtr::from_bits(imp.addr_with_thumb_bit())
    } else {
        // cocos2d caches IMPs of its own (guest) methods, so this normally
        // resolves. A host-implemented method (or unknown selector) has no
        // guest address; surface it rather than return a silently-bad pointer.
        log!(
            "TODO: methodForSelector: returned NULL for {} (host-implemented or unknown selector)",
            selector.as_str(&env.mem)
        );
        MutVoidPtr::null()
    }
}

- (id)performSelector:(SEL)sel {
    assert!(!sel.is_null());
    msg_send_no_type_checking(env, (this, sel))
}

- (id)performSelector:(SEL)sel
           withObject:(id)o1 {
    assert!(!sel.is_null());
    msg_send_no_type_checking(env, (this, sel, o1))
}

- (id)performSelector:(SEL)sel
           withObject:(id)o1
           withObject:(id)o2 {
    assert!(!sel.is_null());
    msg_send_no_type_checking(env, (this, sel, o1, o2))
}

- (())performSelectorInBackground:(SEL)sel
                       withObject:(id)arg {
    detach_new_thread_inner(env, sel, this, arg, /* tolerate_type_mismatch: */ true)
}

- (())performSelector:(SEL)sel withObject:(id)arg afterDelay:(NSTimeInterval)delay {
    let run_loop: id = msg_class![env; NSRunLoop currentRunLoop];
    add_perform_request(env, run_loop, this, sel, arg, Some(delay), false);
}

- (())performSelectorOnMainThread:(SEL)sel withObject:(id)arg waitUntilDone:(bool)wait {
    log_dbg!("performSelectorOnMainThread:{} withObject:{:?} waitUntilDone:{}", sel.as_str(&env.mem), arg, wait);
    if wait && env.current_thread == 0 {
        if sel.as_str(&env.mem).ends_with(':') {
            () = msg_send(env, (this, sel, arg));
        } else {
            // A no-argument selector: iOS ignores any withObject: value rather
            // than requiring it to be nil.
            () = msg_send(env, (this, sel));
        }
        return;
    }
    if env.bundle.bundle_identifier().starts_with("com.gameloft.POP") && (sel == env.objc.lookup_selector("startMovie:").unwrap() || sel == env.objc.lookup_selector("stopMovie").unwrap()) && wait {
        log!("Applying game-specific hack for PoP: WW: ignoring performSelectorOnMainThread:SEL({}) waitUntilDone:true", sel.as_str(&env.mem));
        return;
    }
    if env.bundle.bundle_identifier().starts_with("com.gameloft.Asphalt5") && (sel == env.objc.lookup_selector("startMovie:").unwrap() || sel == env.objc.lookup_selector("stopMovie:").unwrap()) && wait {
        log!("Applying game-specific hack for Asphalt5: ignoring performSelectorOnMainThread:SEL({}) waitUntilDone:true", sel.as_str(&env.mem));
        return;
    }
    if env.bundle.bundle_identifier().starts_with("com.gameloft.SplinterCell") && sel == env.objc.lookup_selector("startMovie:").unwrap() && wait {
        log!("Applying game-specific hack for SplinterCell: ignoring performSelectorOnMainThread:SEL({}) waitUntilDone:true", sel.as_str(&env.mem));
        return;
    }
    if env.bundle.bundle_identifier().starts_with("com.gameloft.AssassinsCreed") && sel == env.objc.lookup_selector("moviePlayerInit:").unwrap() && wait {
        log!("Applying game-specific hack for AssassinsCreed: ignoring performSelectorOnMainThread:SEL(moviePlayerInit:) waitUntilDone:true");
        return;
    }
    if env.bundle.bundle_identifier().starts_with("com.gameloft.Ferrari") && wait {
        if sel == env.objc.lookup_selector("startMovie:").unwrap() {
            log!("Applying game-specific hack for Ferrari GT: ignoring performSelectorOnMainThread:SEL({}) waitUntilDone:true", sel.as_str(&env.mem));
            return;
        }
        if sel == env.objc.lookup_selector("initTextInput:").unwrap() || sel == env.objc.lookup_selector("removeTextField:").unwrap() {
            log!("Applying game-specific hack for Ferrari GT: performing performSelectorOnMainThread:SEL({}) waitUntilDone:true on thread {}", sel.as_str(&env.mem), env.current_thread);
            () = msg_send(env, (this, sel, arg));
            return;
        }
    }
    if env.bundle.bundle_identifier().starts_with("com.gameloft.HOS2") && wait {
        if sel == env.objc.lookup_selector("loadMovie:").unwrap() || sel == env.objc.lookup_selector("sendGameInfo").unwrap() || sel == env.objc.lookup_selector("setStatusBar:").unwrap() {
            log!("Applying game-specific hack for HOS2: performing performSelectorOnMainThread:SEL({}) waitUntilDone:true on thread {}", sel.as_str(&env.mem), env.current_thread);
            if sel.as_str(&env.mem).ends_with(':') {
                () = msg_send(env, (this, sel, arg));
            } else {
                assert!(arg.is_null());
                () = msg_send(env, (this, sel));
            }
            return;
        }
        if sel == env.objc.lookup_selector("startMovie:").unwrap() || sel == env.objc.lookup_selector("stopMovie:").unwrap() {
            log!("Applying game-specific hack for HOS2: ignoring performSelectorOnMainThread:SEL({}) waitUntilDone:true", sel.as_str(&env.mem));
            return;
        }
    }

    let run_loop: id = msg_class![env; NSRunLoop mainRunLoop];
    let sem = add_perform_request(env, run_loop, this, sel, arg, None, wait);
    if wait {
        sem_wait(env, sem);
        host_destroy_semaphore(env, sem);
    }
}

// UINibLoadingAdditions protocol
- (())awakeFromNib {
    // no-op
}

@end

};

#[cfg(test)]
mod tests {
    use super::type_tokens;

    #[test]
    fn method_encoding_splits_into_return_and_args() {
        // -(void)setFoo:(float) => void, self, _cmd, float
        assert_eq!(type_tokens("v12@0:4f8"), vec!["v", "@", ":", "f"]);
    }

    #[test]
    fn quoted_class_names_and_structs_stay_whole() {
        assert_eq!(
            type_tokens("v20@0:4@\"NSString\"8{CGPoint=ff}12"),
            vec!["v", "@", ":", "@\"NSString\"", "{CGPoint=ff}"]
        );
    }

    #[test]
    fn qualifiers_are_dropped() {
        assert_eq!(type_tokens("v12@0:4rf8"), vec!["v", "@", ":", "f"]);
    }
}
