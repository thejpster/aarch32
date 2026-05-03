//! Code for managing Cacheability (Protected Memory on Armv5TE)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CP15 Register 2: Cacheability bits
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cacheability {
    /// Bits C7 through C0
    #[bit(0, rw)]
    bits: [bool; 8],
}

impl SysReg for Cacheability {
    const CP: u32 = 15;
    const CRN: u32 = 2;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Cacheability {}

impl Cacheability {
    #[inline]
    /// Reads Cacheability
    pub fn read() -> Cacheability {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Cacheability {}

impl Cacheability {
    #[inline]
    /// Writes Cacheability
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }
}
