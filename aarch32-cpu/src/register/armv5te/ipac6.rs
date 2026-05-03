//! Code for managing Instruction Protecton Area 6 (Protected Memory on Armv5TE)

use super::pac::Pac;
use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CP15 Register 6, 6: Instruction Protecton Area 6
pub struct Ipac6();

impl SysReg for Ipac6 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Ipac6 {}

impl Ipac6 {
    #[inline]
    /// Reads Instruction Protecton Area 6
    pub fn read() -> Pac {
        Pac::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Ipac6 {}

impl Ipac6 {
    #[inline]
    /// Writes Instruction Protecton Area 6
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
