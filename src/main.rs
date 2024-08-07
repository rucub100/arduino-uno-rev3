#![no_std]
#![no_main]

use atmega328p::{sleep_mode, SleepMode};
use panic_halt as _;

/**
 * TODO:
 *
 * Create multiple examples in examples/ folder
 *
 * Interrupts:
 * - enable/disable individual interrupts
 * - allow to implement ISRs dynamically (only one registration - static check)
 *
 * Watchdog:
 * - provide simple API to enable/disable watchdog
 *
 * IO:
 * - api to configure general digital IO
 * - configure pull-up resistors
 *
 * Timer and PWM:
 * - api to configure timers
 * - compare mode
 * - capture mode
 * - api to configure PWM
 *
 * ADC
 * USART
 * SPI
 * TWI/I2C
 * EEPROM
 * Analog Comparator
 * Advanced Topics
 *
 */

#[no_mangle]
pub extern "C" fn main() {
    loop {
        sleep_mode(SleepMode::Idle);
    }
}
