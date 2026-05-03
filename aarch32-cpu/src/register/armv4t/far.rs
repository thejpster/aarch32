//! Code for managing FAR (*Fault Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// FAR (*Fault Address Register*)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Far(pub usize);

impl SysReg for Far {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Far {}

impl Far {
    #[inline]
    /// Reads FAR (*Fault Address Register*)
    pub fn read() -> Self {
        Self(<Self as SysRegRead>::read_raw() as usize)
    }
}

impl crate::register::SysRegWrite for Far {}

impl Far {
    #[inline]
    /// Writes FAR (*Fault Address Register*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0 as u32);
        }
    }
}
