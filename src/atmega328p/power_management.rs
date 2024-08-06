//! Power Management and Sleep Modes
//! Datasheet page 34 et seqq.

use core::{arch::asm, ptr::write_volatile};

use crate::atmega328p::interrupts;

use super::registers::{BODS, BODSE, MCUCR_ADDR_IO, SMCR_ADDR};

#[allow(dead_code)]
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

    #[inline(always)]
    unsafe fn write(&self) {
        write_volatile(SMCR_ADDR as *mut u8, self.control_register_value());
    }
}

pub fn sleep_mode(mode: SleepMode) {
    unsafe {
        mode.write();
        asm!("sleep");
    }
}

#[allow(dead_code)]
pub fn sleep_mode_with_brown_out_detector_disabled(mode: SleepMode) {
    unsafe {
        interrupts::free(|| {
            mode.write();
            disable_brown_out_detector();
        });
        asm!("sleep");
    }
}

#[inline(always)]
unsafe fn disable_brown_out_detector() {
    asm!(
        "sbi {mcucraddr}, {bods}",
        "sbi {mcucraddr}, {bodse}",
        "sbi {mcucraddr}, {bods}",
        "cbi {mcucraddr}, {bodse}",
        mcucraddr = const MCUCR_ADDR_IO,
        bodse = const BODSE,
        bods = const BODS
    );
}
