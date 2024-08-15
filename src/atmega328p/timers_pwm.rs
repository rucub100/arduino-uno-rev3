//! Timers/Counters with PWM, Prescalers and more
//! Datasheet page 74 et seqq.

use core::ptr::{read_volatile, write_volatile};

use super::{
    common::{clear_bit, set_bit},
    poweroff_timer0, poweron_timer0,
    registers::{
        BIT5, BIT6, COM0A0, COM0A1, COM0B0, COM0B1, CS00, CS01, CS02, DDRD_ADDR, OCR0A_ADDR,
        OCR0B_ADDR, TCCR0A_ADDR, TCCR0B_ADDR, TCNT0_ADDR, WGM00, WGM01, WGM02,
    },
};

#[allow(dead_code)]
pub enum WaveformGenerationMode {
    /// Normal mode increments the counter from 0 (bottom) to 0xFF (max) and then overflows to 0.
    Normal,
    /// Clear Timer on Compare Match mode.
    CTC,
    /// Fast PWM mode 1.
    FastPWM1,
    /// Fast PWM mode 2.
    FastPWM2,
    /// Phase Correct PWM mode 1.
    PhaseCorrectPWM1,
    /// Phase Correct PWM mode 2.
    PhaseCorrectPWM2,
}

#[allow(dead_code)]
pub enum ClockSource {
    /// No clock source, timer is stopped.
    NoClock,
    /// No prescaler.
    ClkIo,
    /// Prescaler 8.
    ClkIo8,
    /// Prescaler 64.
    ClkIo64,
    /// Prescaler 256.
    ClkIo256,
    /// Prescaler 1024.
    ClkIo1024,
    /// Exernal clock source on Tn pin, falling edge.
    ExternalFalling,
    /// Exernal clock source on Tn pin, rising edge.
    ExternalRising,
}

#[allow(dead_code)]
pub enum CompareOutputMode {
    Normal,
    Toggle,
    Clear,
    Set,
}

mod timer0 {
    use super::*;

