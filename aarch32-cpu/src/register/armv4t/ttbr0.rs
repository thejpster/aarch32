//! Code for managing TTBR0 (*Translation Table Base Register*)

use arbitrary_int::u18;

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// TTBR0 (*Translation Table Base Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ttbr0 {
    /// Pointer to the currently active first-level translation table
    #[bits(14..=31, rw)]
    addr: u18,
}

impl Ttbr0 {
    const MASK: usize = 0xFFFF_C000;

    /// Get the address
    ///
    /// Returns a full 32-bit memory address.
    pub fn get_address(&self) -> usize {
        (self.raw_value() as usize) & Self::MASK
    }

    /// Set the address
    ///
    /// Pass a full 32-bit memory address with 14-bit alignment.
    ///
    /// Will panic if value is not correctly aligned.
    pub fn set_address(&mut self, address: usize) {
        if (address & Self::MASK) != 0 {
            panic!("Invalid TTBR pointer");
        }
        let addr = u18::from_u32((address >> 14) as u32);
        self.set_addr(addr);
    }

    /// Change the address
    ///
    /// Pass a full 32-bit memory address with 14-bit alignment.
    ///
    /// Will panic if value is not correctly aligned.
    pub fn with_address(self, address: usize) -> Self {
        if (address & Self::MASK) != 0 {
            panic!("Invalid TTBR pointer");
        }
        let addr = u18::from_u32((address >> 14) as u32);
        self.with_addr(addr)
    }
}

impl SysReg for Ttbr0 {
    const CP: u32 = 15;
    const CRN: u32 = 2;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Ttbr0 {}

impl crate::register::SysRegWrite for Ttbr0 {}

impl Ttbr0 {
    #[inline]
    /// Reads TTBR0 (*Translation Table Base Register*)
    pub fn read() -> Ttbr0 {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }

    #[inline]
    /// Writes TTBR0 (*Translation Table Base Register*)
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
