//! Code for managing Bufferability (Protected Memory on Armv5TE)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CP15 Register 3: Bufferability bits
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bufferability {
    /// Bits B7 through B0
    #[bit(0, rw)]
    bits: [bool; 8],
}

impl SysReg for Bufferability {
    const CP: u32 = 15;
    const CRN: u32 = 3;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Bufferability {}

impl Bufferability {
    #[inline]
    /// Reads Bufferability
    pub fn read() -> Bufferability {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Bufferability {}

impl Bufferability {
    #[inline]
    /// Writes Bufferability
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
