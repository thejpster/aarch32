//! Code for managing Protection Area 1 (Protected Memory on Armv5TE)

use super::pac::Pac;
use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CP15 Register 6, 1: Protection Area 1
pub struct Pac1();

impl SysReg for Pac1 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Pac1 {}

impl Pac1 {
    #[inline]
    /// Reads Protecton Area 1
    pub fn read() -> Pac {
        Pac::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Pac1 {}

impl Pac1 {
    #[inline]
    /// Writes Protecton Area 1
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
