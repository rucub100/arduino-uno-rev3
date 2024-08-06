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

pub(super) fn free<F>(handler: F)
where
    F: FnOnce(),
{
    disable_global_interrupts();
    handler();
    enable_global_interrupts();
}
