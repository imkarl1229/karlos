pub use crate::arch::assembly::ArchAssembly;
use core::arch::asm;

pub struct Assembly;

unsafe impl ArchAssembly for Assembly {
    #[inline(always)]
    unsafe fn disable_interrupt() {
        unsafe { asm!("cli") }
    }

    #[inline(always)]
    unsafe fn enable_interrupt() {
        unsafe { asm!("sti") }
    }
}
