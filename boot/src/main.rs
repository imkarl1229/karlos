#![no_std]
#![no_main]

mod elf_headers;

use crate::elf_headers::*;
use core::mem::{size_of, transmute};
use core::panic::PanicInfo;
use core::ptr::write;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use shared::boot_info::*;
use uefi::boot::{
    AllocateType, MemoryType, allocate_pages, exit_boot_services, get_handle_for_protocol,
    image_handle, memory_map, open_protocol_exclusive,
};
use uefi::mem::memory_map::MemoryMap;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::system::with_config_table;
use uefi::table::cfg::{ACPI_GUID, ACPI2_GUID};

const KERNEL_PATH: &uefi::CStr16 = cstr16!("\\KERNEL.ELF");
const KERNEL_LOAD_ADDR: u64 = 0x100000; // 1 MB, classic location

type KernelEntry = extern "Rust" fn(boot_info_ptr: *const BootInfo) -> !;

#[entry]
fn main() -> Status {
    // Load kernel
    let mut kernel = {
        open_protocol_exclusive::<SimpleFileSystem>(image_handle())
            .unwrap()
            .open_volume()
            .unwrap()
            .open(KERNEL_PATH, FileMode::Read, FileAttribute::empty())
            .unwrap()
            .into_regular_file()
            .unwrap()
    };

    let mut info_buf = [0u8; 512];
    let info = kernel.get_info::<FileInfo>(&mut info_buf).unwrap();
    let kernel_size = info.file_size() as usize;
    // Allocate memory for kernel
    allocate_pages(
        AllocateType::Address(KERNEL_LOAD_ADDR),
        MemoryType::LOADER_DATA,
        kernel_size.div_ceil(0x1000),
    )
    .unwrap();
    let elf_buf: &mut [u8] =
        unsafe { from_raw_parts_mut(KERNEL_LOAD_ADDR as *mut u8, kernel_size) };

    kernel.read(elf_buf).unwrap();

    let ehdr = unsafe { &*(elf_buf.as_ptr() as *const Elf64Ehdr) };
    assert_eq!(&ehdr.e_ident[0..4], b"\x7FELF"); // sanity check
    let phdrs = unsafe {
        from_raw_parts(
            elf_buf.as_ptr().add(ehdr.e_phoff as usize) as *const Elf64Phdr,
            ehdr.e_phnum as usize,
        )
    };

    for ph in phdrs {
        if ph.p_type != PT_LOAD {
            continue;
        }

        let addr = ph.p_paddr;
        let pages = ph.p_memsz.div_ceil(0x1000) as usize;

        allocate_pages(AllocateType::Address(addr), MemoryType::LOADER_DATA, pages).unwrap();

        let dest = unsafe { from_raw_parts_mut(addr as *mut u8, ph.p_memsz as usize) };
        let src = &elf_buf[ph.p_offset as usize..(ph.p_offset + ph.p_filesz) as usize];

        dest[..src.len()].copy_from_slice(src);

        // Zero out .bss
        for b in &mut dest[src.len()..] {
            *b = 0;
        }
    }
    // Kernel now loaded into KERNEL_LOAD_ADDRESS

    // Now getting framebuffer info
    let binding = open_protocol_exclusive::<GraphicsOutput>(
        get_handle_for_protocol::<GraphicsOutput>().unwrap(),
    );
    let mut binding = binding.unwrap();
    let gop = binding.get_mut().unwrap();

    let mode = gop.current_mode_info();
    let mut fb = gop.frame_buffer();
    let resolution = mode.resolution();

    let fb_info = FramebufferInfo {
        addr: fb.as_mut_ptr() as u64,
        width: resolution.0 as u32,
        height: resolution.1 as u32,
        pixels_per_scanline: mode.stride() as u32,
        format: mode.pixel_format() as u32,
    };

    // Get ACPI RSDP
    let rsdp_addr = with_config_table(|config_tables| {
        config_tables
            .iter()
            .find(|e| e.guid == ACPI2_GUID || e.guid == ACPI_GUID)
            .map(|e| e.address as u64)
            .unwrap_or(0)
    });

    // Allocate for boot_info
    let pages = size_of::<BootInfo>().div_ceil(0x1000);
    let boot_info_addr = allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages)
        .unwrap()
        .as_ptr() as *mut BootInfo;

    // Get memory map after all allocations
    let memory_map = match memory_map(MemoryType::LOADER_DATA) {
        Ok(r) => r,
        Err(_) => return Status::LOAD_ERROR,
    };
    let memory_map_buf = memory_map.buffer();

    let mm_info = MemoryMapInfo {
        ptr: memory_map_buf.as_ptr(),
        size: memory_map_buf.len(),
        desc_size: memory_map_buf.len() / memory_map.entries().count(),
    };

    let boot_info = BootInfo {
        memory_map: mm_info,
        framebuffer: fb_info,
        rsdp_addr,
    };

    unsafe {
        // Load boot info
        write(boot_info_addr, boot_info);
        // Exit boot services, it had finished its duty
        let _ = exit_boot_services(None);
    }

    // Jump into kernel
    let kernel_entry_addr = ehdr.e_entry as usize;
    let kernel_entry_fn: KernelEntry = unsafe { transmute(kernel_entry_addr) };

    // Enter kernel
    kernel_entry_fn(boot_info_addr as *const BootInfo);
}

#[panic_handler]
fn _panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
