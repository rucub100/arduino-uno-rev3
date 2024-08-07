//! Interrupts
//! Datasheet page 49 et seqq.

use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

use super::registers::{
    ACIE, ACSR_ADDR, ADCSRA_ADDR, ADIE, EECR_ADDR, EERIE, EICRA_ADDR, EIMSK_ADDR, ICIE1, INT0,
    INT1, OCIE0A, OCIE0B, OCIE1A, OCIE1B, OCIE2A, OCIE2B, PCICR_ADDR, PCIE0, PCIE1, PCIE2, RXCIE0,
    SPCR_ADDR, SPIE, SPMCSR_ADDR, SPMIE, TIMSK0_ADDR, TIMSK1_ADDR, TIMSK2_ADDR, TOIE0, TOIE1,
    TOIE2, TWCR_ADDR, TWIE, TXCIE0, UCSR0B_ADDR, UDRIE0, WDIE, WDTCSR_ADDR,
};

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
#[derive(PartialEq, Eq)]
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

/// TODO
enum PinInterrupt {}

impl Interrupt {
    fn get_mask(&self) -> u8 {
        match self {
            Interrupt::Int0 => 1 << INT0,
            Interrupt::Int1 => 1 << INT1,
            Interrupt::PCInt0 => 1 << PCIE0, // TODO - PCMSK0???
            Interrupt::PCInt1 => 1 << PCIE1, // TODO - PCMSK1???
            Interrupt::PCInt2 => 1 << PCIE2, // TODO - PCMSK2???
            Interrupt::WDT => 1 << WDIE,
            Interrupt::Timer2CompA => 1 << OCIE2A,
            Interrupt::Timer2CompB => 1 << OCIE2B,
            Interrupt::Timer2Ovf => 1 << TOIE2,
            Interrupt::Timer1Capt => 1 << ICIE1,
            Interrupt::Timer1CompA => 1 << OCIE1A,
            Interrupt::Timer1CompB => 1 << OCIE1B,
            Interrupt::Timer1Ovf => 1 << TOIE1,
            Interrupt::Timer0CompA => 1 << OCIE0A,
            Interrupt::Timer0CompB => 1 << OCIE0B,
            Interrupt::Timer0Ovf => 1 << TOIE0,
            Interrupt::SpiStc => 1 << SPIE,
            Interrupt::UsartRx => 1 << RXCIE0,
            Interrupt::UsartDRE => 1 << UDRIE0,
            Interrupt::UsartTx => 1 << TXCIE0,
            Interrupt::ADC => 1 << ADIE,
            Interrupt::EeRdy => 1 << EERIE,
            Interrupt::AnalogComp => 1 << ACIE,
            Interrupt::I2C => 1 << TWIE,
            Interrupt::SPMRdy => 1 << SPMIE,
        }
    }

    fn get_register_addr(&self) -> u8 {
        match self {
            Interrupt::Int0 | Interrupt::Int1 => EIMSK_ADDR,
            Interrupt::PCInt0 | Interrupt::PCInt1 | Interrupt::PCInt2 => PCICR_ADDR,
            Interrupt::WDT => WDTCSR_ADDR,
            Interrupt::Timer2CompA | Interrupt::Timer2CompB | Interrupt::Timer2Ovf => TIMSK2_ADDR,
            Interrupt::Timer1Capt
            | Interrupt::Timer1CompA
            | Interrupt::Timer1CompB
            | Interrupt::Timer1Ovf => TIMSK1_ADDR,
            Interrupt::Timer0CompA | Interrupt::Timer0CompB | Interrupt::Timer0Ovf => TIMSK0_ADDR,
            Interrupt::SpiStc => SPCR_ADDR,
            Interrupt::UsartRx | Interrupt::UsartDRE | Interrupt::UsartTx => UCSR0B_ADDR,
            Interrupt::ADC => ADCSRA_ADDR,
            Interrupt::EeRdy => EECR_ADDR,
            Interrupt::AnalogComp => ACSR_ADDR,
            Interrupt::I2C => TWCR_ADDR,
            Interrupt::SPMRdy => SPMCSR_ADDR,
        }
    }

    unsafe fn modify_register<F>(&self, f: F)
    where
        F: FnOnce(u8) -> u8,
    {
        let addr = self.get_register_addr();

        let value = read_volatile(addr as *const u8);
        write_volatile(addr as *mut u8, f(value));
    }

    unsafe fn enable(&self, sense_control: Option<SenseControl>) {
        if *self == Interrupt::Int0 || *self == Interrupt::Int1 {
            if let Some(sense_control) = sense_control {
                sense_control.write(self);
            }
        }

        self.modify_register(|value| value | self.get_mask());
    }

    unsafe fn disable(&self) {
        self.modify_register(|value| value & !self.get_mask());
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
pub fn enable_pin_interrupt(pin_interrupt: PinInterrupt) {
    unimplemented!();
}

#[allow(dead_code)]
pub fn disable_interrupt(interrupt: Interrupt) {
    unsafe {
        interrupt.disable();
    }
}
