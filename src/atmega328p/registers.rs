//! Register/Port Description
//! Datasheet page 275 et seqq.

/// # Constants
const BIT0: usize = 0x00;
const BIT1: usize = 0x01;
const BIT2: usize = 0x02;
const BIT3: usize = 0x03;
const BIT4: usize = 0x04;
const BIT5: usize = 0x05;
const BIT6: usize = 0x06;
const BIT7: usize = 0x07;
/// Timer/Counter Overflow Flag
const TOV: usize = 0x00;
/// Timer/Counter Output Compare Flag A
const OCFA: usize = 0x01;
/// Timer/Counter Output Compare Flag B
const OCFB: usize = 0x02;

/// # The Port B is 8-bit wide
/// PINB - The Port B Input Pins Address (Read Only)
const PINB_ADDR: usize = 0x23;
const PINB_ADDR_IO: usize = 0x03;
/// DDRB - The Port B Data Direction Register
const DDRB_ADDR: usize = 0x24;
const DDRB_ADDR_IO: usize = 0x04;
/// PORTB - The Port B Data Register
const PORTB_ADDR: usize = 0x25;
const PORTB_ADDR_IO: usize = 0x05;

/// # The Port C is 7-bit wide
/// PINC - The Port C Input Pins Address (Read Only)
const PINC_ADDR: usize = 0x26;
const PINC_ADDR_IO: usize = 0x06;
/// DDRC - The Port C Data Direction Register
const DDRC_ADDR: usize = 0x27;
const DDRC_ADDR_IO: usize = 0x07;
/// PORTC - The Port C Data Register
const PORTC_ADDR: usize = 0x28;
const PORTC_ADDR_IO: usize = 0x08;

/// # The Port D is 8-bit wide
/// PIND - The Port D Input Pins Address (Read Only)
const PIND_ADDR: usize = 0x29;
const PIND_ADDR_IO: usize = 0x09;
/// DDRD - The Port D Data Direction Register
const DDRD_ADDR: usize = 0x2A;
const DDRD_ADDR_IO: usize = 0x0A;
/// PORTD - The Port D Data Register
const PORTD_ADDR: usize = 0x2B;
const PORTD_ADDR_IO: usize = 0x0B;

/// # The Timer/Counter 0 is 8-bit wide
/// TIFR0 - Timer/Counter 0 Interrupt Flag Register
const TIFR0_ADDR: usize = 0x35;
const TIFR0_ADDR_IO: usize = 0x15;

/// # The Timer/Counter 1 is 16-bit wide
/// TIFR1 - Timer/Counter 1 Interrupt Flag Register
const TIFR1_ADDR: usize = 0x36;
const TIFR1_ADDR_IO: usize = 0x16;
/// Bit 5 - Timer/Counter1 Input Capture Flag
/// This flag is set when a capture event occurs on the ICP1 pin.
/// When the input capture register (ICR1) is set by the WGM13:0 to be used as the TOP value,
/// the ICF1 flag is set when the counter reaches the TOP value.
///
/// ICF1 is automatically cleared when the input capture interrupt vector is executed.
/// Alternatively, ICF1 can be cleared by writing a logic one to its bit location.
const ICF1: usize = 0x05;

/// # The Timer/Counter 2 is 8-bit wide
/// TIFR2 - Timer/Counter 2 Interrupt Flag Register
const TIFR2_ADDR: usize = 0x37;
const TIFR2_ADDR_IO: usize = 0x17;

/// PCIFR - Pin Change Interrupt Flag Register
const PCIFR_ADDR: usize = 0x3B;
const PCIFR_ADDR_IO: usize = 0x1B;
/// Bit 0 - Pin Change Interrupt Flag 0
/// When a logic change on any PCINT7..0 pin triggers an interrupt request, PCIF0 becomes set (one).
/// If the I-bit in SREG and the PCIE0 bit in PCICR are set (one), the MCU will jump to the corresponding interrupt vector.
/// The flag is cleared when the interrupt routine is executed.
/// Alternatively, the flag can be cleared by writing a logical one to it.
const PCIF0: usize = 0x00;
/// Bit 1 - Pin Change Interrupt Flag 1
const PCIF1: usize = 0x01;
/// Bit 2 - Pin Change Interrupt Flag 2
const PCIF2: usize = 0x02;

