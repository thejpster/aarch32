//! Code for managing Access Permissions (Protected Memory on Armv5TE)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CP15 Register 4: Access Permissions bits
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AccessPermissions {
    /// Bits B7 through B0
    #[bits([0..=1], rw)]
    bits: [AccessPermission; 8],
}

/// Access Permissions for an MPU Region
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[bitbybit::bitenum(u2, exhaustive = true)]
pub enum AccessPermission {
    /// No Access
    NoAccess = 0b00,
    /// Read-Write at Privileged Mode, No User Permissions
    RwUserNone = 0b01,
    /// Read-Write at Privileged Mode, Read-Only at User Mode
    RwUserRo = 0b10,
    /// Read-Write at Privileged Mode and User Mode
    Rw = 0b11,
}

impl SysReg for AccessPermissions {
    const CP: u32 = 15;
    const CRN: u32 = 3;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl crate::register::SysRegRead for AccessPermissions {}

impl AccessPermissions {
    #[inline]
    /// Reads Access Permission bits
    pub fn read() -> AccessPermissions {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}

impl crate::register::SysRegWrite for AccessPermissions {}

impl AccessPermissions {
    #[inline]
    /// Writes Access Permission bits
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
