//! Example triggering a prefetch abort exception.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use aarch32_cpu::register::{Ifar, Ifsr};
use aarch32_rt::{entry, exception};
use mps3_an536 as _;
use semihosting::println;

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    println!("Hello, this is a prefetch abort exception example");

    // A BKPT instruction triggers a Prefetch Abort except when Halting debug-mode is enabled.
    // See p. 2038 of ARMv7-M Architecture Reference Manual

    // trigger an prefetch abort exception, from A32 (Arm) mode
    bkpt_from_a32();

    println!("Recovered from fault OK!");

    semihosting::process::exit(0);
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
extern "C" fn bkpt_from_a32() {
    core::arch::naked_asm!("bkpt    #0", "bx      lr",);
}

#[exception(Undefined)]
fn undefined_handler(_addr: usize) -> ! {
    panic!("unexpected undefined exception");
}

#[exception(PrefetchAbort)]
unsafe fn prefetch_abort_handler(addr: usize) -> usize {
    println!("prefetch abort occurred");
    let ifsr = Ifsr::read();
    println!("IFSR (Fault Status Register): {:?}", ifsr);
    println!("IFSR Status: {:?}", ifsr.status());
    let ifar = Ifar::read();
    println!("IFAR (Faulting Address Register): {:?}", ifar);

    let expect_fault_at = bkpt_from_a32 as extern "C" fn() as usize;

    if addr == expect_fault_at {
        println!("caught bkpt_from_a32");
    } else {
        panic!(
            "Bad fault address {:08x} is not {:08x}",
            addr, expect_fault_at
        );
    }

    match COUNTER.fetch_add(1, Ordering::Relaxed) {
        0 => {
            // first time, huh?
            // go back and do it again
            println!("Doing it again");
            addr
        }
        1 => {
            // second time, huh?
            // go back but skip the instruction
            println!("Skipping instruction");
            addr + 4
        }
        _ => {
            // we've faulted thrice - time to quit
            panic!("prefetch_handler called too often");
        }
    }
}

#[exception(DataAbort)]
fn data_abort_handler(_addr: usize) -> ! {
    panic!("unexpected data abort exception");
}
