//! Watchdog
//! Datasheet page 43 et seqq.

use core::arch::asm;

use crate::atmega328p::registers::{WDCE, WDE, WDTCSR_ADDR};

use super::{
    common::{clear_and_set_mask, clear_bit, clear_register, set_mask},
    interrupts,
    registers::{MCUSR_ADDR, WDIE, WDRF},
};

pub enum WatchdogMode {
    Stopped,
    Interrupt,
    Reset,
    InterruptAndReset,
}

pub enum WatchdogTimeout {
    Ms16,
    Ms32,
    Ms64,
    Ms125,
    Ms250,
    Ms500,
    S1,
    S2,
    S4,
    S8,
}

#[allow(dead_code)]
#[inline(always)]
unsafe fn watchdog_reset() {
    asm!("wdr");
}

#[allow(dead_code)]
#[inline(always)]
unsafe fn watchdog_change_enable() {
    watchdog_reset();
    clear_bit(MCUSR_ADDR, WDRF);
    set_mask(WDTCSR_ADDR, (1 << WDCE) | (1 << WDE));
}

#[allow(dead_code)]
#[inline(always)]
pub fn configure_watchdog_timer(timeout: WatchdogTimeout) {
    unsafe {
        asm!(
            "cli", // disable global interrupts
            "wdr", // reset watchdog timer
            "lds r16, {wdtcsr}", // load WDTCSR into r16
            "ori r16, {wdce}", // set WDCE and WDE
            "sts {wdtcsr}, r16", // timed sequence
            "mov r16, {timeout}", // load timeout into r16
            "sts {wdtcsr}, r16", // set timeout
            "sei",  // enable global interrupts
            wdtcsr = const WDTCSR_ADDR,
            wdce = const (1 << WDCE) | (1 << WDE),
            timeout = in(reg) timeout as u8,
            out("r16") _,
        );
    }
}

#[allow(dead_code)]
pub fn configure_watchdog_mode(mode: WatchdogMode) {
    unsafe {
        match mode {
            WatchdogMode::Stopped => {
                interrupts::free(|| {
                    watchdog_change_enable();
                    clear_register(WDTCSR_ADDR);
                });
            }
            WatchdogMode::Interrupt => {
                interrupts::free(|| {
                    watchdog_change_enable();
                    clear_and_set_mask(WDTCSR_ADDR, 1 << WDE, 1 << WDIE);
                });
            }
            WatchdogMode::Reset => {
                interrupts::free(|| {
                    watchdog_change_enable();
                    clear_and_set_mask(WDTCSR_ADDR, 1 << WDIE, 1 << WDE);
                });
            }
            WatchdogMode::InterruptAndReset => {
                interrupts::free(|| {
                    watchdog_change_enable();
                    set_mask(WDTCSR_ADDR, (1 << WDIE) | (1 << WDE));
                });
            }
        }
    }
}
