#![no_std]
#![no_main]
// https://doc.rust-lang.org/nightly/unstable-book/language-features/abi-avr-interrupt.html
#![feature(abi_avr_interrupt)]

use core::ptr::{read_volatile, write_volatile};

use panic_halt as _;

mod atmega328p;

use atmega328p::registers::{DDRB_ADDR, PORTB_ADDR};

#[no_mangle]
pub extern "C" fn main() {
    // configure pin 5 of PORTB as output
    unsafe {
        let ddrb = read_volatile(DDRB_ADDR as *const u8);
        let new_ddrb = ddrb | (1 << 5);
        write_volatile(DDRB_ADDR as *mut u8, new_ddrb);
    }
    loop {
        // toggle pin 5 of PORTB
        unsafe {
            let portb = read_volatile(PORTB_ADDR as *const u8);
            let new_portb = portb ^ (1 << 5);
            write_volatile(PORTB_ADDR as *mut u8, new_portb);
            // delay
            for _ in 0..1000000 {
                core::hint::black_box(());
            }
        }
    }
}
