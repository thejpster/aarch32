//! Instruction/Data Cache related operations

use crate::register::{SysReg, SysRegTrigger, SysRegWrite};

/// Clean I/D Cache
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanIDCache();

impl SysReg for CleanIDCache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 0;
}

impl SysRegTrigger for CleanIDCache {}

impl CleanIDCache {
    /// Clean the cache
    pub fn trigger() {
        unsafe {
            <Self as SysRegTrigger>::trigger();
        }
    }
}

/// Clean I/D Cache Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanIDCacheEntry(pub usize);

impl SysReg for CleanIDCacheEntry {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 1;
}

impl SysRegWrite for CleanIDCacheEntry {}

impl CleanIDCacheEntry {
    /// Clean a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}

/// Flush ID Cache
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushIDCache();

impl SysReg for FlushIDCache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 0;
}

impl SysRegTrigger for FlushIDCache {}

impl FlushIDCache {
    /// Flush the cache
    pub fn trigger() {
        unsafe {
            <Self as SysRegTrigger>::trigger();
        }
    }
}

/// Flush ID Cache Single Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushIDCacheSingleEntry(pub usize);

impl SysReg for FlushIDCacheSingleEntry {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 1;
}

impl SysRegWrite for FlushIDCacheSingleEntry {}

impl FlushIDCacheSingleEntry {
    /// Flush a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}

/// Clean and Flush I/D Cache
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanFlushIDCache();

impl SysReg for CleanFlushIDCache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 15;
    const OP2: u32 = 0;
}

impl SysRegTrigger for CleanFlushIDCache {}

impl CleanFlushIDCache {
    /// Clean and flush the cache
    pub fn trigger() {
        unsafe {
            <Self as SysRegTrigger>::trigger();
        }
    }
}

/// Clean and Flush I/D Cache Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanFlushIDCacheEntry(pub usize);

impl SysReg for CleanFlushIDCacheEntry {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 15;
    const OP2: u32 = 1;
}

impl SysRegWrite for CleanFlushIDCacheEntry {}

impl CleanFlushIDCacheEntry {
    /// Clean and flush a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}