    #[allow(dead_code)]
    pub fn configure_timer0_clock_source(cs: ClockSource) {
        unsafe {
            match cs {
                ClockSource::NoClock => {
                    clear_bit(TCCR0B_ADDR, CS00);
                    clear_bit(TCCR0B_ADDR, CS01);
                    clear_bit(TCCR0B_ADDR, CS02);
                }
                ClockSource::ClkIo => {
                    set_bit(TCCR0B_ADDR, CS00);
                    clear_bit(TCCR0B_ADDR, CS01);
                    clear_bit(TCCR0B_ADDR, CS02);
                }
                ClockSource::ClkIo8 => {
                    clear_bit(TCCR0B_ADDR, CS00);
                    set_bit(TCCR0B_ADDR, CS01);
                    clear_bit(TCCR0B_ADDR, CS02);
                }
                ClockSource::ClkIo64 => {
                    set_bit(TCCR0B_ADDR, CS00);
                    set_bit(TCCR0B_ADDR, CS01);
                    clear_bit(TCCR0B_ADDR, CS02);
                }
                ClockSource::ClkIo256 => {
                    clear_bit(TCCR0B_ADDR, CS00);
                    clear_bit(TCCR0B_ADDR, CS01);
                    set_bit(TCCR0B_ADDR, CS02);
                }
                ClockSource::ClkIo1024 => {
                    set_bit(TCCR0B_ADDR, CS00);
                    clear_bit(TCCR0B_ADDR, CS01);
                    set_bit(TCCR0B_ADDR, CS02);
                }
                ClockSource::ExternalFalling => {
                    clear_bit(TCCR0B_ADDR, CS00);
                    set_bit(TCCR0B_ADDR, CS01);
                    set_bit(TCCR0B_ADDR, CS02);
                }
                ClockSource::ExternalRising => {
                    set_bit(TCCR0B_ADDR, CS00);
                    set_bit(TCCR0B_ADDR, CS01);
                    set_bit(TCCR0B_ADDR, CS02);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn configure_timer0_wgm(wgm: WaveformGenerationMode) {
        unsafe {
            match wgm {
                WaveformGenerationMode::Normal => {
                    clear_bit(TCCR0A_ADDR, WGM00);
                    clear_bit(TCCR0A_ADDR, WGM01);
                    clear_bit(TCCR0B_ADDR, WGM02);
                }
                WaveformGenerationMode::CTC => {
                    clear_bit(TCCR0A_ADDR, WGM00);
                    set_bit(TCCR0A_ADDR, WGM01);
                    clear_bit(TCCR0B_ADDR, WGM02);
                }
                WaveformGenerationMode::FastPWM1 => {
                    set_bit(TCCR0A_ADDR, WGM00);
                    set_bit(TCCR0A_ADDR, WGM01);
                    clear_bit(TCCR0B_ADDR, WGM02);
                }
                WaveformGenerationMode::FastPWM2 => {
                    set_bit(TCCR0A_ADDR, WGM00);
                    set_bit(TCCR0A_ADDR, WGM01);
                    set_bit(TCCR0B_ADDR, WGM02);
                }
                WaveformGenerationMode::PhaseCorrectPWM1 => {
                    set_bit(TCCR0A_ADDR, WGM00);
                    clear_bit(TCCR0A_ADDR, WGM01);
                    clear_bit(TCCR0B_ADDR, WGM02);
                }
                WaveformGenerationMode::PhaseCorrectPWM2 => {
                    set_bit(TCCR0A_ADDR, WGM00);
                    clear_bit(TCCR0A_ADDR, WGM01);
                    set_bit(TCCR0B_ADDR, WGM02);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn configure_timer0_coma(com: CompareOutputMode) {
        unsafe {
            match com {
                CompareOutputMode::Normal => {
                    clear_bit(TCCR0A_ADDR, COM0A0);
                    clear_bit(TCCR0A_ADDR, COM0A1);
                }
                CompareOutputMode::Toggle => {
                    set_bit(TCCR0A_ADDR, COM0A0);
                    clear_bit(TCCR0A_ADDR, COM0A1);
                }
                CompareOutputMode::Clear => {
                    clear_bit(TCCR0A_ADDR, COM0A0);
                    set_bit(TCCR0A_ADDR, COM0A1);
                }
                CompareOutputMode::Set => {
                    set_bit(TCCR0A_ADDR, COM0A0);
                    set_bit(TCCR0A_ADDR, COM0A1);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn configure_timer0_comb(com: CompareOutputMode) {
        unsafe {
            match com {
                CompareOutputMode::Normal => {
                    clear_bit(TCCR0A_ADDR, COM0B0);
                    clear_bit(TCCR0A_ADDR, COM0B1);
                }
                CompareOutputMode::Toggle => {
                    set_bit(TCCR0A_ADDR, COM0B0);
                    clear_bit(TCCR0A_ADDR, COM0B1);
                }
                CompareOutputMode::Clear => {
                    clear_bit(TCCR0A_ADDR, COM0B0);
                    set_bit(TCCR0A_ADDR, COM0B1);
                }
                CompareOutputMode::Set => {
                    set_bit(TCCR0A_ADDR, COM0B0);
                    set_bit(TCCR0A_ADDR, COM0B1);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn configure_timer0(
        wgm: WaveformGenerationMode,
        cs: ClockSource,
        com_a: Option<CompareOutputMode>,
        com_b: Option<CompareOutputMode>,
    ) {
        poweroff_timer0();
        configure_timer0_wgm(wgm);
        configure_timer0_clock_source(cs);
        if let Some(com) = com_a {
            configure_timer0_coma(com);
        }
        if let Some(com) = com_b {
            configure_timer0_comb(com);
        }
        poweron_timer0();
    }

    #[allow(dead_code)]
    pub fn configure_timer0_oca_output() {
        unsafe {
            set_bit(DDRD_ADDR, BIT6);
        }
    }

    #[allow(dead_code)]
    pub fn configure_timer0_ocb_output() {
        unsafe {
            set_bit(DDRD_ADDR, BIT5);
        }
    }

    #[allow(dead_code)]
    pub fn set_timer0_oca(value: u8) {
        unsafe {
            write_volatile(OCR0A_ADDR as *mut u8, value);
        }
    }

    #[allow(dead_code)]
    pub fn set_timer0_ocb(value: u8) {
        unsafe {
            write_volatile(OCR0B_ADDR as *mut u8, value);
        }
    }

    #[allow(dead_code)]
    pub fn get_timer0() -> u8 {
        unsafe { read_volatile(TCNT0_ADDR as *const u8) }
    }

    #[allow(dead_code)]
    pub fn set_timer0(value: u8) {
        unsafe {
            write_volatile(TCNT0_ADDR as *mut u8, value);
        }
    }

    #[allow(dead_code)]
    pub fn clear_timer0() {
        set_timer0(0);
    }
}

pub use timer0::*;
