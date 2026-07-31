/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `dlfcn.h` (`dlopen()` and friends)

use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::{ConstPtr, MutVoidPtr, Ptr};
use crate::Environment;

const RTLD_DEFAULT: MutVoidPtr = Ptr::from_bits(-2 as _);

fn is_known_library(path: &str) -> bool {
    crate::dyld::DYLIB_LIST
        .iter()
        .any(|dylib| dylib.path == path || dylib.aliases.contains(&path))
}

fn dlopen(env: &mut Environment, path: ConstPtr<u8>, _mode: i32) -> MutVoidPtr {
    if path.is_null() {
        return RTLD_DEFAULT;
    }
    // TODO: dlopen() support for real dynamic libraries.
    assert!(is_known_library(env.mem.cstr_at_utf8(path).unwrap()));
    // For convenience, use the path as the handle.
    // TODO: Find out whether the handle is truly opaque on iPhone OS, and if
    // not, where it points.
    path.cast_mut().cast()
}

fn dlsym(env: &mut Environment, handle: MutVoidPtr, symbol: ConstPtr<u8>) -> MutVoidPtr {
    assert!(
        handle == RTLD_DEFAULT || is_known_library(env.mem.cstr_at_utf8(handle.cast()).unwrap())
    );
    // For some reason, the symbols passed to dlsym() don't have the leading _.
    let symbol = format!("_{}", env.mem.cstr_at_utf8(symbol).unwrap());
    // TODO: Symbol lookup should be scoped to the specific library requested,
    // where appropriate!

    // First, try to resolve the symbol as a function.
    if let Ok(addr) = env
        .dyld
        .create_proc_address(&mut env.mem, &mut env.cpu, &symbol)
    {
        return Ptr::from_bits(addr.addr_with_thumb_bit());
    }

    // Otherwise, try to resolve it as an exported constant. Some apps look up
    // constants such as UIKit notification-name NSStrings via dlsym() at
    // runtime rather than through a normal non-lazy relocation. Materialize the
    // constant and return the address of the cell holding its value, matching
    // the address a static reference to the symbol would resolve to.
    if let Some((_, template)) =
        crate::dyld::search_host_dylibs(|dylib| dylib.constant_exports, &symbol)
    {
        let value_ptr = crate::dyld::Dyld::materialize_constant(env, template);
        return Ptr::from_bits(value_ptr.to_bits());
    }

    // TODO: real dlsym() returns NULL for a missing symbol. We panic instead,
    // since during bring-up an unresolved symbol most likely indicates a
    // missing host function/constant we should implement.
    panic!("dlsym() for unimplemented symbol {symbol}");
}

fn dlclose(env: &mut Environment, handle: MutVoidPtr) -> i32 {
    assert!(
        handle == RTLD_DEFAULT || is_known_library(env.mem.cstr_at_utf8(handle.cast()).unwrap())
    );
    0 // success
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(dlopen(_, _)),
    export_c_func!(dlsym(_, _)),
    export_c_func!(dlclose(_)),
];
