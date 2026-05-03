//! Code for managing Protection Area 6 (Protected Memory on Armv5TE)

use super::pac::Pac;
use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CP15 Register 6, 6: Protection Area 6
pub struct Pac6();

impl SysReg for Pac6 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Pac6 {}

impl Pac6 {
    #[inline]
    /// Reads Protecton Area 6
    pub fn read() -> Pac {
        Pac::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Pac6 {}

impl Pac6 {
    #[inline]
    /// Writes Protecton Area 6
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Pac) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
