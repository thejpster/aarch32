//! Code for managing DFSR (*Data Fault Status Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DFSR (*Data Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
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
    /// Translation fault
    Translation = 4,
    /// Permission fault
    Permission = 12,
    /// Synchronous external abort, other than synchronous parity or ECC error
    SyncExtAbort = 16,
    /// SError interrupt
    SErrorInterrupt = 17,
    /// Synchronous parity or ECC error on memory access
    SyncParityEccError = 24,
    /// SError parity or ECC error on memory access
    SErrorParityEccError = 25,
    /// Alignment fault
    AlignmentFault = 33,
    /// Debug exception
    Debug = 34,
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
