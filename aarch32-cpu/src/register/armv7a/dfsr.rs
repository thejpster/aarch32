//! Code for managing DFSR (*Data Fault Status Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DFSR (*Instruction Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dfsr {
    /// FAR not Valid
    #[bit(16, rw)]
    fnv: bool,
    /// Cache manintenance fault
    #[bit(13, rw)]
    cm: bool,
    /// External Abort type
    #[bit(12, rw)]
    ext: bool,
    /// Write not Read
    #[bit(11, rw)]
    wnr: bool,
    /// Status bitfield.
    #[bits([0..=5], rw)]
    status: Option<DfsrStatus>,
}

/// Fault status register enumeration for DFSR
#[bitbybit::bitenum(u6, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DfsrStatus {
    /// Alignment fault.
    AlignmentFault = 0b00001,
    /// Debug exception.
    Debug = 0b00010,
    /// Access flag fault, level 1
    AccessFlagFaultFirstLevel = 0b00011,
    /// Fault on instruction cache maintenance.
    CacheMaintenance = 0b00100,
    /// Translation fault, level 1
    TranslationFaultFirstLevel = 0b00101,
    /// Access flag fault, level 2.
    AccessFlagFaultSecondLevel = 0b00110,
    /// Translation fault, level 2.
    TranslationFaultSecondLevel = 0b00111,
    /// Synchronous External abort, not on translation table walk.
    SyncExtAbort = 0b01000,
    /// Domain fault, level 1
    DomainFaultFirstLevel = 0b01001,
    /// Domain fault, level 2.
    DomainFaultSecondLevel = 0b01011,
    /// Synchronous External abort, on translation table walk, level 1
    SyncExtAbortOnTranslationTableWalkFirstLevel = 0b01100,
    /// Permission fault, level 1
    PermissionFaultFirstLevel = 0b01101,
    /// Synchronous External abort, on translation table walk, level 2.
    SyncExtAbortOnTranslationTableWalkSecondLevel = 0b01110,
    /// Permission fault, level 2.
    PermissionFaultSecondLevel = 0b01111,
    /// TLB conflict abort.
    TldConflictAbort = 0b10000,
    /// SError exception.
    SError = 0b10110,
    /// SError exception, from a parity or ECC error on memory access.
    SErrorParityEccError = 0b11000,
    /// Synchronous parity or ECC error on memory access, not on translation table walk.
    SyncParErrorOnMemAccess = 0b11001,
    /// Synchronous parity or ECC error on translation table walk, level 1
    SyncParErrorOnTranslationTableWalkFirstLevel = 0b11100,
    /// Synchronous parity or ECC error on translation table walk, level 2.
    SyncParErrorOnTranslationTableWalkSecondLevel = 0b11110,
}

impl SysReg for Dfsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Dfsr {}

impl Dfsr {
    #[inline]
    /// Reads DFSR (*Data Fault Status Register*)
    pub fn read() -> Dfsr {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Dfsr {}

impl Dfsr {
    #[inline]
    /// Writes DFSR (*Data Fault Status Register*)
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
