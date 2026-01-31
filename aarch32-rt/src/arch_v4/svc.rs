//! SVC handler for Armv4 to Armv6

extern "C" {
    fn _svc_handler(nr: u32, frame: &crate::Frame) -> u32;
}

/// The default assembly-language handler for an SVCall
///
/// Called from the vector table when an SVCall exeception occurs.
///
/// First it saves the system stack (SPSR, R0-R3, R12 and the FPU context). It
/// then calls `extern "C" fn _svc_handler(nr: u32, frame: &aarch32_rt::Frame)
/// -> u32 ` with the code from the `SVC` instruction, and a pointer to the
/// stacked registers R0-R3. The function can return a value which will be left
/// in `R0`.
///
/// # Safety
///
/// Only call this function when in SVC mode handling a SVCall exception.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
pub unsafe extern "C" fn _asm_default_svc_handler() {
    core::arch::naked_asm!(
        "push    {{ r12, lr }}             // save LR and R12 - can now use R12 (but leave LR alone for SVC code lookup)",
        "mrs     r12, spsr                 // grab SPSR using R12",
        "push    {{ r12 }}                 // save SPSR value",
        "mov     r12, sp                   // align SP down to eight byte boundary using R12",
        "and     r12, r12, 7               //",
        "sub     sp, r12                   // SP now aligned - only push 64-bit values from here",
        "push    {{ r0-r4, r12 }}          // push alignment amount, and preserved registers - can now use R0-R3 (R4 is just padding)",
        "mov     r12, sp                   // save SP for integer frame",
        crate::save_fpu_context!(),
        "mrs     r0, spsr                  // Load processor status that was banked on entry",
        "tst     r0, {t_bit}               // SVC occurred from Thumb state?",
        "beq     1f",
        "ldrh    r0, [lr,#-2]              // Yes: Load halfword and...",
        "bic     r0, r0, #0xFF00           // ...r0 now contains SVC number",
        "b       2f",
        "1:",
        "ldr     r0, [lr,#-4]              // No: Load word and...",
        "bic     r0, r0, #0xFF000000       // ...r0 now contains SVC number",
        "2:",
        "mov     r1, r12                   // pass the stacked integer registers in r1",
        "bl      {handler}                 // Call Rust handler",
        "mov     lr, r0                    // move r0 out of the way - restore_fpu_context will trash it",
        crate::restore_fpu_context!(),
        "pop     {{ r0-r4, r12 }}          // restore preserved registers, dummy value, and alignment amount",
        "mov     r0, lr                    // replace R0 with return value from Rust handler",
        "add     sp, r12                   // restore SP alignment using R12",
        "pop     {{ lr }}                  // restore SPSR using LR",
        "msr     spsr_cxsf, lr             //",
        "ldmfd   sp!, {{ r12, pc }}^       // restore R12 and return from exception",
        t_bit = const { crate::Cpsr::new_with_raw_value(0).with_t(true).raw_value() },
        handler = sym _svc_handler,
    );
}
