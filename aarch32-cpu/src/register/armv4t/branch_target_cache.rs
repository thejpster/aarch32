//! Branch Target Cache related operations
//!

use crate::register::{SysReg, SysRegWrite};

/// Flush Branch Target Cache
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushBranchTargetCache();

impl SysReg for FlushBranchTargetCache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 6;
}

impl SysRegWrite for FlushBranchTargetCache {}

impl FlushBranchTargetCache {
    /// Clean the cache
    pub fn trigger() {
        unsafe {
            <Self as SysRegWrite>::write_raw(0);
        }
    }
}

/// Flush Branch Target Cache Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushBranchTargetCacheEntry(pub usize);

impl SysReg for FlushBranchTargetCacheEntry {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 7;
}

impl SysRegWrite for FlushBranchTargetCacheEntry {}

impl FlushBranchTargetCacheEntry {
    /// Flush a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}
