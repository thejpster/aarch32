//! Defines various AArch32 system registers
//!
//! These are all ready using Co-Processor read/write instructions

pub mod armv4t;
pub mod armv5te;
pub mod armv6;
pub mod armv7a;
pub mod armv7r;
pub mod armv8r;

pub(crate) mod common;
pub(crate) mod generic_timer;
pub(crate) mod hyp;

#[cfg(arm_architecture = "v4t")]
#[doc(inline)]
pub use armv4t::*;

#[cfg(arm_architecture = "v5te")]
#[doc(inline)]
pub use armv5te::*;

#[cfg(arm_architecture = "v6")]
#[doc(inline)]
pub use armv6::*;

#[cfg(arm_architecture = "v7-r")]
#[doc(inline)]
pub use armv7r::*;

#[cfg(arm_architecture = "v7-a")]
#[doc(inline)]
pub use armv7a::*;

#[cfg(arm_architecture = "v8-r")]
#[doc(inline)]
pub use armv8r::*;

/// Describes a 32-bit System Register
pub trait SysReg {
    /// Which Co-Processor (e.g. 15 for CP15) is this register in?
    const CP: u32;
    /// Which CRn argument (e.g. 0 for c0) accesses this register
    const CRN: u32;
    /// Which OP1 argument accesses this register
    const OP1: u32;
    /// Which CRm argument (e.g. 1 for c1) accesses this register
    const CRM: u32;
    /// Which OP2 argument accesses this register
    const OP2: u32;
}

/// Readable 32-bit System Registers
pub trait SysRegRead: SysReg {
    /// Read a value from this 32-bit register
    ///
    /// Our working assumption is that no Arm system register read has
    /// side-effects that can cause Undefined Behaviour, so this method
    /// is safe.
    #[cfg_attr(not(feature = "check-asm"), inline)]
    #[cfg_attr(
        any(
            arm_architecture = "v4t",
            arm_architecture = "v5te",
            arm_architecture = "v6"
        ),
        instruction_set(arm::a32)
    )]
    fn read_raw() -> u32 {
        let r: u32;
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!(
                "mrc p{cp}, {op1}, {reg}, c{crn}, c{crm}, {op2}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                reg = out(reg) r,
                crn = const Self::CRN,
                crm = const Self::CRM,
                op2 = const Self::OP2,
                options(nomem, nostack, preserves_flags)
            );
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        r
    }
}

/// Writable 32-bit System Registers
pub trait SysRegWrite: SysReg {
    /// Write a value to this 32-bit register
    ///
    /// # Safety
    ///
    /// You need to read the Architecture Reference Manual to verify that you are
    /// writing valid data here.
    #[cfg_attr(not(feature = "check-asm"), inline)]
    #[cfg_attr(
        any(
            arm_architecture = "v4t",
            arm_architecture = "v5te",
            arm_architecture = "v6"
        ),
        instruction_set(arm::a32)
    )]
    unsafe fn write_raw(_value: u32) {
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!(
                "mcr p{cp}, {op1}, {reg}, c{crn}, c{crm}, {op2}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                reg = in(reg) _value,
                crn = const Self::CRN,
                crm = const Self::CRM,
                op2 = const Self::OP2,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}

/// Triggerable 32-bit System Registers
pub trait SysRegTrigger: SysReg {
    /// Trigger an operation by writing zero to this 32-bit register
    ///
    /// # Safety
    ///
    /// You need to read the Architecture Reference Manual to verify that the side-effects
    /// of this action are acceptable.
    #[cfg_attr(not(feature = "check-asm"), inline)]
    #[cfg_attr(
        any(
            arm_architecture = "v4t",
            arm_architecture = "v5te",
            arm_architecture = "v6"
        ),
        instruction_set(arm::a32)
    )]
    unsafe fn trigger() {
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!(
                "mcr p{cp}, {op1}, {reg}, c{crn}, c{crm}, {op2}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                reg = in(reg) 0,
                crn = const Self::CRN,
                crm = const Self::CRM,
                op2 = const Self::OP2,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}

/// Describes a 64-bit System Register
pub trait SysReg64 {
    /// Which Co-Processor (e.g. 15 for CP15) is this register in?
    const CP: u32;
    /// Which OP1 argument accesses this register
    const OP1: u32;
    /// Which CRm argument (e.g. 1 for c1) accesses this register
    const CRM: u32;
}

/// Readable 64-bit System Registers
pub trait SysRegRead64: SysReg64 {
    /// Read a value from this 64-bit register
    ///
    /// Our working assumption is that no Arm system register read has
    /// side-effects that can cause Undefined Behaviour, so this method
    /// is safe.
    #[inline]
    fn read_raw() -> u64 {
        let r_lo: u32;
        let r_hi: u32;
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!(
                "mrrc p{cp}, {op1}, {rt}, {rt2}, c{crm}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                rt = out(reg) r_lo,
                rt2 = out(reg) r_hi,
                crm = const Self::CRM,
                options(nomem, nostack, preserves_flags)
            );
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r_lo = 0;
            r_hi = 0;
        }
        ((r_hi as u64) << 32) | (r_lo as u64)
    }
}

/// Writable 64-bit System Registers
pub trait SysRegWrite64: SysReg64 {
    /// Write a value to this 64-bit register
    ///
    /// # Safety
    ///
    /// You need to read the Architecture Reference Manual to verify that you are
    /// writing valid data here.
    #[inline]
    unsafe fn write_raw(_value: u64) {
        #[cfg(target_arch = "arm")]
        unsafe {
            let r_lo = _value as u32;
            let r_hi = (_value >> 32) as u32;
            core::arch::asm!(
                "mcrr p{cp}, {op1}, {rt}, {rt2}, c{crm}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                rt = in(reg) r_lo,
                rt2 = in(reg) r_hi,
                crm = const Self::CRM,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}
