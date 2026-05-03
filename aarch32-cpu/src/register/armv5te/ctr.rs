//! Code for managing CTR (*Cache Type Register*)

use crate::register::{SysReg, SysRegRead};

use arbitrary_int::{u12, u2, u3};

/// CTR (*Cache Type Register*)
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ctr {
    /// Cache Type
    #[bits(25..=28, r)]
    ctype: Option<Ctype>,
    /// Split Cache
    #[bit(24, r)]
    s: bool,
    /// Size of D Cache
    #[bits([12..=23], r)]
    dsize: CacheInfo,
    /// Size of I Cache
    #[bits([0..=11], r)]
    isize: CacheInfo,
}

/// Cache Type
#[bitbybit::bitenum(u4, exhaustive = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctype {
    /// Write-through cache, cache cleaning not needed, cache lock-down not supported
    WriteThrough = 0b0000,
    /// Write-back cache, Read Data Block for cache cleaning, cache lock-down not supported
    ///
    /// Deprecated in Armv6
    WriteBackCleanOnRead = 0b0001,
    /// Write-back cache, Register 7 for cache cleaning, cache lock-down not supported
    ///
    /// Deprecated in Armv6
    WriteBackRegister7 = 0b0010,
    /// Write-back cache, Register 7 for cache cleaning, cache lock-down supported with format A
    WriteBackRegister7LockDownA = 0b0110,
    /// Write-back cache, Register 7 for cache cleaning, cache lock-down supported with format B
    ///
    /// Deprecated in Armv6
    WriteBackRegister7LockDownB = 0b0111,
    /// Write-back cache, Register 7 for cache cleaning, cache lock-down supported with format C
    ///
    /// Added in Armv6
    WriteBackRegister7LockDownC = 0b1110,
    /// Write-back cache, Register 7 for cache cleaning, cache lock-down supported with format D
    ///
    /// Added in Armv6
    WriteBackRegister7LockDownD = 0b0101,
}

/// Cache Size
#[bitbybit::bitfield(u12, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CacheInfo {
    /// Size
    #[bits(6..=8, r)]
    size: u3,
    /// Associativity
    #[bits(3..=5, r)]
    assoc: u3,
    /// Cache size adjusted
    #[bit([2], r)]
    m: bool,
    /// Line length
    #[bits([0..=1], r)]
    len: u2,
}

/// Cache Associativity
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Associativity {
    /// Cache absent
    CacheAbsent,
    /// 1-Way (direct-mapped)
    DirectMapped,
    /// 2-way
    _2Way,
    /// 3-way
    _3Way,
    /// 4-way
    _4Way,
    /// 6-way
    _6Way,
    /// 8-way
    _8Way,
    /// 12-way
    _12Way,
    /// 16-way
    _16Way,
    /// 24-way
    _24Way,
    /// 32-way
    _32Way,
    /// 48-way
    _48Way,
    /// 64-way
    _64Way,
    /// 96-way
    _96Way,
    /// 128-way
    _128Way,
    /// 192-way
    _192Way,
}

impl CacheInfo {
    /// Get the size of the cache, in bytes
    pub const fn size_bytes(self) -> usize {
        let mut output = 2usize.pow(self.size().value() as u32) * 512;
        if self.m() {
            output = output + (output / 2);
        }
        output
    }

    /// Get the cache associativity
    pub const fn associativity(self) -> Associativity {
        if self.m() {
            match self.assoc().value() {
                0 => Associativity::CacheAbsent,
                1 => Associativity::_3Way,
                2 => Associativity::_6Way,
                3 => Associativity::_12Way,
                4 => Associativity::_24Way,
                5 => Associativity::_48Way,
                6 => Associativity::_96Way,
                _ => Associativity::_192Way,
            }
        } else {
            match self.assoc().value() {
                0 => Associativity::DirectMapped,
                1 => Associativity::_2Way,
                2 => Associativity::_4Way,
                3 => Associativity::_8Way,
                4 => Associativity::_16Way,
                5 => Associativity::_32Way,
                6 => Associativity::_64Way,
                _ => Associativity::_128Way,
            }
        }
    }

    /// Get the cache line length
    pub const fn cache_line_length_bytes(self) -> usize {
        match self.len().value() {
            0 => 8,
            1 => 16,
            2 => 32,
            _ => 64,
        }
    }
}

impl SysReg for Ctr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}

impl crate::register::SysRegRead for Ctr {}

impl Ctr {
    #[inline]
    /// Reads CTR (*Cache Type Register*)
    pub fn read() -> Ctr {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }
}
