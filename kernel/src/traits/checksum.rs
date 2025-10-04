use core::slice::from_raw_parts;

/// Trait that help checksum
/// # Safety
/// Never impl this trait on struct with padding and/or reference
pub unsafe trait Checksum {
    /// # Safety
    /// This code use core::slice::from_raw_parts, which is dangerous to struct with padding or reference
    unsafe fn checksum(&self) -> bool
    where
        Self: Sized,
    {
        let size = size_of::<Self>();
        let b = unsafe { from_raw_parts(self as *const Self as *const u8, size) };
        let mut sum = 0u16;
        for &i in b {
            sum = sum.wrapping_add(i as u16);
        }

        sum = !sum;
        sum == 0
    }
}
