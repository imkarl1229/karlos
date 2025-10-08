use core::arch::asm;

#[inline(always)]
pub unsafe fn disable_interrupt() {
    unsafe { asm!("cli") }
}
