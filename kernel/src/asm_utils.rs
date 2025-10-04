//! This module contain assembly utils for low level operation
//! # Safety
//! Functions in this module are dangerous, there will not be any check on these low level instructions to the CPU.
//! Avoid using functions in this file unless you KNOW what you are doing!

use core::arch::asm;

/// Self-Explanatory
/// # Safety
/// This function can only be run on Ring 0, otherwise CPU will throw General Protection
#[inline]
pub unsafe fn disable_interrupt() {
    unsafe { asm!("cli") }
}

/// Self-Explanatory
/// # Safety
/// This function can only be run on Ring 0, otherwise CPU will throw General Protection
#[inline]
pub unsafe fn enable_interrupt() {
    unsafe { asm!("sti") }
}

/// Self-Explanatory
/// # Safety
/// This function can only be run on Ring 0, otherwise CPU will throw General Protection
#[inline]
pub unsafe fn halt_cpu() {
    unsafe { asm!("hlt") }
}
