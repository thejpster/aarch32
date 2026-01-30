//! CPU/peripheral support for Arm AArch32
#![no_std]

mod critical_section;

#[cfg(any(
    doc,
    arm_architecture = "v7-a",
    arm_architecture = "v7-r",
    arm_architecture = "v8-r"
))]
#[path = "asmv7.rs"]
pub mod asm;

#[cfg(not(any(
    doc,
    arm_architecture = "v7-a",
    arm_architecture = "v7-r",
    arm_architecture = "v8-r"
)))]
#[path = "asmv4.rs"]
pub mod asm;

pub mod cache;
pub mod interrupt;
pub mod mmu;
pub mod register;

#[cfg(any(test, doc, arm_architecture = "v7-r"))]
pub mod pmsav7;

#[cfg(any(test, doc, arm_architecture = "v8-r"))]
pub mod generic_timer;

#[cfg(any(test, doc, arm_architecture = "v8-r"))]
pub mod pmsav8;

/// Generate an SVC call with the given argument.
///
/// Safe to call even in Supervisor (SupervisorCall) mode, as long as your SVC handler
/// saves and restores SPSR_svc correctly.
#[macro_export]
macro_rules! svc {
    ($num:expr) => { {
        let retval: u32;
        unsafe {
            core::arch::asm!("svc {arg}", arg = const $num, lateout("r0") retval, out("lr") _);
        }
        retval
    } }
}

/// Generate an SVC call with the given argument, plus setup r0 with the given value
///
/// Safe to call even in Supervisor (SupervisorCall) mode, as long as your SVC handler
/// saves and restores SPSR_svc correctly.
#[macro_export]
macro_rules! svc1 {
    ($num:expr, $arg0:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        unsafe {
            core::arch::asm!("svc {arg}", arg = const $num, inout("r0") arg0 => retval, out("lr") _);
        }
        retval
    } }
}

/// Generate an SVC call with the given argument, plus setup r0-r1 with the given values
///
/// Safe to call even in Supervisor (SupervisorCall) mode, as long as your SVC handler
/// saves and restores SPSR_svc correctly.
#[macro_export]
macro_rules! svc2 {
    ($num:expr, $arg0:expr, $arg1:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        unsafe {
            core::arch::asm!("svc {arg}", arg = const $num, inout("r0") arg0 => retval, in("r1") arg1, out("lr") _);
        }
        retval
    } }
}

/// Generate an SVC call with the given argument, plus setup r0-r2 with the given values
///
/// Safe to call even in Supervisor (SupervisorCall) mode, as long as your SVC handler
/// saves and restores SPSR_svc correctly.
#[macro_export]
macro_rules! svc3 {
    ($num:expr, $arg0:expr, $arg1:expr, $arg2:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        let arg2: u32 = $arg2;
        unsafe {
            core::arch::asm!("svc {arg}", arg = const $num, inout("r0") arg0 => retval, in("r1") arg1, in("r2") arg2, out("lr") _);
        }
        retval
    } }
}

/// Generate an SVC call with the given argument, plus setup r0-r3 with the given values
///
/// Safe to call even in Supervisor (SupervisorCall) mode, as long as your SVC handler
/// saves and restores SPSR_svc correctly.
#[macro_export]
macro_rules! svc4 {
    ($num:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr) => { {
        let retval: u32;
        let arg0: u32 = $arg0;
        let arg1: u32 = $arg1;
        let arg2: u32 = $arg2;
        let arg3: u32 = $arg3;
        unsafe {
            core::arch::asm!("svc {arg}", arg = const $num, inout("r0") arg0 => retval, in("r1") arg1, in("r2") arg2, in("r3") arg3, out("lr") _);
        }
        retval
    } }
}