/// EIFR - External Interrupt Flag Register
const EIFR_ADDR: usize = 0x3C;
const EIFR_ADDR_IO: usize = 0x1C;
/// Bit 0 - External Interrupt Flag 0
/// When an edge or logic change on the INT0 pin triggers an interrupt request,
/// INTF0 becomes set (one). If the I-bit in SREG and the INT0 bit in EIMSK are set (one),
/// the MCU will jump to the corresponding interrupt vector.
/// The flag is cleared when the interrupt routine is executed.
/// Alternatively, the flag can be cleared by writing a logical one to it.
/// This flag is always cleared when INT0 is configured as a level interrupt.
const INTF0: usize = 0x00;
/// Bit 1 - External Interrupt Flag 1
/// When an edge or logic change on the INT1 pin triggers an interrupt request,
/// INTF1 becomes set (one). If the I-bit in SREG and the INT1 bit in EIMSK are set (one),
/// the MCU will jump to the corresponding interrupt vector.
/// The flag is cleared when the interrupt routine is executed.
/// Alternatively, the flag can be cleared by writing a logical one to it.
/// This flag is always cleared when INT1 is configured as a level interrupt.
const INTF1: usize = 0x01;

/// EIMSK - External Interrupt Mask Register
const EIMSK_ADDR: usize = 0x3D;
const EIMSK_ADDR_IO: usize = 0x1D;
/// Bit 0 - External Interrupt Request 0 Enable
/// When the INT0 bit is set (one) and the I-bit in the status register (SREG) is set (one),
/// the external pin interrupt is enabled. The interrupt sense control0 bits 1/0 (ISC01 and ISC00)
/// in the external interrupt control register A (EICRA) define
/// whether the external interrupt is activated on rising and/or falling edge of the INT0 pin or level sensed.
/// Activity on the pin will cause an interrupt request even if INT0 is configured as an output.
/// The corresponding interrupt of external interrupt request 0 is executed from the INT0 interrupt vector.
const INT0: usize = 0x00;
/// Bit 1 - External Interrupt Request 1 Enable
/// When the INT1 bit is set (one) and the I-bit in the status register (SREG) is set (one),
/// the external pin interrupt is enabled. The interrupt sense control1 bits 1/0 (ISC11 and ISC10)
/// in the external interrupt control register A (EICRA) define
/// whether the external interrupt is activated on rising and/or falling edge of the INT1 pin or level sensed.
/// Activity on the pin will cause an interrupt request even if INT1 is configured as an output.
/// The corresponding interrupt of external interrupt request 1 is executed from the INT1 interrupt vector.
const INT1: usize = 0x01;

/// GPIOR0 - General Purpose I/O Register 0
const GPIOR0_ADDR: usize = 0x3E;
const GPIOR0_ADDR_IO: usize = 0x1E;

/// EECR - EEPROM Control Register
const EECR_ADDR: usize = 0x3F;
const EECR_ADDR_IO: usize = 0x1F;
/// Bit 0 - EERE: EEPROM Read Enable
const EERE: usize = 0x00;
/// Bit 1 - EEPE: EEPROM Write Enable
const EEPE: usize = 0x01;
/// Bit 2 - EEMPE: EEPROM Master Write Enable
const EEMPE: usize = 0x02;
/// Bit 3 - EERIE: EEPROM Ready Interrupt Enable
const EERIE: usize = 0x03;
/// Bit 4 - EEPM0: EEPROM Programming Mode Bit 0
const EEPM0: usize = 0x04;
/// Bit 5 - EEPM1: EEPROM Programming Mode Bit 1
const EEPM1: usize = 0x05;

/// EEDR - EEPROM Data Register
const EEDR_ADDR: usize = 0x40;
const EEDR_ADDR_IO: usize = 0x20;

/// EEARL - EEPROM Address Register Low Byte
const EEARL_ADDR: usize = 0x41;
const EEARL_ADDR_IO: usize = 0x21;
/// EEARH - EEPROM Address Register High Byte
const EEARH_ADDR: usize = 0x42;
const EEARH_ADDR_IO: usize = 0x22;

