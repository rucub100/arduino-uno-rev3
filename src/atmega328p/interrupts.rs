//! Interrupts
//! Datasheet page 49 et seqq.

use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

use super::{
    common::RegisterControl,
    registers::{
        ACIE, ACSR_ADDR, ADCSRA_ADDR, ADIE, EECR_ADDR, EERIE, EICRA_ADDR, EIMSK_ADDR, ICIE1, INT0,
        INT1, OCIE0A, OCIE0B, OCIE1A, OCIE1B, OCIE2A, OCIE2B, PCICR_ADDR, PCIE0, PCIE1, PCIE2,
        PCINT0, PCINT1, PCINT10, PCINT11, PCINT12, PCINT13, PCINT14, PCINT16, PCINT17, PCINT18,
        PCINT19, PCINT2, PCINT20, PCINT21, PCINT22, PCINT23, PCINT3, PCINT4, PCINT5, PCINT6,
        PCINT7, PCINT8, PCINT9, PCMSK0_ADDR, PCMSK1_ADDR, PCMSK2_ADDR, RXCIE0, SPCR_ADDR, SPIE,
        SPMCSR_ADDR, SPMIE, TIMSK0_ADDR, TIMSK1_ADDR, TIMSK2_ADDR, TOIE0, TOIE1, TOIE2, TWCR_ADDR,
        TWIE, TXCIE0, UCSR0B_ADDR, UDRIE0, WDIE, WDTCSR_ADDR,
    },
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
            Interrupt::Int0(_) => self.value(),
            Interrupt::Int1(_) => self.value() << 2,
            _ => return,
        };

        let eicra = read_volatile(EICRA_ADDR as *const u8);
        write_volatile(EICRA_ADDR as *mut u8, eicra | mask);
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum PinChangeInt0Pin {
    PCInt0,
    PCInt1,
    PCInt2,
    PCInt3,
    PCInt4,
    PCInt5,
    PCInt6,
    PCInt7,
}

#[allow(dead_code)]
impl RegisterControl for PinChangeInt0Pin {
    fn get_mask(&self) -> u8 {
        match self {
            PinChangeInt0Pin::PCInt0 => 1 << PCINT0,
            PinChangeInt0Pin::PCInt1 => 1 << PCINT1,
            PinChangeInt0Pin::PCInt2 => 1 << PCINT2,
            PinChangeInt0Pin::PCInt3 => 1 << PCINT3,
            PinChangeInt0Pin::PCInt4 => 1 << PCINT4,
            PinChangeInt0Pin::PCInt5 => 1 << PCINT5,
            PinChangeInt0Pin::PCInt6 => 1 << PCINT6,
            PinChangeInt0Pin::PCInt7 => 1 << PCINT7,
        }
    }

    fn get_register_addr(&self) -> u8 {
        PCMSK0_ADDR
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum PinChangeInt1Pin {
    PCInt8,
    PCInt9,
    PCInt10,
    PCInt11,
    PCInt12,
    PCInt13,
    PCInt14,
}

#[allow(dead_code)]
impl RegisterControl for PinChangeInt1Pin {
    fn get_mask(&self) -> u8 {
        match self {
            PinChangeInt1Pin::PCInt8 => 1 << PCINT8,
            PinChangeInt1Pin::PCInt9 => 1 << PCINT9,
            PinChangeInt1Pin::PCInt10 => 1 << PCINT10,
            PinChangeInt1Pin::PCInt11 => 1 << PCINT11,
            PinChangeInt1Pin::PCInt12 => 1 << PCINT12,
            PinChangeInt1Pin::PCInt13 => 1 << PCINT13,
            PinChangeInt1Pin::PCInt14 => 1 << PCINT14,
        }
    }

    fn get_register_addr(&self) -> u8 {
        PCMSK1_ADDR
    }
}
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum PinChangeInt2Pin {
    PCInt16,
    PCInt17,
    PCInt18,
    PCInt19,
    PCInt20,
    PCInt21,
    PCInt22,
    PCInt23,
}

#[allow(dead_code)]
impl RegisterControl for PinChangeInt2Pin {
    fn get_mask(&self) -> u8 {
        match self {
            PinChangeInt2Pin::PCInt16 => 1 << PCINT16,
            PinChangeInt2Pin::PCInt17 => 1 << PCINT17,
            PinChangeInt2Pin::PCInt18 => 1 << PCINT18,
            PinChangeInt2Pin::PCInt19 => 1 << PCINT19,
            PinChangeInt2Pin::PCInt20 => 1 << PCINT20,
            PinChangeInt2Pin::PCInt21 => 1 << PCINT21,
            PinChangeInt2Pin::PCInt22 => 1 << PCINT22,
            PinChangeInt2Pin::PCInt23 => 1 << PCINT23,
        }
    }

    fn get_register_addr(&self) -> u8 {
        PCMSK2_ADDR
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum Interrupt {
    /// External Interrupt Request 0
    Int0(SenseControl),
    /// External Interrupt Request 1
    Int1(SenseControl),
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

impl RegisterControl for Interrupt {
    fn get_mask(&self) -> u8 {
        match self {
            Interrupt::Int0(_) => 1 << INT0,
            Interrupt::Int1(_) => 1 << INT1,
            Interrupt::PCInt0 => 1 << PCIE0,
            Interrupt::PCInt1 => 1 << PCIE1,
            Interrupt::PCInt2 => 1 << PCIE2,
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
            Interrupt::Int0(_) | Interrupt::Int1(_) => EIMSK_ADDR,
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
}

#[allow(dead_code)]
pub fn enable_interrupt(interrupt: Interrupt) {
    unsafe {
        interrupt.set_bit();
    }
}

#[allow(dead_code)]
pub fn enable_pcint0_pin_interrupt(pin: PinChangeInt0Pin) {
    unsafe {
        pin.set_bit();
    }
}

#[allow(dead_code)]
pub fn enable_pcint1_pin_interrupt(pin: PinChangeInt1Pin) {
    unsafe {
        pin.set_bit();
    }
}

#[allow(dead_code)]
pub fn enable_pcint2_pin_interrupt(pin: PinChangeInt2Pin) {
    unsafe {
        pin.set_bit();
    }
}

#[allow(dead_code)]
pub fn disable_pcint0_pin_interrupt(pin: PinChangeInt0Pin) {
    unsafe {
        pin.clear_bit();
    }
}

#[allow(dead_code)]
pub fn disable_pcint1_pin_interrupt(pin: PinChangeInt1Pin) {
    unsafe {
        pin.clear_bit();
    }
}

#[allow(dead_code)]
pub fn disable_pcint2_pin_interrupt(pin: PinChangeInt2Pin) {
    unsafe {
        pin.clear_bit();
    }
}

#[allow(dead_code)]
pub fn disable_interrupt(interrupt: Interrupt) {
    unsafe {
        interrupt.clear_bit();
    }
}
