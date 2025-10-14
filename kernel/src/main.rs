#![no_std]
#![no_main]

mod arch;
mod traits;

use core::panic::PanicInfo;
use shared::boot_info::BootInfo;
use crate::arch::current::*;

#[unsafe(no_mangle)]
extern "Rust" fn _start(boot_info_ptr: *const BootInfo) -> ! {
    unsafe { disable_interrupt() }
    assert_ne!(core::ptr::null(), boot_info_ptr);

    let boot_info = unsafe { &mut *(boot_info_ptr as *mut BootInfo) };

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn _panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
