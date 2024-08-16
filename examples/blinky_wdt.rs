#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)] // https://doc.rust-lang.org/nightly/unstable-book/language-features/abi-avr-interrupt.html
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use atmega328p::{
    configure_output_low, configure_watchdog_mode, configure_watchdog_timer,
    enable_global_interrupts, sleep_mode, toggle_output_pin, PortB,
};
use panic_halt as _;

/// WDT - Watchdog Time-out Interrupt
#[no_mangle]
#[export_name = "__vector_6"]
pub extern "avr-interrupt" fn wdt_isr() {
    toggle_output_pin(PortB::PB5);
}

#[no_mangle]
pub extern "C" fn main() {
    configure_output_low(PortB::PB5);

    configure_watchdog_timer(atmega328p::WatchdogTimeout::Ms500);
    configure_watchdog_mode(atmega328p::WatchdogMode::Interrupt);

    enable_global_interrupts();

    loop {
        sleep_mode(atmega328p::SleepMode::Idle);
    }
}
