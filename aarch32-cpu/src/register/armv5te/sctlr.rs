//! Code for managing SCTLR (*Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// SCTLR (*System Control Register*)
#[bitbybit::bitfield(u32, forbid_overlaps)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sctlr {
    /// Backwards compatible mode
    #[bit(15, rw)]
    l4: bool,
    /// Predictable (round-robin) cache replacement strategy
    #[bit(14, rw)]
    rr: bool,
    /// High exception vectors selected
    ///
    /// Vectors are at `0xFFFF_0000` instead of `0x0000_0000`.
    #[bit(13, rw)]
    v: bool,
    /// Instruction cache enabled
    #[bit(12, rw)]
    i: bool,
    /// Implementation defined
    #[bit(11, rw)]
    z: bool,
    /// Implementation defined
    #[bit(10, rw)]
    f: bool,
    /// ROM protection
    #[bit(9, rw)]
    r: bool,
    /// System protection
    #[bit(8, rw)]
    s: bool,
    /// Big endian operation
    #[bit(7, rw)]
    b: bool,
    /// 32-bit data address range
    #[bit(5, rw)]
    d: bool,
    /// 32-bit exception handlers
    #[bit(4, rw)]
    p: bool,
    /// Write buffer enabled
    #[bit(3, rw)]
    w: bool,
    /// Cache enabled
    #[bit(2, rw)]
    c: bool,
    /// Alignment check enabled
    #[bit(1, rw)]
    a: bool,
    /// MMU Enabled
    #[bit(0, rw)]
    m: bool,
}

impl SysReg for Sctlr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}

impl SysRegRead for Sctlr {}

impl SysRegWrite for Sctlr {}

impl Sctlr {
    /// Read SCTLR (*System Control Register*)
    #[inline]
    pub fn read() -> Self {
        Self::new_with_raw_value(<Self as SysRegRead>::read_raw())
    }

    /// Write SCTLR (*System Control Register*)
    #[inline]
    pub fn write(_value: Self) {
        // Safety: Writing this register is atomic
        unsafe {
            <Self as SysRegWrite>::write_raw(_value.raw_value());
        }
    }

    /// Modify SCTLR (*System Control Register*)
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

impl core::fmt::Debug for Sctlr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "SCTLR {{ L4={} RR={} V={} I={} Z={} F={} R={} S={} B={} D={} P={} W={} C={} A={} M={} }}",
            self.l4() as u8,
            self.rr() as u8,
            self.v() as u8,
            self.i() as u8,
            self.z() as u8,
            self.f() as u8,
            self.r() as u8,
            self.s() as u8,
            self.b() as u8,
            self.d() as u8,
            self.p() as u8,
            self.w() as u8,
            self.c() as u8,
            self.a() as u8,
            self.m() as u8,
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Sctlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SCTLR {{ L4={0=15..16} RR={0=14..15} V={0=13..14} I={0=12..13} Z={0=11..12} F={0=10..11} R={0=9..10} S={0=8..9} B={0=7..8} D={0=5..6} P={0=4..5} W={0=3..4} C={0=2..3} A={0=1..2} M={0=0..1} }}", self.raw_value())
    }
}
