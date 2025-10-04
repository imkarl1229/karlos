#![no_std]
#![no_main]

mod asm_utils;
mod traits;

use crate::asm_utils::disable_interrupt;
use core::panic::PanicInfo;
use shared::boot_info::BootInfo;

#[unsafe(no_mangle)]
extern "sysv64" fn _start(boot_info_ptr: *const BootInfo) -> ! {
    unsafe { disable_interrupt(); }
    assert_ne!(core::ptr::null(), boot_info_ptr);

    let boot_info = unsafe { &mut *(boot_info_ptr as *mut BootInfo) };

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn _panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
