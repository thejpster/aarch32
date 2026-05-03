//! Code for managing MIDR (*Main ID Register*)

use arbitrary_int::{u12, u4};

use crate::register::{SysReg, SysRegRead};

/// MIDR (*Main ID Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct Midr {
    /// Implementer
    #[bits(24..=31, r)]
    implementer: u8,
    /// Variant
    #[bits(20..=23, r)]
    variant: u4,
    /// Architecture
    #[bits(16..=19, r)]
    arch: u4,
    /// Part Number
    #[bits(4..=15, r)]
    part_no: u12,
    /// Revision
    #[bits(0..=3, r)]
    rev: u4,
}

impl SysReg for Midr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl SysRegRead for Midr {}

impl Midr {
    /// Read MIDR (*Main ID Register*)
    #[inline]
    pub fn read() -> Midr {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}
