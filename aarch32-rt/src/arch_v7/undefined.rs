//! Undefined handler for Armv7 and higher

extern "C" {
    fn _undefined_handler(addr: usize) -> usize;
}

/// The default assembly-language handler for an Undefined exception.
///
/// Called from the vector table when an Undefined exeception occurs.
///
/// First it saves the system stack (SPSR, R0-R3, R12 and the FPU context). It
/// then calls `fn _undefined_handler(addr: usize) -> usize` with the address of
/// the faulting instruction. That function can return a new address from where
/// execution should start, or it can choose not to return.
///
/// # Safety
///
/// Only call this function when in UND mode handling a Undefined exception.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
pub unsafe extern "C" fn _asm_default_undefined_handler() {
    core::arch::naked_asm!(
        "srsfd   sp!, #{und_mode}          // store return state to UND stack",
        "push    {{ r12 }}                 // Save preserved register R12 - can now use it",
        "mrs     r12, spsr                 // Read SPSR into R12",
        "tst     r12, {t_bit}              // Was the code that triggered the exception in Thumb state?",
        "ite     eq                        // Adjust LR to point to faulting instruction - see p.1206 of the ARMv7-A architecture manual.",
        "subeq   lr, lr, #4                // Subtract 4 in Arm Mode",
        "subne   lr, lr, #2                // Subtract 2 in Thumb Mode",
        "mov     r12, sp                   // align SP down to eight byte boundary using R12",
        "and     r12, r12, 7               //",
        "sub     sp, r12                   // SP now aligned - only push 64-bit values from here",
        "push    {{ r0-r4, r12 }}          // push alignment amount, and preserved registers - can now use R0-R3 (R4 is just padding)",
        crate::save_fpu_context!(),
        "mov     r0, lr                    // Pass the faulting instruction address to the handler.",
        "bl      {handler}                 // call Rust handler",
        "mov     lr, r0                    // if we get back here, assume they returned a new LR in r0",
        crate::restore_fpu_context!(),
        "pop     {{ r0-r4, r12 }}          // restore preserved registers, dummy value, and alignment amount",
        "add     sp, r12                   // restore SP alignment using R12",
        "pop     {{ r12 }}                 // restore R12       ",
        "str     lr, [sp]                  // overwrite the saved LR with the one from the Rust handler",
        "rfefd   sp!                       // return from exception",
        und_mode = const crate::ProcessorMode::Und as u8,
        t_bit = const { crate::Cpsr::new_with_raw_value(0).with_t(true).raw_value() },
        handler = sym _undefined_handler,
    );
}
