/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Stack-smashing protector runtime support (`__stack_chk_guard`,
//! `__stack_chk_fail`).
//!
//! Any guest function compiled with `-fstack-protector` reads the global
//! `__stack_chk_guard` (the stack canary) in its prologue, and on a mismatch in
//! its epilogue calls `__stack_chk_fail`. libSystem normally provides both.
//! Without them the guard's non-lazy pointer stays null, so the first protected
//! function dereferences null (observed crashing `-[AppDelegate
//! application:didFinishLaunchingWithOptions:]` on entry). We provide a fixed
//! canary — its value is irrelevant as long as it is consistent, since guest
//! code only compares the prologue-saved copy against this same global — and an
//! aborting `__stack_chk_fail`.

use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::mem::{ConstVoidPtr, MutPtr};
use crate::Environment;

/// Arbitrary fixed stack canary. Real libSystem randomizes this per process.
const STACK_CHK_GUARD: u32 = 0x2ae5_6b00;

fn stack_chk_guard(env: &mut Environment) -> ConstVoidPtr {
    let cell: MutPtr<u32> = env.mem.alloc_and_write(STACK_CHK_GUARD);
    cell.cast().cast_const()
}

fn __stack_chk_fail(_env: &mut Environment) {
    // Real libSystem aborts the process here. With our constant guard this is
    // only reached if guest code genuinely smashed its stack.
    panic!("__stack_chk_fail: stack smashing detected in guest code");
}

pub const FUNCTIONS: FunctionExports = &[export_c_func!(__stack_chk_fail())];

pub const CONSTANTS: ConstantExports =
    &[("___stack_chk_guard", HostConstant::Custom(stack_chk_guard))];
