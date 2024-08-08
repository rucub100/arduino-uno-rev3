use core::ptr::{read_volatile, write_volatile};

use super::interrupts;

/// Linker needs this function implicitly since some core functions use it
#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe {
            *dest.add(i) = *src.add(i);
        }
        i += 1;
    }
    dest
}

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

pub(super) trait RegisterControl {
    fn get_register_addr(&self) -> u8;
    fn get_mask(&self) -> u8;

    unsafe fn modify_register<F>(&self, f: F)
    where
        F: FnOnce(u8) -> u8,
    {
        let addr = self.get_register_addr();
        let value = read_volatile(addr as *const u8);
        write_volatile(addr as *mut u8, f(value));
    }

    unsafe fn set_bit(&self) {
        self.modify_register(|value| value | self.get_mask());
    }

    unsafe fn clear_bit(&self) {
        self.modify_register(|value| value & !self.get_mask());
    }
}
