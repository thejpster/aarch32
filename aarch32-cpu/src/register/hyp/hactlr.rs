//! Code for managing HACTRL (*Hyp Auxiliary Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HACTRL (*Hyp Auxiliary Control Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hactlr {
    /// Controls access to IMP_TESTR1 at EL0 and EL1
    #[bits(15..=15, rw)]
    testr1: bool,
    /// Controls access to IMP_DCERR0, IMP_DCERR1, IMP_ICERR0, IMP_ICERR1,
    /// IMP_TCMERR0, IMP_TCMERR1, IMP_FLASHERR0, and IMP_FLASHERR1 registers
    #[bits(13..=13, rw)]
    err: bool,
    /// Controls access to IMP_INTMONR at EL1
    #[bits(12..=12, rw)]
    intmonr: bool,
    /// Controls access to IMP_BUSTIMEOUTR at EL1
    #[bits(10..=10, rw)]
    bustimeoutr: bool,
    /// Controls access to QOSR at EL1
    #[bits(9..=9, rw)]
    qosr: bool,
    /// Controls access to IMP_PERIPHPREGIONR at EL1
    #[bits(8..=8, rw)]
    periphpregionr: bool,
    /// Controls access to IMP_FLASHIFREGIONR at EL1
    #[bits(7..=7, rw)]
    flashifregionr: bool,
    /// Controls access to CDBGDCI at EL1
    #[bits(1..=1, rw)]
    cdbgdci: bool,
    /// IMP_CPUACTLR write access control
    #[bits(0..=0, rw)]
    cpuactlr: bool,
}

impl SysReg for Hactlr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}

impl SysRegRead for Hactlr {}

impl SysRegWrite for Hactlr {}

impl Hactlr {
    /// Read HACTRL (*Hyp Auxiliary Control Register*)
    #[inline]
    pub fn read() -> Hactlr {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }

    /// Write HACTRL (*Hyp Auxiliary Control Register*)
    #[inline]
    pub fn write(value: Self) {
        // Safety: Writing this register is atomic
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }

    /// Modify HACTRL (*Hyp Auxiliary Control Register*)
    #[inline]
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }
}
