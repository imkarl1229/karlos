#[repr(C)]
#[derive(Copy, Clone)]
pub struct BootInfo {
    pub memory_map: MemoryMapInfo,
    pub framebuffer: FramebufferInfo,
    pub rsdp_addr: u64
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryMapInfo {
    pub ptr: *const u8,
    pub size: usize,
    pub desc_size: usize
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FramebufferInfo {
    pub addr: u64,
    pub width: u32,
    pub height: u32,
    pub pixels_per_scanline: u32,
    pub format: u32
}