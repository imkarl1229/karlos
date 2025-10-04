use core::arch::naked_asm;

/// Self-Explanatory
#[unsafe(naked)]
pub unsafe extern "C" fn disable_interrupt() {
    naked_asm!("cli");
}

/// Self-Explanatory
#[unsafe(naked)]
pub unsafe extern "C" fn enable_interrupt() {
    naked_asm!("sti")
}