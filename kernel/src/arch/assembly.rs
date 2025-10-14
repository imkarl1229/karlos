//! Module that contain ArchAssembly trait

/// Trait that contain assembly functions required by any arch
/// # Safety
/// Only impl this trait on Assembly struct in arch::current_arch
pub unsafe trait ArchAssembly {
    /// Disable interrupt, if interrupt occur, triple fault/general protection can be triggered
    unsafe fn disable_interrupt();
    /// Enable interrupt, if GDT not loaded, triple fault/general protection can be triggered
    unsafe fn enable_interrupt();
}
