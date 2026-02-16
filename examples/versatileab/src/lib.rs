//! Common code for all examples

#![no_std]

/// The base address of our PL190 interrupt controller
pub const PL190_BASE_ADDRESS: usize = 0x1014_0000;

#[cfg(arm_architecture = "v8-r")]
compile_error!("This example/board is not compatible with the ARMv8-R architecture");

static WANT_PANIC: portable_atomic::AtomicBool = portable_atomic::AtomicBool::new(false);

/// Track if we're already in the exit routine.
///
/// Stops us doing infinite recursion if we panic whilst doing the stack reporting.
static IN_EXIT: portable_atomic::AtomicBool = portable_atomic::AtomicBool::new(false);

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
#[cfg(target_os = "none")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::println!("PANIC: {:#?}", info);
    if WANT_PANIC.load(portable_atomic::Ordering::Relaxed) {
        exit(0);
    } else {
        exit(1);
    }
}

/// Set the panic function as no longer returning a failure code via semihosting
pub fn want_panic() {
    WANT_PANIC.store(true, portable_atomic::Ordering::Relaxed);
}

/// Exit from QEMU with code
pub fn exit(code: i32) -> ! {
    if !IN_EXIT.swap(true, portable_atomic::Ordering::Relaxed) {
        stack_dump();
    }
    semihosting::process::exit(code)
}

/// Print stack using to semihosting output for each stack
///
/// Produces output like:
///
/// ```text
/// Stack usage report:
/// SYS Stack =    332 used of  16384 bytes (002%) @ 0x1006bf80..0x1006ff80
/// FIQ Stack =      0 used of     64 bytes (000%) @ 0x1006ff80..0x1006ffc0
/// IRQ Stack =      0 used of     64 bytes (000%) @ 0x1006ffc0..0x10070000
/// ABT Stack =      0 used of  16384 bytes (000%) @ 0x10070000..0x10074000
/// SVC Stack =      0 used of  16384 bytes (000%) @ 0x10074000..0x10078000
/// UND Stack =    244 used of  16384 bytes (001%) @ 0x10078000..0x1007c000
/// HYP Stack =      0 used of  16384 bytes (000%) @ 0x1007c000..0x10080000
/// ```
fn stack_dump() {
    use aarch32_cpu::stacks::stack_used_bytes;
    use core::ptr::addr_of;

    unsafe extern "C" {
        static _sys_stack_end: u32;
        static _sys_stack: u32;
        static _fiq_stack_end: u32;
        static _fiq_stack: u32;
        static _irq_stack_end: u32;
        static _irq_stack: u32;
        static _abt_stack_end: u32;
        static _abt_stack: u32;
        static _svc_stack_end: u32;
        static _svc_stack: u32;
        static _und_stack_end: u32;
        static _und_stack: u32;
        static _hyp_stack_end: u32;
        static _hyp_stack: u32;
    }

    // these are placed in the order they are in aarch32-rt/link.x
    let stacks = [
        ("SYS", addr_of!(_sys_stack_end)..addr_of!(_sys_stack)),
        ("FIQ", addr_of!(_fiq_stack_end)..addr_of!(_fiq_stack)),
        ("IRQ", addr_of!(_irq_stack_end)..addr_of!(_irq_stack)),
        ("ABT", addr_of!(_abt_stack_end)..addr_of!(_abt_stack)),
        ("SVC", addr_of!(_svc_stack_end)..addr_of!(_svc_stack)),
        ("UND", addr_of!(_und_stack_end)..addr_of!(_und_stack)),
        ("HYP", addr_of!(_hyp_stack_end)..addr_of!(_hyp_stack)),
    ];

    semihosting::eprintln!("Stack usage report:");

    unsafe {
        for (name, range) in stacks {
            let (total, used) = stack_used_bytes(range.clone());
            let percent = (used * 100).checked_div(total).unwrap_or(999);
            // Send to stderr, so it doesn't mix with expected output on stdout
            semihosting::eprintln!(
                "{} Stack = {:6} used of {:6} bytes ({:03}%) @ {:08x?}",
                name,
                used,
                total,
                percent,
                range
            );
        }
    }
}
