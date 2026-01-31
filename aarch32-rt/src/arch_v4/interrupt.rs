//! IRQ handler for for Armv4 to Armv6

use crate::{Cpsr, ProcessorMode};

extern "C" {
    fn _irq_handler();
}

/// The default assembly-language handler for an Interrupt
///
/// Called from the vector table when an IRQ occurs.
///
/// First it saves the system stack (SPSR, R0-R3, R12 and the FPU context) and
/// switches to SYS mode. It then calls `extern "C" fn _irq_handler();`
///
/// # Safety
///
/// Only call this function when in IRQ mode handling a IRQ exception.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
pub unsafe extern "C" fn _asm_default_irq_handler() {
    core::arch::naked_asm!(
        "sub     lr, lr, 4                 // make sure we jump back to the right place",
        "push    {{ lr }}                  // save adjusted LR to IRQ stack",
        "mrs     lr, spsr                  // The hardware has copied the interrupted task's CPSR to SPSR_irq - grab it and",
        "push    {{ lr }}                  //   save it to IRQ stack using LR",
        "msr     cpsr_c, {sys_mode}        // switch to system mode so we can handle another interrupt (because if we interrupt irq mode we trash our own shadow registers)",
        "mov     lr, sp                    // align SP down to eight byte boundary using LR",
        "and     lr, lr, 7                 //",
        "sub     sp, lr                    // SP now aligned - only push 64-bit values from here",
        "push    {{ r0-r3, r12, lr }}      // push alignment amount (in LR) and preserved registers",
        crate::save_fpu_context!(),
        "bl      {handler}                 // call Rust handler (they may choose to re-enable interrupts)",
        crate::restore_fpu_context!(),
        "pop     {{ r0-r3, r12, lr }}      // restore alignment amount (in LR) and preserved registers",
        "add     sp, lr                    // restore SP alignment using LR",
        "msr     cpsr_c, {irq_mode}        // switch back to IRQ mode (with IRQ masked)",
        "ldmia   sp!, {{ lr }}             // load and restore SPSR using LR",
        "msr     spsr, lr                  //",
        "ldmfd   sp!, {{ pc }}^            // return from exception",
        sys_mode = const {
            Cpsr::new_with_raw_value(0)
                .with_mode(ProcessorMode::Sys)
                .with_i(true)
                .raw_value()
        },
        handler = sym _irq_handler,
        irq_mode = const {
            Cpsr::new_with_raw_value(0)
                .with_mode(ProcessorMode::Irq)
                .with_i(true)
                .raw_value()
        }
    );
}
