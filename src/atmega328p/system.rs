use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

use crate::atmega328p::registers::{CLKPCE, CLKPR_ADDR};

use super::registers::OSCCAL_ADDR;

#[allow(dead_code)]
pub fn set_rc_oscillator_calibration(value: u8) {
    unsafe {
        write_volatile(OSCCAL_ADDR as *mut u8, value);
    }
}

#[allow(dead_code)]
pub fn get_rc_oscillator_calibration() -> u8 {
    unsafe { read_volatile(OSCCAL_ADDR as *const u8) }
}

pub enum ClockPrescaler {
    Div1,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
    Div256,
}

#[allow(dead_code)]
pub fn set_clock_prescaler(prescaler: ClockPrescaler) {
    unsafe {
        let clkpce: u8 = 1 << CLKPCE;
        let clkps = prescaler as u8;
        asm!(
            "sts {addr}, {clkpce}",
            "sts {addr}, {clkps}",
            addr = const CLKPR_ADDR,
            clkpce = in(reg) clkpce,
            clkps = in(reg) clkps,
        );
    }
}
