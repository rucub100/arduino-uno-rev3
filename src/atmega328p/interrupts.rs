//! Interrupts
//! Datasheet page 49 et seqq.

use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

use super::registers::{EICRA_ADDR, EIMSK_ADDR, INT0, INT1};

#[inline(always)]
pub fn enable_global_interrupts() {
    unsafe {
        asm!("sei");
    }
}

#[inline(always)]
pub fn disable_global_interrupts() {
    unsafe {
        asm!("cli");
    }
}

#[allow(dead_code)]
pub fn free<F>(handler: F)
where
    F: FnOnce(),
{
    disable_global_interrupts();
    handler();
    enable_global_interrupts();
}

#[allow(dead_code)]
pub enum Interrupt {
    /// External Interrupt Request 0
    Int0,
    /// External Interrupt Request 1
    Int1,
    /// Pin Change Interrupt Request 0
    PCInt0,
    /// Pin Change Interrupt Request 1
    PCInt1,
    /// Pin Change Interrupt Request 2
    PCInt2,
    /// Watchdog Time-out Interrupt
    WDT,
    /// Timer/Counter2 Compare Match A
    Timer2CompA,
    /// Timer/Counter2 Compare Match B
    Timer2CompB,
    /// Timer/Counter2 Overflow
    Timer2Ovf,
    /// Timer/Counter1 Capture Event
    Timer1Capt,
    /// Timer/Counter1 Compare Match A
    Timer1CompA,
    /// Timer/Counter1 Compare Match B
    Timer1CompB,
    /// Timer/Counter1 Overflow
    Timer1Ovf,
    /// Timer/Counter0 Compare Match A
    Timer0CompA,
    /// Timer/Counter0 Compare Match B
    Timer0CompB,
    /// Timer/Counter0 Overflow
    Timer0Ovf,
    /// SPI Serial Transfer Complete
    SpiStc,
    /// USART Rx Complete
    UsartRx,
    /// USART Data Register Empty
    UsartDRE,
    /// USART Tx Complete
    UsartTx,
    /// ADC Conversion Complete
    ADC,
    /// EEPROM Ready
    EeRdy,
    /// Analog Comparator
    AnalogComp,
    /// Inter-Integrated Circuit
    I2C,
    /// Store Program Memory Ready
    SPMRdy,
}

impl Interrupt {
    fn get_mask(&self) -> u8 {
        match self {
            Interrupt::Int0 => 1 << INT0,
            Interrupt::Int1 => 1 << INT1,
            _ => unimplemented!(),
        }
    }

    unsafe fn enable(&self, sense_control: Option<SenseControl>) {
        match self {
            Interrupt::Int0 | Interrupt::Int1 => {
                if let Some(sense_control) = sense_control {
                    sense_control.write(self);
                }

                let eimsk = read_volatile(EIMSK_ADDR as *const u8);
                write_volatile(EIMSK_ADDR as *mut u8, eimsk | self.get_mask());
            }
            _ => unimplemented!(),
        }
    }

    unsafe fn disable(&self) {
        match self {
            Interrupt::Int0 | Interrupt::Int1 => {
                let eimsk = read_volatile(EIMSK_ADDR as *const u8);
                write_volatile(EIMSK_ADDR as *mut u8, eimsk & !self.get_mask());
            }
            _ => unimplemented!(),
        }
    }
}

#[allow(dead_code)]
pub enum SenseControl {
    LowLevel,
    AnyChange,
    FallingEdge,
    RisingEdge,
}

#[allow(dead_code)]
impl SenseControl {
    fn value(&self) -> u8 {
        match self {
            SenseControl::LowLevel => 0b00,
            SenseControl::AnyChange => 0b01,
            SenseControl::FallingEdge => 0b10,
            SenseControl::RisingEdge => 0b11,
        }
    }

    unsafe fn write(&self, interrupt: &Interrupt) {
        let mask = match interrupt {
            Interrupt::Int0 => self.value(),
            Interrupt::Int1 => self.value() << 2,
            _ => return,
        };

        let eicra = read_volatile(EICRA_ADDR as *const u8);
        write_volatile(EICRA_ADDR as *mut u8, eicra | mask);
    }
}

#[allow(dead_code)]
pub fn enable_interrupt(interrupt: Interrupt, sense_control: Option<SenseControl>) {
    unsafe {
        interrupt.enable(sense_control);
    }
}

#[allow(dead_code)]
pub fn disable_interrupt(interrupt: Interrupt) {
    unsafe {
        interrupt.disable();
    }
}
