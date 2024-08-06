#![no_std]
#![no_main]

use atmega328p::{sleep_mode, SleepMode};
use panic_halt as _;

#[no_mangle]
pub extern "C" fn main() {
    loop {
        sleep_mode(SleepMode::Idle);
    }
}
