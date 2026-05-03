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
    #[bits([0..=3, 10], rw)]
    status: Option<IfsrStatus>,
}

/// Fault status register enumeration for IFSR
#[bitbybit::bitenum(u5, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
pub enum IfsrStatus {
    /// Synchronous External abort, on translation table walk, level 1
    SyncExtAbortOnTranslationTableWalkFirstLevel = 0b01100,
    /// Synchronous External abort, on translation table walk, level 2
    SyncExtAbortOnTranslationTableWalkSecondLevel = 0b01110,
    /// Synchronous parity or ECC error on memory access, on translation table walk, level 1
    SyncParErrorOnTranslationTableWalkFirstLevel = 0b11100,
    /// Synchronous parity or ECC error on memory access, on translation table walk, level 2
    SyncParErrorOnTranslationTableWalkSecondLevel = 0b11110,
    /// Translation fault, level 1
    TranslationFaultFirstLevel = 0b00101,
    /// Translation fault, level 2
    TranslationFaultSecondLevel = 0b00111,
    /// Access flag fault, level 1
    AccessFlagFaultFirstLevel = 0b00011,
    /// Access flag fault, level 2
    AccessFlagFaultSecondLevel = 0b00110,
    /// Domain fault, level 1
    DomainFaultFirstLevel = 0b01001,
    /// Domain fault, level 2
    DomainFaultSecondLevel = 0b01011,
    /// Permission fault, level 1
    PermissionFaultFirstLevel = 0b01101,
    /// Permission fault, level 2
    PermissionFaultSecondLevel = 0b01111,
    /// Debug exception
    DebugEvent = 0b00010,
    /// Synchronous External abort
    SyncExtAbort = 0b01000,
    /// TLB conflict abort
    TlbConflictAbort = 0b10000,
    /// IMPLEMENTATION DEFINED fault (Lockdown fault)
    Lockdown = 0b10100,
    /// Co-Processor Abort
    CoprocessorAbort = 0b11010,
    /// Synchronous parity or ECC error on memory access, not on translation table walk
    SyncParErrorOnMemAccess = 0b11001,
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
