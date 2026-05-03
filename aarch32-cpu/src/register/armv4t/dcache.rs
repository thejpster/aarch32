//! Data Cache related operations

use crate::register::{SysReg, SysRegWrite};

/// Clean Data Cache
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanDCache();

impl SysReg for CleanDCache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 10;
    const OP2: u32 = 0;
}

impl SysRegWrite for CleanDCache {}

impl CleanDCache {
    /// Clean the cache
    pub fn trigger() {
        unsafe {
            <Self as SysRegWrite>::write_raw(0);
        }
    }
}

/// Clean Data Cache Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanDCacheEntry(pub usize);

impl SysReg for CleanDCacheEntry {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 10;
    const OP2: u32 = 1;
}

impl SysRegWrite for CleanDCacheEntry {}

impl CleanDCacheEntry {
    /// Clean a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}

/// Flush D Cache
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushDCache();

impl SysReg for FlushDCache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 0;
}

impl SysRegWrite for FlushDCache {}

impl FlushDCache {
    /// Flush the cache
    pub fn trigger() {
        unsafe {
            <Self as SysRegWrite>::write_raw(0);
        }
    }
}

/// Flush D Cache Single Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushDCacheSingleEntry(pub usize);

impl SysReg for FlushDCacheSingleEntry {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 1;
}

impl SysRegWrite for FlushDCacheSingleEntry {}

impl FlushDCacheSingleEntry {
    /// Flush a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}

/// Clean and Flush D Cache
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanFlushDCache();

impl SysReg for CleanFlushDCache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 0;
}

impl SysRegWrite for CleanFlushDCache {}

impl CleanFlushDCache {
    /// Clean and flush the Data Cache
    pub fn trigger() {
        unsafe {
            <Self as SysRegWrite>::write_raw(0);
        }
    }
}

/// Clean and Flush D Cache Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanFlushDCacheEntry(pub usize);

impl SysReg for CleanFlushDCacheEntry {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 1;
}

impl SysRegWrite for CleanFlushDCacheEntry {}

impl CleanFlushDCacheEntry {
    /// Clean and Flush a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}
