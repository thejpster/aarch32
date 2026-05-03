//! Code for DrainWriteBuffer

use crate::register::{SysReg, SysRegWrite};

/// DrainWriteBuffer
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DrainWriteBuffer();

impl SysReg for DrainWriteBuffer {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 10;
    const OP2: u32 = 4;
}

impl SysRegWrite for DrainWriteBuffer {}

impl DrainWriteBuffer {
    #[inline]
    /// Flush Prefetch Buffer
    pub fn trigger() {
        unsafe {
            <Self as SysRegWrite>::write_raw(0);
        }
    }
}
