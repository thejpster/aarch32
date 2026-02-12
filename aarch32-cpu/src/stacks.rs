//! Code for checking stack usage

/// Reports stack usage as a count of bytes
///
/// It starts at the lower bound, and looks for values that are set to 0,
/// concluding that those values have never been used. It returns `(total,
/// used)` in bytes.
///
/// # Safety
///
/// Pass a range of valid, readable, memory with 32-bit aligned addresses.
pub unsafe fn stack_used_bytes(stack: core::ops::Range<*const u32>) -> (usize, usize) {
    let start = stack.start;
    let end: *const u32 = stack.end;
    let size_words = unsafe { end.offset_from_unsigned(start) };
    let free_words = unsafe { stack_used_bytes_asm(start, size_words) };
    let used_words = size_words - free_words;
    (
        size_words * core::mem::size_of::<u32>(),
        used_words * core::mem::size_of::<u32>(),
    )
}

/// Counts number of words that are equal to zero
///
/// Written in Arm assembly to avoid any issues with pointing at things that are
/// not validly initialised integers (as far as Rust is concerned).
///
/// Returns a count of the number of words equal to 0x0 at `start`, with a
/// maximum of `size` words
///
/// # Safety
///
/// The address `start` must be 32-bit aligned, and point to a region of memory
/// of at least `size` words in length.
unsafe fn stack_used_bytes_asm(start: *const u32, size: usize) -> usize {
    let result: usize;
    core::arch::asm!(
        r#"
        // skip out if size is zero
        movs    {result}, #0
        cmp     {size}, #0
        beq     3f
2:      // loop
        ldr     {scratch}, [{start}]
        cmp     {scratch}, #0
        // break out if value is non-zero
        bne     3f
        // otherwise increment counter
        adds    {result}, {result}, #1
        adds    {start}, {start}, #4
        // loop if not finished yet
        cmp     {result}, {size}
        bne     2b
        // all finished
3:
        "#,
        size = in(reg) size,
        start = in(reg) start,
        result = out(reg) result,
        scratch = out(reg) _,
    );
    result
}
