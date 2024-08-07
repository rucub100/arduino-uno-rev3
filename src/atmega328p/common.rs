use core::ptr::{read_volatile, write_volatile};

use super::interrupts;

pub(super) unsafe fn set_bit(addr: u8, bit: u8) {
    let value = read_volatile(addr as *const u8);
    write_volatile(addr as *mut u8, value | 1 << bit);
}

pub(super) unsafe fn set_bit_interrupts_free(addr: u8, bit: u8) {
    interrupts::free(|| set_bit(addr, bit));
}

pub(super) unsafe fn clear_bit(addr: u8, bit: u8) {
    let value = read_volatile(addr as *const u8);
    write_volatile(addr as *mut u8, value & !(1 << bit));
}

pub(super) unsafe fn clear_bit_interrupts_free(addr: u8, bit: u8) {
    interrupts::free(|| clear_bit(addr, bit));
}