/// GTCCR - General Timer/Counter Control Register
const GTCCR_ADDR: usize = 0x43;
const GTCCR_ADDR_IO: usize = 0x23;
/// Bit 0 - Prescaler Reset (Timer/Counter 1 and 2)
const PSRSYNC: usize = 0x00;
/// Bit 1 - Prescaler Reset Timer/Counter2
const PSRASY: usize = 0x01;
/// Bit 7 - Timer/Counter Synchronization Mode
const TSM: usize = 0x07;

/// TCCR0A - Timer/Counter 0 Control Register A
const TCCR0A_ADDR: usize = 0x44;
const TCCR0A_ADDR_IO: usize = 0x24;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TCCR0B - Timer/Counter 0 Control Register B
const TCCR0B_ADDR: usize = 0x45;
const TCCR0B_ADDR_IO: usize = 0x25;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TCNT0 - Timer/Counter 0
const TCNT0_ADDR: usize = 0x46;
const TCNT0_ADDR_IO: usize = 0x26;

/// OCR0A - Timer/Counter 0 Output Compare Register A
const OCR0A_ADDR: usize = 0x47;
const OCR0A_ADDR_IO: usize = 0x27;

/// OCR0B - Timer/Counter 0 Output Compare Register B
const OCR0B_ADDR: usize = 0x48;
const OCR0B_ADDR_IO: usize = 0x28;

/// GPIOR1 - General Purpose I/O Register 1
const GPIOR1_ADDR: usize = 0x4A;
const GPIOR1_ADDR_IO: usize = 0x2A;

/// GPIOR2 - General Purpose I/O Register 2
const GPIOR2_ADDR: usize = 0x4B;
const GPIOR2_ADDR_IO: usize = 0x2B;

/// SPCR - SPI Control Register
const SPCR_ADDR: usize = 0x4C;
const SPCR_ADDR_IO: usize = 0x2C;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// SPSR - SPI Status Register
const SPSR_ADDR: usize = 0x4D;
const SPSR_ADDR_IO: usize = 0x2D;
/// Bit 0 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// SPDR - SPI Data Register
const SPDR_ADDR: usize = 0x4E;
const SPDR_ADDR_IO: usize = 0x2E;

/// ACSR - Analog Comparator Control and Status Register
const ACSR_ADDR: usize = 0x50;
const ACSR_ADDR_IO: usize = 0x30;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// SMCR - Sleep Mode Control Register
const SMCR_ADDR: usize = 0x53;
const SMCR_ADDR_IO: usize = 0x33;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO

/// MCUSR - MCU Status Register
const MCUSR_ADDR: usize = 0x54;
const MCUSR_ADDR_IO: usize = 0x34;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO

/// MCUCR - MCU Control Register
const MCUCR_ADDR: usize = 0x55;
const MCUCR_ADDR_IO: usize = 0x35;
/// Bit 0 - Interrupt Vector Change Enable
/// The IVCE bit must be written to logic one to enable change of the IVSEL bit.
/// IVCE is cleared by hardware four cycles after it is written or when IVSEL is written.
/// Setting the IVCE bit will disable interrupts, as explained in the IVSEL description above.
const IVCE: usize = 0x00;
/// Bit 1 - Interrupt Vector Select
/// When the IVSEL bit is cleared (zero), the interrupt vectors are placed at the start of the flash memory.
/// When this bit is set (one), the interrupt vectors are moved to the beginning of the boot loader section of the flash.
/// The actual address of the start of the boot flash section is determined by the BOOTSZ fuses.
///
/// To avoid unintentional changes of interrupt vector tables,
/// a special write procedure must be followed to change the IVSEL bit:
///     a. Write the interrupt vector change enable (IVCE) bit to one.
///     b. Within four cycles, write the desired value to IVSEL while writing a zero to IVCE.
///
/// Interrupts will automatically be disabled while this sequence is executed.
/// Interrupts are disabled in the cycle IVCE is set,
/// and they remain disabled until after the instruction following the write to IVSEL.
/// If IVSEL is not written, interrupts remain disabled for four cycles.
/// The I-bit in the status register is unaffected by the automatic disabling.
const IVSEL: usize = 0x01;
/// Bit 4 - Pull-up Disable
/// When this bit is written to one, the pull-ups in the I/O ports are disabled
/// even if the DDxn and PORTxn registers are configured to enable the pull-ups ({DDxn, PORTxn} = 0b01).
const PUD: usize = 0x04;
/// Bit 5 - BOD Sleep Enable
/// BODSE enables setting of BODS control bit, as explained in BODS bit description.
/// BOD disable is controlled by a timed sequence.
const BODSE: usize = 0x05;
/// Bit 6 - BOD Sleep
/// The BODS bit must be written to logic one in order to turn off BOD during sleep.
/// Writing to the BODS bit is controlled by a timed sequence and an enable bit, BODSE in MCUCR.
/// To disable BOD in relevant sleep modes, both BODS and BODSE must first be set to one.
/// Then, to set the BODS bit, BODS must be set to one and BODSE must be set to zero within four clock cycles.
///
/// The BODS bit is active three clock cycles after it is set.
/// A sleep instruction must be executed while BODS is active in order to turn off the BOD for the actual sleep mode.
/// The BODS bit is automatically cleared after three clock cycles.
const BODS: usize = 0x06;

