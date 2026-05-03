//! Translation Lookaside Buffer related operations

use crate::register::{SysReg, SysRegTrigger, SysRegWrite};

/// Flush D TLB
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushDTLB();

impl SysReg for FlushDTLB {
    const CP: u32 = 15;
    const CRN: u32 = 8;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 0;
}

impl SysRegTrigger for FlushDTLB {}

impl FlushDTLB {
    /// Flush the D TLB
    pub fn trigger() {
        unsafe {
            <Self as SysRegTrigger>::trigger();
        }
    }
}

/// Flush D TLB Single Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushDTLBSingleEntry(pub usize);

impl SysReg for FlushDTLBSingleEntry {
    const CP: u32 = 15;
    const CRN: u32 = 8;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 1;
}

impl SysRegWrite for FlushDTLBSingleEntry {}

impl FlushDTLBSingleEntry {
    /// Flush a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}

/// Flush ID TLB
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushIDTLB();

impl SysReg for FlushIDTLB {
    const CP: u32 = 15;
    const CRN: u32 = 8;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 0;
}

impl SysRegTrigger for FlushIDTLB {}

impl FlushIDTLB {
    /// Flush the ID TLB
    pub fn trigger() {
        unsafe {
            <Self as SysRegTrigger>::trigger();
        }
    }
}

/// Flush ID TLB Single Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushIDTLBSingleEntry(pub usize);

impl SysReg for FlushIDTLBSingleEntry {
    const CP: u32 = 15;
    const CRN: u32 = 8;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 1;
}

impl SysRegWrite for FlushIDTLBSingleEntry {}

impl FlushIDTLBSingleEntry {
    /// Flush a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}

/// Flush I TLB
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushITLB();

impl SysReg for FlushITLB {
    const CP: u32 = 15;
    const CRN: u32 = 8;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 0;
}

impl SysRegTrigger for FlushITLB {}

impl FlushITLB {
    /// Flush the I TLB
    pub fn trigger() {
        unsafe {
            <Self as SysRegTrigger>::trigger();
        }
    }
}

/// Flush I TLB Single Entry
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushITLBSingleEntry(pub usize);

impl SysReg for FlushITLBSingleEntry {
    const CP: u32 = 15;
    const CRN: u32 = 8;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 1;
}

impl SysRegWrite for FlushITLBSingleEntry {}

impl FlushITLBSingleEntry {
    /// Flush a single entry
    pub fn trigger(value: Self) {
        unsafe {
            Self::write_raw(value.0 as u32);
        }
    }
}
