//! Power Management and Sleep Modes
//! Datasheet page 34 et seqq.

use core::{arch::asm, ptr::write_volatile};

use super::registers::SMCR_ADDR;

pub enum SleepMode {
    Idle,
    ADCNoiseReduction,
    PowerDown,
    PowerSave,
    /// this mode is only recommended for use with external crystal
    StandBy,
    /// this mode is only recommended for use with external crystal
    ExtendedStandBy,
}

impl SleepMode {
    fn control_register_value(&self) -> u8 {
        match self {
            SleepMode::Idle => 0b0001,
            SleepMode::ADCNoiseReduction => 0b0011,
            SleepMode::PowerDown => 0b0101,
            SleepMode::PowerSave => 0b0111,
            SleepMode::StandBy => 0b1101,
            SleepMode::ExtendedStandBy => 0b1111,
        }
    }
}

pub fn sleep_mode(mode: SleepMode) {
    unsafe {
        write_volatile(SMCR_ADDR as *mut u8, mode.control_register_value());
        asm!("sleep");
    }
}

#[inline(always)]
fn disable_brown_out_detector() {
    // TODO - implement
    // page 38 bod sleep only to be used with sleep modes
}
