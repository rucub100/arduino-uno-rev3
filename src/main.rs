#![no_std]
#![no_main]
// https://doc.rust-lang.org/nightly/unstable-book/language-features/abi-avr-interrupt.html
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]

use panic_halt as _;

mod atmega328p;

#[no_mangle]
pub extern "C" fn main() {
    loop {}
}
