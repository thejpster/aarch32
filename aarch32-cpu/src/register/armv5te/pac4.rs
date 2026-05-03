//! Code for managing Protection Area 4 (Protected Memory on Armv5TE)

use super::pac::Pac;
use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CP15 Register 6, 4: Protection Area 4
pub struct Pac4();

impl SysReg for Pac4 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 4;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Pac4 {}

impl Pac4 {
    #[inline]
    /// Reads Protecton Area 4
    pub fn read() -> Pac {
        Pac::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Pac4 {}

impl Pac4 {
    #[inline]
    /// Writes Protecton Area 4
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
