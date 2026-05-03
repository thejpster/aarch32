//! Code for managing Protection Areas (Protected Memory on Armv5TE)

use arbitrary_int::u20;

/// CP15 Register 6: Protection area control
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pac {
    /// Region Base Address
    #[bits([12..=31], rw)]
    base_address: u20,
    /// Region size (and alignment)
    #[bits([1..=5], rw)]
    size: Option<PacSize>,
    /// Region enabled
    #[bit(0, rw)]
    enabled: bool,
}

/// Size of an MPU Region
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[bitbybit::bitenum(u5, exhaustive = false)]
pub enum PacSize {
    /// 4 KiB region
    _4K = 11,
    /// 8 KiB region
    _8K,
    /// 16 KiB region
    _16K,
    /// 32 KiB region
    _32K,
    /// 64 KiB region
    _64K,
    /// 128 KiB region
    _128K,
    /// 256 KiB region
    _256K,
    /// 512 KiB region
    _512K,
    /// 1 MiB region
    _1M,
    /// 2 MiB region
    _2M,
    /// 4 MiB region
    _4M,
    /// 8 MiB region
    _8M,
    /// 16 MiB region
    _16M,
    /// 32 MiB region
    _32M,
    /// 64 MiB region
    _64M,
    /// 128 MiB region
    _128M,
    /// 256 MiB region
    _256M,
    /// 512 MiB region
    _512M,
    /// 1 GiB region
    _1G,
    /// 2 GiB region
    _2G,
    /// 4 GiB region
    _4G,
}