/// SPMCSR - Store Program Memory Control and Status Register
const SPMCSR_ADDR: usize = 0x57;
const SPMCSR_ADDR_IO: usize = 0x37;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// SPL - Stack Pointer Low Byte
const SPL_ADDR: usize = 0x5D;
const SPL_ADDR_IO: usize = 0x3D;
/// SPH - Stack Pointer High Byte
const SPH_ADDR: usize = 0x5E;
const SPH_ADDR_IO: usize = 0x3E;

/// SREG - Status Register
const SREG_ADDR: usize = 0x5F;
const SREG_ADDR_IO: usize = 0x3F;
/// Bit 0 - C: Carry Flag
const C: usize = 0x00;
/// Bit 1 - Z: Zero Flag
const Z: usize = 0x01;
/// Bit 2 - N: Negative Flag
const N: usize = 0x02;
/// Bit 3 - V: Two’s complement overflow Flag
const V: usize = 0x03;
/// Bit 4 - S: Sign Bit, S = N ⊕ V
const S: usize = 0x04;
/// Bit 5 - H: Half Carry Flag
const H: usize = 0x05;
/// Bit 6 - T: Transfer Bit
const T: usize = 0x06;
/// Bit 7 - I: Global Interrupt Enable
const I: usize = 0x07;

/// WDTCSR - Watchdog Timer Control Register
const WDTCSR_ADDR: usize = 0x60;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// CLKPR - Clock Prescale Register
const CLKPR_ADDR: usize = 0x61;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 7 - TODO

/// PRR - Power Reduction Register
const PRR_ADDR: usize = 0x64;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// OSCCAL - Oscillator Calibration Value
const OSCCAL_ADDR: usize = 0x66;

/// PCICR - Pin Change Interrupt Control Register
const PCICR_ADDR: usize = 0x68;
/// Bit 0 - Pin Change Interrupt Enable 0
const PCIE0: usize = 0x00;
/// Bit 1 - Pin Change Interrupt Enable 1
const PCIE1: usize = 0x01;
/// Bit 2 - Pin Change Interrupt Enable 2
const PCIE2: usize = 0x02;

/// EICRA - External Interrupt Control Register A
const EICRA_ADDR: usize = 0x69;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO

/// PCMSK0 - Pin Change Mask Register 0
const PCMSK0_ADDR: usize = 0x6B;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// PCMSK1 - Pin Change Mask Register 1
const PCMSK1_ADDR: usize = 0x6C;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO

/// PCMSK2 - Pin Change Mask Register 2
const PCMSK2_ADDR: usize = 0x6D;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TIMSK0 - Timer/Counter 0 Interrupt Mask Register
const TIMSK0_ADDR: usize = 0x6E;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO

/// TIMSK1 - Timer/Counter 1 Interrupt Mask Register
const TIMSK1_ADDR: usize = 0x6F;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 5 - TODO

/// TIMSK2 - Timer/Counter 2 Interrupt Mask Register
const TIMSK2_ADDR: usize = 0x70;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO

/// ADCL - ADC Data Register Low Byte
const ADCL_ADDR: usize = 0x78;
/// ADCH - ADC Data Register High Byte
const ADCH_ADDR: usize = 0x79;

