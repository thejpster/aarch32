//! Code for managing DACR (*Domain Access Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DACR (*Domain Access Control Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dacr {
    #[bits(30..=31, rw)]
    d15: DomainAccess,
    #[bits(28..=29, rw)]
    d14: DomainAccess,
    #[bits(26..=27, rw)]
    d13: DomainAccess,
    #[bits(24..=25, rw)]
    d12: DomainAccess,
    #[bits(22..=23, rw)]
    d11: DomainAccess,
    #[bits(20..=21, rw)]
    d10: DomainAccess,
    #[bits(18..=19, rw)]
    d9: DomainAccess,
    #[bits(16..=17, rw)]
    d8: DomainAccess,
    #[bits(14..=15, rw)]
    d7: DomainAccess,
    #[bits(12..=13, rw)]
    d6: DomainAccess,
    #[bits(10..=11, rw)]
    d5: DomainAccess,
    #[bits(8..=9, rw)]
    d4: DomainAccess,
    #[bits(6..=7, rw)]
    d3: DomainAccess,
    #[bits(4..=5, rw)]
    d2: DomainAccess,
    #[bits(2..=3, rw)]
    d1: DomainAccess,
    #[bits(0..=1, rw)]
    d0: DomainAccess,
}

/// Domain Access Permissions
#[derive(Debug)]
#[bitbybit::bitenum(u2, exhaustive = true)]
pub enum DomainAccess {
    /// No access. Any access to the domain generates a Domain fault.
    NoAccess = 0b00,
    /// Cient. Accesses are checked against the permission bits in the translation tables.
    Client = 0b01,
    /// Reserved
    Reserved = 0b10,
    /// Manager. Accesses are not checked against the permission bits in the translation tables.
    Manager = 0b11,
}

impl SysReg for Dacr {
    const CP: u32 = 15;
    const CRN: u32 = 3;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for Dacr {}

impl Dacr {
    #[inline]
    /// Reads DACR (*Domain Access Control Register*)
    pub fn read() -> Dacr {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Dacr {}

impl Dacr {
    #[inline]
    /// Writes DACR (*Domain Access Control Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }

    /// Modify DACR (*Domain Access Control Register*)
    #[inline]
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }
}
