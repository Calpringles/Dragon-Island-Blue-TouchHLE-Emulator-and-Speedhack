//! Things from `NSObjCRuntime.h`.

use super::ns_string;
use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::{ConstPtr, Ptr};
use crate::objc::{id, nil, Class, SEL};
use crate::Environment;

fn NSStringFromSelector(env: &mut Environment, selector: SEL) -> id {
    // TODO: caching?
    let string = selector.as_str(&env.mem).to_string();
    ns_string::from_rust_string(env, string)
}

fn NSSelectorFromString(env: &mut Environment, string: id) -> SEL {
    // TODO: avoid copy?
    let string = ns_string::to_rust_string(env, string);
    env.objc.register_host_selector(string.into(), &mut env.mem)
}

pub fn NSStringFromClass(env: &mut Environment, class: Class) -> id {
    if class.is_null() {
        return nil;
    }
    // TODO: caching?
    let string = env.objc.get_class_name(class).to_string();
    ns_string::from_rust_string(env, string)
}

fn NSStringFromProtocol(env: &mut Environment, protocol: id) -> id {
    if protocol == nil {
        return nil;
    }
    // touchHLE doesn't track protocols, so a `Protocol *` is just a guest pointer
    // to the binary's `protocol_t` struct: `isa` at offset 0 and a
    // `const char *name` at offset 4 (32-bit). Read the name directly. The name
    // is a static C string in the app binary, so it's valid even though the
    // struct's `isa` may never have been fixed up.
    let name_field: ConstPtr<ConstPtr<u8>> = Ptr::from_bits(protocol.to_bits() + 4);
    let name_ptr: ConstPtr<u8> = env.mem.read(name_field);
    let name = env.mem.cstr_at_utf8(name_ptr).unwrap().to_string();
    ns_string::from_rust_string(env, name)
}

fn NSClassFromString(env: &mut Environment, string: id) -> Class {
    if string == nil {
        return nil;
    }
    // TODO: avoid copy?
    let string = ns_string::to_rust_string(env, string);

    // While this method is supposed to return nil if the class is not found,
    // touchHLE is missing many classes that apps might expect to be present,
    // so this could be troublesome. So, let's use get_known_class, which panics
    // when it can't find the class. We could except certain classes or apps if
    // we need to.
    env.objc.get_known_class(&string, &mut env.mem)
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(NSStringFromSelector(_)),
    export_c_func!(NSSelectorFromString(_)),
    export_c_func!(NSClassFromString(_)),
    export_c_func!(NSStringFromClass(_)),
    export_c_func!(NSStringFromProtocol(_)),
];
