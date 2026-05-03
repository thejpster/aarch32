//! Code for managing FSR (*Fault Status Register*)

use arbitrary_int::u4;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// FSR (*Fault Status Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fsr {
    /// Domain
    #[bits(4..=7, rw)]
    domain: u4,
    /// Status bitfield.
    #[bits([0..=3], rw)]
    status: FsrStatus,
}

/// Fault status register enumeration for FSR
#[bitbybit::bitenum(u4, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FsrStatus {
    /// Terminal Exception
    TerminalException = 0b0010,
    /// Vector Exception
    VectorException = 0b0000,
    /// Alignment fault
    Alignment = 0b0001,
    /// Alignment fault (alt)
    AlignmentAlt = 0b0011,
    /// External Abort on Translation on Section
    ExternalAbortOnTranslationFirstLevel = 0b1100,
    /// External Abort on Translation on Page
    ExternalAbortOnTranslationSecondLevel = 0b1110,
    /// Translation fault on Section
    TranslationFirstLevel = 0b0101,
    /// Translation fault on Page
    TranslationSecondLevel = 0b0111,
    /// Domain fault on Section
    DomainFirstLevel = 0b1001,
    /// Domain fault on Page
    DomainSecondLevel = 0b1011,
    /// Permission fault on Section
    PermissionFirstLevel = 0b1101,
    /// Permission fault on Page
    PermissionSecondLevel = 0b1111,
    /// External Abort on Linefetch on Section
    ExternalAbortOnLineFetchFirstLevel = 0b0100,
    /// External Abort on Linefetch on Page
    ExternalAbortOnLineFetchSecondLevel = 0b0110,
    /// External Abort on Section
    ExternalAbortOnFirstLevel = 0b1000,
    /// External Abort on Page
    ExternalAbortOnSecondLevel = 0b1010,
}

impl SysReg for Fsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Fsr {}

impl Fsr {
    #[inline]
    /// Reads FSR (*Fault Status Register*)
    pub fn read() -> Fsr {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for Fsr {}

impl Fsr {
    #[inline]
    /// Writes FSR (*Fault Status Register*)
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
