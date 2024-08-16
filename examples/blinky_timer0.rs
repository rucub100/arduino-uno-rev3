#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)] // https://doc.rust-lang.org/nightly/unstable-book/language-features/abi-avr-interrupt.html
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use atmega328p::{
    configure_output_low, configure_timer0_clock_source, configure_timer0_wgm,
    enable_global_interrupts, enable_interrupt, poweron_timer0, set_clock_prescaler, sleep_mode,
    toggle_output_pin, PortB,
};

use panic_halt as _;

/// TIMER0 OVF - Timer/Counter0 Overflow
#[no_mangle]
#[export_name = "__vector_16"]
pub extern "avr-interrupt" fn timer0_ovf_isr() {
    toggle_output_pin(PortB::PB5);
}

#[no_mangle]
pub extern "C" fn main() {
    // 16MHz / 32 = 500kHz
    // 500kHz / 1024 = 488.28125Hz
    // 488.28125Hz / 256 = 1.9073486328125Hz
    // LED turn on about every second
    set_clock_prescaler(atmega328p::ClockPrescaler::Div32);
    configure_output_low(PortB::PB5);
    poweron_timer0();
    configure_timer0_clock_source(atmega328p::ClockSource::ClkIo1024);
    configure_timer0_wgm(atmega328p::WaveformGenerationMode::Normal);
    enable_interrupt(atmega328p::Interrupt::Timer0Ovf);
    enable_global_interrupts();

    loop {
        sleep_mode(atmega328p::SleepMode::Idle);
    }
}
