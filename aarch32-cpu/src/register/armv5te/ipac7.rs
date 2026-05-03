//! Code for managing Instruction Protecton Area 7 (Protected Memory on Armv5TE)

use super::pac::Pac;
use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CP15 Register 6, 7: Instruction Protecton Area 7
pub struct Ipac7();

impl SysReg for Ipac7 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Ipac7 {}

impl Ipac7 {
    #[inline]
    /// Reads Instruction Protecton Area 7
    pub fn read() -> Pac {
        Pac::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Ipac7 {}

impl Ipac7 {
    #[inline]
    /// Writes Instruction Protecton Area 7
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
