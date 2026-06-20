//! Code for managing MPUIR (*MPU Type Register*)

use crate::register::{SysReg, SysRegRead};

/// MPUIR (*MPU Type Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mpuir {
    /// Specifies the number of Instruction regions implemented by the MPU.
    ///
    /// If the MPU implements a Unified memory map this field is UNK/SBZ.
    #[bits(16..=23, r)]
    iregions: u8,
    /// Specifies the number of Data or Unified regions implemented by the MPU.
    #[bits(8..=15, r)]
    dregions: u8,
    /// Is the MPU non-unified
    #[bits(0..=0, r)]
    non_unified: bool,
}

impl SysReg for Mpuir {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 4;
}

impl crate::register::SysRegRead for Mpuir {}

impl Mpuir {
    #[inline]
    /// Reads MPUIR (*MPU Type Register*)
    pub fn read() -> Mpuir {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}
