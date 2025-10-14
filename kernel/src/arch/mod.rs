//! CPU-arch-depended codes. Like assembly instructions.
//! Functions re-exported in this module are dangerous, use them at your own risk

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
pub mod assembly;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
pub use aarch64 as current_arch;
#[cfg(target_arch = "x86_64")]
pub use x86_64 as current_arch;
