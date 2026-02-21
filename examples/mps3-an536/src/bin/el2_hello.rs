//! Semihosting hello-world for Arm Cortex-R52 running in EL2 (Hyp Mode)

#![no_std]
#![no_main]

use aarch32_cpu::generic_timer::GenericTimer;
use aarch32_rt::{entry, exception};
use arm_gic::{
    IntId,
    gicv3::{GicCpuInterface, Group, SecureIntGroup},
};
use semihosting::println;

use mps3_an536::HYP_TIMER_PPI;

const SGI_INTID_LO: IntId = IntId::sgi(3);
const SGI_INTID_HI: IntId = IntId::sgi(4);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code at the bottom of this file.
#[entry]
fn main() -> ! {
    let mut board = mps3_an536::Board::new().unwrap();

    // Only interrupts with a higher priority (numerically lower) will be signalled.
    GicCpuInterface::set_priority_mask(0x80);

    let mut el2_timer = unsafe { aarch32_cpu::generic_timer::El2PhysicalTimer::new() };
    println!("Timer Hz = {}", el2_timer.frequency_hz());
    el2_timer.enable(true);
    el2_timer.interrupt_mask(false);
    el2_timer.counter_compare_set(
        el2_timer
            .counter()
            .wrapping_add(el2_timer.frequency_hz() as u64 / 5),
    );

    println!("Configure Timer Interrupt...");
    board
        .gic
        .set_interrupt_priority(HYP_TIMER_PPI, Some(0), 0x31)
        .unwrap();
    board
        .gic
        .set_group(HYP_TIMER_PPI, Some(0), Group::Secure(SecureIntGroup::Group1S))
        .unwrap();
    board
        .gic
        .enable_interrupt(HYP_TIMER_PPI, Some(0), true)
        .unwrap();

    println!("Enabling interrupts...");
    dump_sctlr();
    unsafe {
        aarch32_cpu::interrupt::enable();
    }
    dump_sctlr();

    let x = 1.0f64;
    let y = x * 2.0;
    println!("Hello, this is semihosting! x = {:0.3}, y = {:0.3}", x, y);

    let mut mpu = unsafe { aarch32_cpu::pmsav8::El2Mpu::new() };
    for idx in 0..mpu.num_regions() {
        if let Some(region) = mpu.get_region(idx) {
            println!("Region {}: {:?}", idx, region);
        }
    }

    let mut count: u32 = 0;
    loop {
        aarch32_cpu::asm::wfi();
        println!("Main loop wake up {}", count);
        count = count.wrapping_add(1);

        if count == 10 {
            println!("Timer IRQ test completed OK");
            mps3_an536::exit(0);
        }
    }

    println!("EL2 timer test completed OK");

    mps3_an536::exit(0);
}

fn dump_sctlr() {
    let sctlr = aarch32_cpu::register::Sctlr::read();
    println!("{:?}", sctlr);
}

/// This is our HVC exception handler
#[exception(HypervisorCall)]
fn hvc_handler(hsr: u32, frame: &aarch32_rt::Frame) -> u32 {
    let hsr = aarch32_cpu::register::Hsr::new_with_raw_value(hsr);
    println!("In hvc_handler, with {:08x?}, {:08x?}", hsr, frame);
    return frame.r0;
}

// Provide a custom `_start` function that sets us up in EL2 mode, with a
// stack.
//
// Unlike the default routine, it does not initialise any other stacks, or
// switch to EL1 mode.
core::arch::global_asm!(
    r#"
    // Work around https://github.com/rust-lang/rust/issues/127269
    .fpu vfp3-d16

    .section .text.start
    .global _start
    .type _start, %function
    _start:
        // Set stack pointer
        ldr     sp, =_hyp_stack_high_end
        // Set the HVBAR (for EL2) to _vector_table
        ldr     r1, =_vector_table
        mcr     p15, 4, r1, c12, c0, 0
        // Configure HACTLR to let us enter EL1
        mrc     p15, 4, r1, c1, c0, 1
        mov     r2, {hactlr_bits}
        orr     r1, r1, r2
        mcr     p15, 4, r1, c1, c0, 1
        // Init .data and .bss
        bl      _init_segments
        // Allow VFP coprocessor access
        mrc     p15, 0, r0, c1, c0, 2
        orr     r0, r0, #0xF00000
        mcr     p15, 0, r0, c1, c0, 2
        // Enable VFP
        mov     r0, #0x40000000
        vmsr    fpexc, r0
        // Zero all registers before calling kmain
        mov     r0, 0
        mov     r1, 0
        mov     r2, 0
        mov     r3, 0
        mov     r4, 0
        mov     r5, 0
        mov     r6, 0
        mov     r7, 0
        mov     r8, 0
        mov     r9, 0
        mov     r10, 0
        mov     r11, 0
        mov     r12, 0
        // Jump to application
        bl      kmain
        // In case the application returns, loop forever
        b       .
    .size _start, . - _start
    "#,
);
