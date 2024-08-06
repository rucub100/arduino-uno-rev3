//! Power Management and Sleep Modes
//! Datasheet page 34 et seqq.

use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

use crate::atmega328p::interrupts;

use super::registers::{
    BODS, BODSE, MCUCR_ADDR_IO, PRADC, PRR_ADDR, PRSPI, PRTIM0, PRTIM1, PRTIM2, PRTWI, PRUSAR0,
    SMCR_ADDR,
};

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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn poweroff_i2c() {
    unsafe {
        prr_set_bit_interrupts_free(PRTWI);
    }
}

#[allow(dead_code)]
pub fn poweron_i2c() {
    unsafe {
        prr_clear_bit_interrupts_free(PRTWI);
    }
}

#[allow(dead_code)]
pub fn poweroff_timer0() {
    unsafe {
        prr_set_bit_interrupts_free(PRTIM0);
    }
}

#[allow(dead_code)]
pub fn poweron_timer0() {
    unsafe {
        prr_clear_bit_interrupts_free(PRTIM0);
    }
}

#[allow(dead_code)]
pub fn poweroff_timer1() {
    unsafe {
        prr_set_bit_interrupts_free(PRTIM1);
    }
}

#[allow(dead_code)]
pub fn poweron_timer1() {
    unsafe {
        prr_clear_bit_interrupts_free(PRTIM1);
    }
}

#[allow(dead_code)]
pub fn poweroff_timer2() {
    unsafe {
        prr_set_bit_interrupts_free(PRTIM2);
    }
}

#[allow(dead_code)]
pub fn poweron_timer2() {
    unsafe {
        prr_clear_bit_interrupts_free(PRTIM2);
    }
}

#[allow(dead_code)]
pub fn poweroff_adc() {
    unsafe {
        prr_set_bit_interrupts_free(PRADC);
    }
}

#[allow(dead_code)]
pub fn poweron_adc() {
    unsafe {
        prr_clear_bit_interrupts_free(PRADC);
    }
}

#[allow(dead_code)]
pub fn poweroff_spi() {
    unsafe {
        prr_set_bit_interrupts_free(PRSPI);
    }
}

#[allow(dead_code)]
pub fn poweron_spi() {
    unsafe {
        prr_clear_bit_interrupts_free(PRSPI);
    }
}

#[allow(dead_code)]
pub fn poweroff_usart() {
    unsafe {
        prr_set_bit_interrupts_free(PRUSAR0);
    }
}

#[allow(dead_code)]
pub fn poweron_usart() {
    unsafe {
        prr_clear_bit_interrupts_free(PRUSAR0);
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

unsafe fn prr_set_bit_interrupts_free(bit: u8) {
    interrupts::free(|| {
        let prr = read_volatile(PRR_ADDR as *const u8);
        write_volatile(PRR_ADDR as *mut u8, prr | 1 << bit);
    });
}

unsafe fn prr_clear_bit_interrupts_free(bit: u8) {
    interrupts::free(|| {
        let prr = read_volatile(PRR_ADDR as *const u8);
        write_volatile(PRR_ADDR as *mut u8, prr & !(1 << bit));
    });
}
