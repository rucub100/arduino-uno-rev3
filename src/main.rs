#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)] // https://doc.rust-lang.org/nightly/unstable-book/language-features/abi-avr-interrupt.html
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use atmega328p::sleep_mode;
use panic_halt as _;

mod vectors;

#[no_mangle]
pub extern "C" fn main() {
    loop {
        sleep_mode(atmega328p::SleepMode::Idle);
    }
}