/// ADCSRA - ADC Control and Status Register A
const ADCSRA_ADDR: usize = 0x7A;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// ADCSRB - ADC Control and Status Register B
const ADCSRB_ADDR: usize = 0x7B;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 6 - TODO

/// ADMUX - ADC Multiplexer Selection Register
const ADMUX_ADDR: usize = 0x7C;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// DIDR0 - Digital Input Disable Register 0
const DIDR0_ADDR: usize = 0x7E;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO

/// DIDR1 - Digital Input Disable Register 1
const DIDR1_ADDR: usize = 0x7F;
/// Bit 0 - TODO
/// Bit 1 - TODO

/// TCCR1A - Timer/Counter 1 Control Register A
const TCCR1A_ADDR: usize = 0x80;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TCCR1A - Timer/Counter 1 Control Register B
const TCCR1B_ADDR: usize = 0x81;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TCCR1A - Timer/Counter 1 Control Register C
const TCCR1C_ADDR: usize = 0x82;
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TCNT1L - Timer/Counter 1 Register Low Byte
const TCNT1L_ADDR: usize = 0x84;
/// TCNT1H - Timer/Counter 1 Register High Byte
const TCNT1H_ADDR: usize = 0x85;

/// ICR1L - Timer/Counter 1 Input Capture Register Low Byte
const ICR1L_ADDR: usize = 0x86;
/// ICR1H - Timer/Counter 1 Input Capture Register High Byte
const ICR1H_ADDR: usize = 0x87;

/// OCR1AL - Timer/Counter 1 Output Compare Register A Low Byte
const OCR1AL_ADDR: usize = 0x88;
/// OCR1AH - Timer/Counter 1 Output Compare Register A High Byte
const OCR1AH_ADDR: usize = 0x89;

/// OCR1AL - Timer/Counter 1 Output Compare Register B Low Byte
const OCR1BL_ADDR: usize = 0x8A;
/// OCR1AH - Timer/Counter 1 Output Compare Register B High Byte
const OCR1BH_ADDR: usize = 0x8B;

/// TCCR2A - Timer/Counter 2 Control Register A
const TCCR2A_ADDR: usize = 0xB0;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TCCR2B - Timer/Counter 2 Control Register B
const TCCR2B_ADDR: usize = 0xB1;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TCNT2 - Timer/Counter 2 Register
const TCNT2_ADDR: usize = 0xB2;

/// OCR2A - Timer/Counter 2 Output Compare Register A
const OCR2A_ADDR: usize = 0xB3;

/// OCR2B - Timer/Counter 2 Output Compare
const OCR2B_ADDR: usize = 0xB4;

/// ASSR - Asynchronous Status Register
const ASSR_ADDR: usize = 0xB6;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO

/// TWBR - TWI Bit Rate Register
const TWBR_ADDR: usize = 0xB8;

/// TWSR - TWI Status Register
const TWSR_ADDR: usize = 0xB9;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TWAR - TWI (Slave) Address Register
const TWAR_ADDR: usize = 0xBA;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TWDR - TWI Data Register
const TWDR_ADDR: usize = 0xBB;

/// TWCR - TWI Control Register
const TWCR_ADDR: usize = 0xBC;
/// Bit 0 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// TWAMR - TWI (Slave) Address Mask Register
const TWAMR_ADDR: usize = 0xBD;
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// UCSR0A - USART Control and Status Register A
const UCSR0A_ADDR: usize = 0xC0;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// UCSROB - USART Control and Status Register B
const UCSR0B_ADDR: usize = 0xC1;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// UCSROC - USART Control and Status Register C
const UCSR0C_ADDR: usize = 0xC2;
/// Bit 0 - TODO
/// Bit 1 - TODO
/// Bit 2 - TODO
/// Bit 3 - TODO
/// Bit 4 - TODO
/// Bit 5 - TODO
/// Bit 6 - TODO
/// Bit 7 - TODO

/// UBRR0L - USART Baud Rate Register Low Byte
const UBRR0L_ADDR: usize = 0xC4;
/// UBRR0H - USART Baud Rate Register High Byte
const UBRR0H_ADDR: usize = 0xC5;

/// UDR0 - USART I/O Data Register
const UDR0_ADDR: usize = 0xC6;
