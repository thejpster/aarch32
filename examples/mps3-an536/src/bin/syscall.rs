//! Syscall example
//!
//! * Syscall 0xF0 takes no arguments and returns:    0x1000_0000
//! * Syscall 0xF1 takes one argument and returns:    0x1000_0000 + (arg0)
//! * Syscall 0xF2 takes two arguments and returns:   0x1000_0000 + (arg0) + (arg1 << 4)
//! * Syscall 0xF3 takes three arguments and returns: 0x1000_0000 + (arg0) + (arg1 << 4) + (arg2 << 8)
//! * Syscall 0xF4 takes four arguments and returns:  0x1000_0000 + (arg0) + (arg1 << 4) + (arg2 << 8) + (arg3 << 12)

#![no_std]
#![no_main]

use aarch32_rt::{entry, exception};
use mps3_an536 as _;
use semihosting::println;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    let x = 1;
    let y = x + 1;
    let z = (y as f64) * 1.5;
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);

    let retval = aarch32_cpu::svc4!(0xF4, 0xE000_0001, 0xE000_0002, 0xE000_0003, 0xE000_0004);
    if retval != 0xF000_4321 {
        panic!("Wanted 0xF000_4321, got {:08x}", retval);
    }
    let retval = aarch32_cpu::svc3!(0xF3, 0xE000_0001, 0xE000_0002, 0xE000_0003);
    if retval != 0xF000_0321 {
        panic!("Wanted 0xF000_0321, got {:08x}", retval);
    }
    let retval = aarch32_cpu::svc2!(0xF2, 0xE000_0001, 0xE000_0002);
    if retval != 0xF000_0021 {
        panic!("Wanted 0xF000_0021, got {:08x}", retval);
    }
    let retval = aarch32_cpu::svc1!(0xF1, 0xE000_0001);
    if retval != 0xF000_0001 {
        panic!("Wanted 0xF000_0001, got {:08x}", retval);
    }
    let retval = aarch32_cpu::svc!(0xF0);
    if retval != 0x1000_0000 {
        panic!("Wanted 0x1000_0000, got {:08x}", retval);
    }
    println!("Syscalls all look OK, printing local variables...");
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);
    semihosting::process::exit(0);
}

/// This is our syscall handler
#[exception(SupervisorCall)]
fn svc_handler(arg: u32, frame: &aarch32_rt::Frame) -> u32 {
    match arg {
        0xF0 => 0x1000_0000,
        0xF1 => 0x1000_0000 + frame.r0,
        0xF2 => 0x1000_0000 + frame.r0 + (frame.r1 << 4),
        0xF3 => 0x1000_0000 + frame.r0 + (frame.r1 << 4) + (frame.r2 << 8),
        0xF4 => 0x1000_0000 + frame.r0 + (frame.r1 << 4) + (frame.r2 << 8) + (frame.r3 << 12),
        _ => 0xDEADC0DE,
    }
}
