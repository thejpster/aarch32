//! Data and Prefetch Abort handlers for Armv4 to Armv6

extern "C" {
    fn _data_abort_handler(addr: usize) -> usize;
    fn _prefetch_abort_handler(addr: usize) -> usize;
}

/// The default assembly-language handler for a Data Abort
///
/// Called from the vector table when an Data Abort occurs.
///
/// First it saves the system stack (SPSR, R0-R3, R12 and the FPU context). It
/// then calls `fn _data_abort_handler(addr: usize) -> usize` with the address
/// of the faulting instruction. That function can return a new address from
/// where execution should start, or it can choose not to return.
///
/// # Safety
///
/// Only call this function when in ABT mode handling a Data Abort exception.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
pub unsafe extern "C" fn _asm_default_data_abort_handler() {
    core::arch::naked_asm!(
        "sub     lr, lr, #8                // Subtract 8 from LR, see p.1214 of the ARMv7-A architecture manual.",
        "push    {{ r12 }}                 // Save preserved register R12 - can now use it",
        "mrs     r12, spsr                 // grab SPSR",
        "push    {{ r12 }}                 // save SPSR value",
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
        "pop     {{ r12 }}                 // restore SPSR using R12",
        "msr     spsr_cxsf, r12            //",
        "pop     {{ r12 }}                 // restore R12",
        "movs    pc, lr                    // return from exception",
        handler = sym _data_abort_handler,
    );
}

/// The default assembly-language handler for a Prefetch Abort
///
/// Called from the vector table when an Prefetch Abort occurs.
///
/// First it saves the system stack (SPSR, R0-R3, R12 and the FPU context). It
/// then calls `fn _prefetch_abort_handler(addr: usize) -> usize` with the address
/// of the faulting instruction. That function can return a new address from
/// where execution should start, or it can choose not to return.
///
/// # Safety
///
/// Only call this function when in ABT mode handling a Prefetch Abort exception.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[instruction_set(arm::a32)]
pub unsafe extern "C" fn _asm_default_prefetch_abort_handler() {
    core::arch::naked_asm!(
        "sub     lr, lr, #4                // Subtract 4 from LR, see p.1212 of the ARMv7-A architecture manual.",
        "push    {{ r12 }}                 // Save preserved register R12 - can now use it",
        "mrs     r12, spsr                 // grab SPSR",
        "push    {{ r12 }}                 // save SPSR value",
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
        "pop     {{ r12 }}                 // restore SPSR using R12",
        "msr     spsr_cxsf, r12            //",
        "pop     {{ r12 }}                 // restore R12",
        "movs    pc, lr                    // return from exception",
        handler = sym _prefetch_abort_handler,
    );
}
