//! Instruction Cache related operations

use crate::register::{SysReg, SysRegWrite};

/// Flush I Cache
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushICache();

impl SysReg for FlushICache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 0;
}

impl SysRegWrite for FlushICache {}

impl FlushICache {
    /// Flush the I Cache
    pub fn trigger() {
        unsafe {
            <Self as SysRegWrite>::write_raw(0);
        }
    }
}

/// Flush I Single Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushICacheSingleEntry(pub usize);

impl SysReg for FlushICacheSingleEntry {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 1;
}

impl SysRegWrite for FlushICacheSingleEntry {}

impl FlushICacheSingleEntry {
    /// Flush a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}
