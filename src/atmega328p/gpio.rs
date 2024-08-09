//! General digital I/O ports
//! Datasheet page 58 et seqq.

use core::arch::asm;

use super::{
    common::{clear_bit, read_bit, set_bit},
    registers::{
        BIT0, BIT1, BIT2, BIT3, BIT4, BIT5, BIT6, BIT7, DDRB_ADDR, DDRC_ADDR, DDRD_ADDR,
        MCUCR_ADDR_IO, PINB_ADDR, PINC_ADDR, PIND_ADDR, PORTB_ADDR, PORTC_ADDR, PORTD_ADDR, PUD,
    },
};

pub trait Port {
    fn index(&self) -> u8;
    fn get_direction_addr(&self) -> u8;
    fn get_port_addr(&self) -> u8;
    fn get_pin_addr(&self) -> u8;

    unsafe fn set_input_tri_state(&self) {
        let index = self.index();
        let ddr_addr = self.get_direction_addr();
        let port_addr = self.get_port_addr();

        clear_bit(port_addr, index);
        clear_bit(ddr_addr, index);
    }

    /// The pin will source current if externally pulled low.
    /// Set the PUB bit in the MCUCR register to disable all pull-ups.
    unsafe fn set_input_pull_up(&self) {
        let index = self.index();
        let ddr_addr = self.get_direction_addr();
        let port_addr = self.get_port_addr();

        set_bit(port_addr, index);
        clear_bit(ddr_addr, index);
    }

    unsafe fn set_output_low(&self) {
        let index = self.index();
        let ddr_addr = self.get_direction_addr();
        let port_addr = self.get_port_addr();

        clear_bit(port_addr, index);
        set_bit(ddr_addr, index);
    }

    unsafe fn set_output_high(&self) {
        let index = self.index();
        let ddr_addr = self.get_direction_addr();
        let port_addr = self.get_port_addr();

        set_bit(port_addr, index);
        set_bit(ddr_addr, index);
    }

    unsafe fn toggle(&self) {
        let index = self.index();
        let port_addr = self.get_port_addr();

        set_bit(port_addr, index);
    }

    unsafe fn read(&self) -> bool {
        let index = self.index();
        let pin_addr = self.get_pin_addr();

        read_bit(pin_addr, index)
    }
}

pub enum PortB {
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
}

impl Port for PortB {
    fn index(&self) -> u8 {
        match self {
            PortB::PB0 => BIT0,
            PortB::PB1 => BIT1,
            PortB::PB2 => BIT2,
            PortB::PB3 => BIT3,
            PortB::PB4 => BIT4,
            PortB::PB5 => BIT5,
            PortB::PB6 => BIT6,
            PortB::PB7 => BIT7,
        }
    }

    fn get_direction_addr(&self) -> u8 {
        DDRB_ADDR
    }

    fn get_port_addr(&self) -> u8 {
        PORTB_ADDR
    }

    fn get_pin_addr(&self) -> u8 {
        PINB_ADDR
    }
}

pub enum PortC {
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
}

impl Port for PortC {
    fn index(&self) -> u8 {
        match self {
            PortC::PC0 => BIT0,
            PortC::PC1 => BIT1,
            PortC::PC2 => BIT2,
            PortC::PC3 => BIT3,
            PortC::PC4 => BIT4,
            PortC::PC5 => BIT5,
            PortC::PC6 => BIT6,
        }
    }

    fn get_direction_addr(&self) -> u8 {
        DDRC_ADDR
    }

    fn get_port_addr(&self) -> u8 {
        PORTC_ADDR
    }

    fn get_pin_addr(&self) -> u8 {
        PINC_ADDR
    }
}

pub enum PortD {
    PD0,
    PD1,
    PD2,
    PD3,
    PD4,
    PD5,
    PD6,
    PD7,
}

impl Port for PortD {
    fn index(&self) -> u8 {
        match self {
            PortD::PD0 => BIT0,
            PortD::PD1 => BIT1,
            PortD::PD2 => BIT2,
            PortD::PD3 => BIT3,
            PortD::PD4 => BIT4,
            PortD::PD5 => BIT5,
            PortD::PD6 => BIT6,
            PortD::PD7 => BIT7,
        }
    }

    fn get_direction_addr(&self) -> u8 {
        DDRD_ADDR
    }

    fn get_port_addr(&self) -> u8 {
        PORTD_ADDR
    }

    fn get_pin_addr(&self) -> u8 {
        PIND_ADDR
    }
}

/// Disable pull-up resistors for all pins
pub fn disable_pull_ups() {
    unsafe {
        asm!(
            "sbi {mcucr}, {pud}",
            mcucr = const MCUCR_ADDR_IO,
            pud = const PUD
        );
    }
}

/// Allow pull-up resistors for all pins
pub fn allow_pull_ups() {
    unsafe {
        asm!(
            "cbi {mcucr}, {pud}",
            mcucr = const MCUCR_ADDR_IO,
            pud = const PUD
        );
    }
}

pub fn configure_input_tri_state(port: impl Port) {
    unsafe {
        port.set_input_tri_state();
    }
}

pub fn configure_input_pull_up(port: impl Port) {
    unsafe {
        port.set_input_pull_up();
    }
}

pub fn configure_output_low(port: impl Port) {
    unsafe {
        port.set_output_low();
    }
}

pub fn configure_output_high(port: impl Port) {
    unsafe {
        port.set_output_high();
    }
}

pub fn read_pin(port: impl Port) -> bool {
    unsafe { port.read() }
}

pub fn toggle_pin(port: impl Port) {
    unsafe {
        port.toggle();
    }
}
