//! Code for managing IFSR (*Instruction Fault Status Register*)

#[allow(unused)]
use arbitrary_int::u4;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IFSR (*Instruction Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ifsr {
    /// FAR not Valid
    #[bit(16, rw)]
    fnv: bool,
    /// External Abort type
    #[bit(12, rw)]
    ext: bool,
    /// Status bitfield.
    #[bits([0..=5], rw)]
    status: Option<IfsrStatus>,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u6, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum IfsrStatus {
    /// Translation fault
    Translation = 4,
    /// Permission fault
    Permission = 12,
    /// Synchronous External abort
    SyncExtAbort = 16,
    /// Synchronous parity or ECC error on memory access
    SyncParityEccError = 24,
    /// PC alignment fault
    PcAlignment = 33,
    /// Debug exception
    Debug = 34,
}

impl SysReg for Ifsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Ifsr {}

impl Ifsr {
    #[inline]
    /// Reads IFSR (*Instruction Fault Status Register*)
    pub fn read() -> Ifsr {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Ifsr {}

impl Ifsr {
    #[inline]
    /// Writes IFSR (*Instruction Fault Status Register*)
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
