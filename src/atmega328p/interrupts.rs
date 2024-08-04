//! Interrupts
//! Datasheet page 49 et seqq.

use core::arch::asm;

#[inline(always)]
pub(super) fn enable_global_interrupts() {
    unsafe {
        asm!("sei");
    }
}

#[inline(always)]
pub(super) fn disable_global_interrupts() {
    unsafe {
        asm!("cli");
    }
}
