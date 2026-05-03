//! Code for FPFB (*Flush Prefetch Buffer*)

use crate::register::{SysReg, SysRegWrite};

/// Flush Prefetch Buffer
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlushPrefetchBuffer();

impl SysReg for FlushPrefetchBuffer {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 0b0101;
    const OP2: u32 = 0b100;
}

impl crate::register::SysRegWrite for FlushPrefetchBuffer {}

impl FlushPrefetchBuffer {
    #[inline]
    /// Flush Prefetch Buffer
    pub fn trigger() {
        unsafe {
            <Self as SysRegWrite>::write_raw(0);
        }
    }
}
