/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Grand Central Dispatch (`libdispatch`), and the block-calling support it
//! needs.
//!
//! touchHLE runs the guest on a single host thread, so there is no thread pool
//! here: anything dispatched is simply run where it stands.

use crate::abi::{CallFromHost, GuestFunction};
use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::{ConstVoidPtr, GuestUSize, MutPtr, Ptr, SafeRead};
use crate::Environment;

/// The start of a block literal, as laid out by the Objective-C block ABI. The
/// captured variables follow `descriptor` and are only known to the block's own
/// code, so they are not described here.
#[repr(C, packed)]
struct BlockLiteral {
    isa: ConstVoidPtr,
    flags: i32,
    reserved: i32,
    /// The block's code. Takes the block itself as its first argument, which is
    /// how it reaches its captured variables.
    invoke: GuestFunction,
    descriptor: ConstVoidPtr,
}
unsafe impl SafeRead for BlockLiteral {}

/// Call a block that takes no arguments beyond itself and returns nothing.
pub fn call_block(env: &mut Environment, block: ConstVoidPtr) {
    let literal: BlockLiteral = env.mem.read(block.cast());
    let invoke = literal.invoke;
    invoke.call_from_host(env, (block,))
}

/// `dispatch_once_t`, a word that is zero until the block has run.
type dispatch_once_t = GuestUSize;

fn dispatch_once(env: &mut Environment, predicate: MutPtr<dispatch_once_t>, block: ConstVoidPtr) {
    if env.mem.read(predicate) != 0 {
        return;
    }
    // Set the flag before running the block, not after: a block that re-enters
    // dispatch_once with the same predicate would otherwise recurse forever.
    // Real GCD deadlocks in that situation, which is no more useful.
    env.mem.write(predicate, 1);
    call_block(env, block);
}

/// `dispatch_once_f`, the callback-and-context form, which needs no block.
fn dispatch_once_f(
    env: &mut Environment,
    predicate: MutPtr<dispatch_once_t>,
    context: ConstVoidPtr,
    function: GuestFunction,
) {
    if env.mem.read(predicate) != 0 {
        return;
    }
    env.mem.write(predicate, 1);
    function.call_from_host(env, (context,))
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(dispatch_once(_, _)),
    export_c_func!(dispatch_once_f(_, _, _)),
];

/// `_NSConcreteStackBlock` and friends are the isa values the compiler stores in
/// block literals. Nothing here reads a block's isa — [call_block] only needs
/// `invoke` — but the symbols must resolve to *something*, or every block
/// literal the guest builds gets a null isa written through an unlinked GOT
/// slot.
pub const CONSTANTS: crate::dyld::ConstantExports = &[
    (
        "__NSConcreteStackBlock",
        crate::dyld::HostConstant::Custom(|env| {
            env.mem
                .alloc_and_write(Ptr::<u8, false>::null())
                .cast_void()
                .cast_const()
        }),
    ),
    (
        "__NSConcreteGlobalBlock",
        crate::dyld::HostConstant::Custom(|env| {
            env.mem
                .alloc_and_write(Ptr::<u8, false>::null())
                .cast_void()
                .cast_const()
        }),
    ),
];
