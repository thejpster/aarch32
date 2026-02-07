//! Syscall example
//!
//! * Syscall 0xF0 takes no arguments and returns:    0x1000_0000
//! * Syscall 0xF1 takes one argument and returns:    0x1000_0000 + (arg0)
//! * Syscall 0xF2 takes two arguments and returns:   0x1000_0000 + (arg0) + (arg1 << 4)
//! * Syscall 0xF3 takes three arguments and returns: 0x1000_0000 + (arg0) + (arg1 << 4) + (arg2 << 8)
//! * Syscall 0xF4 takes four arguments and returns:  0x1000_0000 + (arg0) + (arg1 << 4) + (arg2 << 8) + (arg3 << 12)
//! * Syscall 0xF5 takes five arguments and returns:  0x1000_0000 + (arg0) + (arg1 << 4) + (arg2 << 8) + (arg3 << 12) + (arg4 << 16)
//! * Syscall 0xF6 takes six arguments and returns:   0x1000_0000 + (arg0) + (arg1 << 4) + (arg2 << 8) + (arg3 << 12) + (arg4 << 16) + (arg5 << 20)

#![no_std]
#![no_main]

use aarch32_rt::{entry, exception};
use semihosting::println;
use versatileab as _;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    let x = 1;
    let y = x + 1;
    let z = (y as f64) * 1.5;
    println!("x = {}, y = {}, z = {:0.3}", x, y, z);

    let retval = aarch32_cpu::svc6!(
        0xF6,
        0xE000_0001,
        0xE000_0002,
        0xE000_0003,
        0xE000_0004,
        0xE000_0005,
        0xE000_0006
    );
    if retval != 0xF065_4321 {
        panic!("Wanted 0xF065_4321, got {:08x}", retval);
    }
    let retval = aarch32_cpu::svc5!(
        0xF5,
        0xE000_0001,
        0xE000_0002,
        0xE000_0003,
        0xE000_0004,
        0xE000_0005
    );
    if retval != 0xF005_4321 {
        panic!("Wanted 0xF005_4321, got {:08x}", retval);
    }
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
///
/// We mix together the arguments given in `frame` according to the rules at the
/// top of the file. The arguments are designed to be easy to see in a debugger
/// and the output is designed to be easy to check.
#[exception(SupervisorCall)]
fn svc_handler(arg: u32, frame: &aarch32_rt::Frame) -> u32 {
    // println!("Frame: {:08x?}", frame);
    if arg & 0xF8 != 0xF0 {
        return 0xDEAD_C0DE;
    }
    let mut output = 0x1000_0000;
    if arg >= 0xF6 {
        output += frame.r5 << 20;
    }
    if arg >= 0xF5 {
        output += frame.r4 << 16;
    }
    if arg >= 0xF4 {
        output += frame.r3 << 12;
    }
    if arg >= 0xF3 {
        output += frame.r2 << 8;
    }
    if arg >= 0xF2 {
        output += frame.r1 << 4;
    }
    if arg >= 0xF1 {
        output += frame.r0;
    }
    output
}
