/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSOperation`, `NSInvocationOperation` and `NSOperationQueue`.
//!
//! Minimal **synchronous** implementation: an operation added to a queue is run
//! immediately on the calling (main) thread, so there is no real concurrency.
//! This is enough for games that use an `NSOperationQueue` to load resources in
//! the background — the resources still load, just without the background
//! threading (and without any inter-frame progress pacing). `NSInvocationOperation`
//! invokes its target/selector directly, avoiding the need for a full
//! `NSInvocation` implementation.

use super::NSInteger;
use crate::objc::{
    id, msg, msg_class, msg_send_no_type_checking, nil, objc_classes, release, retain,
    ClassExports, HostObject, NSZonePtr, TrivialHostObject, SEL,
};
use crate::Environment;

#[derive(Default)]
struct NSOperationHostObject {
    /// For NSInvocationOperation: the invocation target/selector/argument.
    target: id,
    selector: Option<SEL>,
    object: id,
    cancelled: bool,
    finished: bool,
}
impl HostObject for NSOperationHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSOperation: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NSOperationHostObject {
        target: nil,
        selector: None,
        object: nil,
        cancelled: false,
        finished: false,
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)init {
    this
}

- (())start {
    if env.objc.borrow::<NSOperationHostObject>(this).cancelled {
        env.objc.borrow_mut::<NSOperationHostObject>(this).finished = true;
        return;
    }
    () = msg![env; this main];
    env.objc.borrow_mut::<NSOperationHostObject>(this).finished = true;
}

// Base main does nothing; subclasses override.
- (())main {
}

- (())cancel {
    env.objc.borrow_mut::<NSOperationHostObject>(this).cancelled = true;
}

- (bool)isCancelled {
    env.objc.borrow::<NSOperationHostObject>(this).cancelled
}
- (bool)isFinished {
    env.objc.borrow::<NSOperationHostObject>(this).finished
}
- (bool)isExecuting {
    false
}
- (bool)isConcurrent {
    false
}
- (bool)isReady {
    true
}

- (())dealloc {
    let &NSOperationHostObject { target, object, .. } =
        env.objc.borrow(this);
    release(env, target);
    release(env, object);
    env.objc.dealloc_object(this, &mut env.mem);
}

@end

@implementation NSInvocationOperation: NSOperation

- (id)initWithTarget:(id)target
            selector:(SEL)sel
              object:(id)object {
    retain(env, target);
    retain(env, object);
    let host = env.objc.borrow_mut::<NSOperationHostObject>(this);
    host.target = target;
    host.selector = Some(sel);
    host.object = object;
    this
}

- (())main {
    let (target, selector, object) = {
        let host = env.objc.borrow::<NSOperationHostObject>(this);
        (host.target, host.selector, host.object)
    };
    let Some(sel) = selector else { return; };
    if target == nil {
        return;
    }
    // Discard any return value (NSInvocationOperation would expose it via
    // -result, which this game's loader doesn't use).
    if object == nil {
        let _: id = msg_send_no_type_checking(env, (target, sel));
    } else {
        let _: id = msg_send_no_type_checking(env, (target, sel, object));
    }
}

@end

@implementation NSOperationQueue: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(TrivialHostObject);
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)init {
    this
}

- (())addOperation:(id)op {
    // Synchronous model: run the operation now, on this thread.
    () = msg![env; op start];
}

- (())addOperations:(id)ops // NSArray<NSOperation*>*
     waitUntilFinished:(bool)_wait {
    let count: crate::frameworks::foundation::NSUInteger = msg![env; ops count];
    for i in 0..count {
        let op: id = msg![env; ops objectAtIndex:i];
        () = msg![env; op start];
    }
}

- (())cancelAllOperations {
    // Nothing is ever pending in the synchronous model.
}

- (())setMaxConcurrentOperationCount:(NSInteger)_count {
}
- (NSInteger)maxConcurrentOperationCount {
    1
}

- (())setSuspended:(bool)_suspended {
}
- (bool)isSuspended {
    false
}

- (())setName:(id)_name { // NSString*
}

- (())waitUntilAllOperationsAreFinished {
    // Everything already ran synchronously.
}

- (id)operations {
    msg_class![env; NSArray array]
}
- (crate::frameworks::foundation::NSUInteger)operationCount {
    0
}

@end

};
