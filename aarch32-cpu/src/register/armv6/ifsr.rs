//! Code for managing IFSR (*Instruction Fault Status Register*)

#[allow(unused)]
use arbitrary_int::u4;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IFSR (*Instruction Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ifsr {
    /// Which domain was being accessed
    #[bits(4..=7, rw)]
    domain: u4,
    /// Status bitfield.
    #[bits([0..=3], rw)]
    status: Option<IfsrStatus>,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u4, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum IfsrStatus {
    /// PC Alignment Fault
    Alignment = 1,
    /// Debug Exception
    DebugEvent = 2,
    /// Access Flag fault, level 1
    AccessFlagFaultFirstLevel = 3,
    /// Translation fault, level 1
    TranslationFaultFirstLevel = 5,
    /// Access Flag fault, level 2
    AccessFlagFaultSecondLevel = 6,
    /// Translation fault, level 2
    TranslationFaultSecondLevel = 7,
    /// Synchronouse External Abort
    SyncExtAbort = 8,
    /// Domain fault, level 1
    DomainFaultFirstLevel = 9,
    /// Domain fault, level 2
    DomainFaultSecondLevel = 11,
    /// Synchronous External abort, on translation table walk, level 1
    SyncExtAbortOnTranslationTableWalkFirstLevel = 12,
    /// Permission fault, level 1
    PermissionFaultFirstLevel = 13,
    /// Synchronous External abort, on translation table walk, level 2
    SyncExtAbortOnTranslationTableWalkSecondLevel = 14,
    /// Permission fault, level 2
    PermissionFaultSecondLevel = 15,
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
