//! Code for managing DFSR (*Data Fault Status Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DFSR (*Data Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dfsr {
    /// Status
    #[bits([0..=3], rw)]
    status: Option<DfsrStatus>,
}

/// Fault status register enumeration for DFSR
#[bitbybit::bitenum(u4, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DfsrStatus {
    /// Alignment fault
    AlignmentFault = 1,
    /// Debug Exception
    Debug = 2,
    /// Alternate value for Alignment fault
    AlignmentAlt = 3,
    /// Translation fault, level 1
    TranslationFaultFirstLevel = 5,
    /// Translation fault, level 2
    TranslationFaultSecondLevel = 7,
    /// Synchronous External Abort
    SyncExtAbort = 8,
    /// Domain fault, level 1
    DomainFaultFirstLevel = 9,
    /// Alternate value for Synchronous External Abort
    SyncExtAbortAlt = 10,
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
