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
const TOV: usize = BIT0;
/// Timer/Counter Output Compare Flag A
const OCFA: usize = BIT1;
/// Timer/Counter Output Compare Flag B
const OCFB: usize = BIT2;

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
const ICF1: usize = BIT5;

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
const PCIF0: usize = BIT0;
/// Bit 1 - Pin Change Interrupt Flag 1
const PCIF1: usize = BIT1;
/// Bit 2 - Pin Change Interrupt Flag 2
const PCIF2: usize = BIT2;

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
const INTF0: usize = BIT0;
/// Bit 1 - External Interrupt Flag 1
/// When an edge or logic change on the INT1 pin triggers an interrupt request,
/// INTF1 becomes set (one). If the I-bit in SREG and the INT1 bit in EIMSK are set (one),
/// the MCU will jump to the corresponding interrupt vector.
/// The flag is cleared when the interrupt routine is executed.
/// Alternatively, the flag can be cleared by writing a logical one to it.
/// This flag is always cleared when INT1 is configured as a level interrupt.
const INTF1: usize = BIT1;

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
const INT0: usize = BIT0;
/// Bit 1 - External Interrupt Request 1 Enable
/// When the INT1 bit is set (one) and the I-bit in the status register (SREG) is set (one),
/// the external pin interrupt is enabled. The interrupt sense control1 bits 1/0 (ISC11 and ISC10)
/// in the external interrupt control register A (EICRA) define
/// whether the external interrupt is activated on rising and/or falling edge of the INT1 pin or level sensed.
/// Activity on the pin will cause an interrupt request even if INT1 is configured as an output.
/// The corresponding interrupt of external interrupt request 1 is executed from the INT1 interrupt vector.
const INT1: usize = BIT1;

/// GPIOR0 - General Purpose I/O Register 0
const GPIOR0_ADDR: usize = 0x3E;
const GPIOR0_ADDR_IO: usize = 0x1E;

/// EECR - EEPROM Control Register
const EECR_ADDR: usize = 0x3F;
const EECR_ADDR_IO: usize = 0x1F;
/// Bit 0 - EERE: EEPROM Read Enable
const EERE: usize = BIT0;
/// Bit 1 - EEPE: EEPROM Write Enable
const EEPE: usize = BIT1;
/// Bit 2 - EEMPE: EEPROM Master Write Enable
const EEMPE: usize = BIT2;
/// Bit 3 - EERIE: EEPROM Ready Interrupt Enable
const EERIE: usize = BIT3;
/// Bit 4 - EEPM0: EEPROM Programming Mode Bit 0
const EEPM0: usize = BIT4;
/// Bit 5 - EEPM1: EEPROM Programming Mode Bit 1
const EEPM1: usize = BIT5;

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
const PSRSYNC: usize = BIT0;
/// Bit 1 - Prescaler Reset Timer/Counter2
const PSRASY: usize = BIT1;
/// Bit 7 - Timer/Counter Synchronization Mode
const TSM: usize = BIT7;

/// TCCR0A - Timer/Counter 0 Control Register A
const TCCR0A_ADDR: usize = 0x44;
const TCCR0A_ADDR_IO: usize = 0x24;
/// Bit 0 - Waveform Generation Mode
const WGM00: usize = BIT0;
/// Bit 1 - Waveform Generation Mode
const WGM01: usize = BIT1;
/// Bit 4 - Compare Match Output Mode for Channel B
const COM0B0: usize = BIT4;
/// Bit 5 - Compare Match Output Mode for Channel B
const COM0B1: usize = BIT5;
/// Bit 6 - Compare Match Output Mode for Channel A
const COM0A0: usize = BIT6;
/// Bit 7 - Compare Match Output Mode for Channel A
const COM0A1: usize = BIT7;

/// TCCR0B - Timer/Counter 0 Control Register B
const TCCR0B_ADDR: usize = 0x45;
const TCCR0B_ADDR_IO: usize = 0x25;
/// Bit 0 - Clock Select
const CS00: usize = BIT0;
/// Bit 1 - Clock Select
const CS01: usize = BIT1;
/// Bit 2 - Clock Select
const CS02: usize = BIT2;
/// Bit 3 - Waveform Generation Mode
const WGM02: usize = BIT3;
/// Bit 6 - Force Output Compare for Channel B
const FOC0B: usize = BIT6;
/// Bit 7 - Force Output Compare for Channel A
const FOC0A: usize = BIT7;

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
/// Bit 0 - SPI Clock Rate Select 0
const SPR0: usize = BIT0;
/// Bit 1 - SPI Clock Rate Select 1
const SPR1: usize = BIT1;
/// Bit 2 - SPI Clock Phase
const CPHA: usize = BIT2;
/// Bit 3 - SPI Clock Polarity
const CPOL: usize = BIT3;
/// Bit 4 - Master/Slave Select
const MSTR: usize = BIT4;
/// Bit 5 - Data Order
const DORD: usize = BIT5;
/// Bit 6 - SPI Enable
const SPE: usize = BIT6;
/// Bit 7 - SPI Interrupt Enable
const SPIE: usize = BIT7;

/// SPSR - SPI Status Register
const SPSR_ADDR: usize = 0x4D;
const SPSR_ADDR_IO: usize = 0x2D;
/// Bit 0 - Double SPI Speed Bit
const SPI2X: usize = BIT0;
/// Bit 6 - Write Collision Flag
const WCOL: usize = BIT6;
/// Bit 7 - SPI Interrupt Flag
const SPIF: usize = BIT7;

/// SPDR - SPI Data Register
const SPDR_ADDR: usize = 0x4E;
const SPDR_ADDR_IO: usize = 0x2E;

/// ACSR - Analog Comparator Control and Status Register
const ACSR_ADDR: usize = 0x50;
const ACSR_ADDR_IO: usize = 0x30;
/// Bit 0 - Analog Comparator Interrupt Mode Select 0
const ACIS0: usize = BIT0;
/// Bit 1 - Analog Comparator Interrupt Mode Select 1
const ACIS1: usize = BIT1;
/// Bit 2 - Analog Comparator Input Capture Enable
const ACIC: usize = BIT2;
/// Bit 3 - Analog Comparator Interrupt Enable
const ACIE: usize = BIT3;
/// Bit 4 - Analog Comparator Interrupt Flag
const ACI: usize = BIT4;
/// Bit 5 - Analog Comparator Output
const ACO: usize = BIT5;
/// Bit 6 - Analog Comparator Bandgap Select
const ACBG: usize = BIT6;
/// Bit 7 - Analog Comparator Disable
const ACD: usize = BIT7;

/// SMCR - Sleep Mode Control Register
const SMCR_ADDR: usize = 0x53;
const SMCR_ADDR_IO: usize = 0x33;
/// Bit 0 - Sleep Enable
const SE: usize = BIT0;
/// Bit 1 - Sleep Mode Select Bit 0
const SM0: usize = BIT1;
/// Bit 2 - Sleep Mode Select Bit 1
const SM1: usize = BIT2;
/// Bit 3 - Sleep Mode Select Bit 2
const SM2: usize = BIT3;

/// MCUSR - MCU Status Register
const MCUSR_ADDR: usize = 0x54;
const MCUSR_ADDR_IO: usize = 0x34;
/// Bit 0 - Power-on Reset Flag
const PORF: usize = BIT0;
/// Bit 1 - External Reset Flag
const EXTRF: usize = BIT1;
/// Bit 2 - Brown-out Reset Flag
const BORF: usize = BIT2;
/// Bit 3 - Watchdog Reset Flag
const WDRF: usize = BIT3;

/// MCUCR - MCU Control Register
const MCUCR_ADDR: usize = 0x55;
const MCUCR_ADDR_IO: usize = 0x35;
/// Bit 0 - Interrupt Vector Change Enable
/// The IVCE bit must be written to logic one to enable change of the IVSEL bit.
/// IVCE is cleared by hardware four cycles after it is written or when IVSEL is written.
/// Setting the IVCE bit will disable interrupts, as explained in the IVSEL description above.
const IVCE: usize = BIT0;
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
const IVSEL: usize = BIT1;
/// Bit 4 - Pull-up Disable
/// When this bit is written to one, the pull-ups in the I/O ports are disabled
/// even if the DDxn and PORTxn registers are configured to enable the pull-ups ({DDxn, PORTxn} = 0b01).
const PUD: usize = BIT4;
/// Bit 5 - BOD Sleep Enable
/// BODSE enables setting of BODS control bit, as explained in BODS bit description.
/// BOD disable is controlled by a timed sequence.
const BODSE: usize = BIT5;
/// Bit 6 - BOD Sleep
/// The BODS bit must be written to logic one in order to turn off BOD during sleep.
/// Writing to the BODS bit is controlled by a timed sequence and an enable bit, BODSE in MCUCR.
/// To disable BOD in relevant sleep modes, both BODS and BODSE must first be set to one.
/// Then, to set the BODS bit, BODS must be set to one and BODSE must be set to zero within four clock cycles.
///
/// The BODS bit is active three clock cycles after it is set.
/// A sleep instruction must be executed while BODS is active in order to turn off the BOD for the actual sleep mode.
/// The BODS bit is automatically cleared after three clock cycles.
const BODS: usize = BIT6;

/// SPMCSR - Store Program Memory Control and Status Register
const SPMCSR_ADDR: usize = 0x57;
const SPMCSR_ADDR_IO: usize = 0x37;
/// Bit 0 - Self Programming Enable
const SELFPRGN: usize = BIT0;
/// Bit 1 - Page Erase
const PGERS: usize = BIT1;
/// Bit 2 - Page Write
const PGWRT: usize = BIT2;
/// Bit 3 - Boot Lock Bit Set
const BLBSET: usize = BIT3;
/// Bit 4 - Read While Write Section Read Enable
const RWWSRE: usize = BIT4;
/// Bit 6 - Read While Write Section Busy
const RWWSB: usize = BIT6;
/// Bit 7 - Store Program Memory Interrupt Enable
const SPMIE: usize = BIT7;

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
const C: usize = BIT0;
/// Bit 1 - Z: Zero Flag
const Z: usize = BIT1;
/// Bit 2 - N: Negative Flag
const N: usize = BIT2;
/// Bit 3 - V: Two’s complement overflow Flag
const V: usize = BIT3;
/// Bit 4 - S: Sign Bit, S = N ⊕ V
const S: usize = BIT4;
/// Bit 5 - H: Half Carry Flag
const H: usize = BIT5;
/// Bit 6 - T: Transfer Bit
const T: usize = BIT6;
/// Bit 7 - I: Global Interrupt Enable
const I: usize = BIT7;

/// WDTCSR - Watchdog Timer Control Register
const WDTCSR_ADDR: usize = 0x60;
/// Bit 0 - Watchdog Timer Prescaler 0
const WDP0: usize = BIT0;
/// Bit 1 - Watchdog Timer Prescaler 1
const WDP1: usize = BIT1;
/// Bit 2 - Watchdog Timer Prescaler 2
const WDP2: usize = BIT2;
/// Bit 3 - Watchdog System Reset Enable
const WDE: usize = BIT3;
/// Bit 4 - Watchdog Change Enable
const WDCE: usize = BIT4;
/// Bit 5 - Watchdog Timer Prescaler 3
const WDP3: usize = BIT5;
/// Bit 6 - Watchdog Interrupt Enable
const WDIE: usize = BIT6;
/// Bit 7 - Watchdog Interrupt Flag
const WDIF: usize = BIT7;

/// CLKPR - Clock Prescale Register
const CLKPR_ADDR: usize = 0x61;
/// Bit 0 - Clock Prescaler 0
const CLKPS0: usize = BIT0;
/// Bit 1 - Clock Prescaler 1
const CLKPS1: usize = BIT1;
/// Bit 2 - Clock Prescaler 2
const CLKPS2: usize = BIT2;
/// Bit 3 - Clock Prescaler 3
const CLKPS3: usize = BIT3;
/// Bit 7 - Clock Prescaler Change Enable
const CLKPCE: usize = BIT7;

/// PRR - Power Reduction Register
const PRR_ADDR: usize = 0x64;
/// Bit 0 - Power Reduction ADC
const PRADC: usize = BIT0;
/// Bit 1 - Power Reduction USART0
const PRUSAR0: usize = BIT1;
/// Bit 2 - Power Reduction SPI
const PRSPI: usize = BIT2;
/// Bit 3 - Power Reduction Timer/Counter 1
const PRTIM1: usize = BIT3;
/// Bit 5 - Power Reduction Timer/Counter 0
const PRTIM0: usize = BIT5;
/// Bit 6 - Power Reduction Timer/Counter 2
const PRTIM2: usize = BIT6;
/// Bit 7 - Power Reduction TWI
const PRTWI: usize = BIT7;

/// OSCCAL - Oscillator Calibration Value
const OSCCAL_ADDR: usize = 0x66;

/// PCICR - Pin Change Interrupt Control Register
const PCICR_ADDR: usize = 0x68;
/// Bit 0 - Pin Change Interrupt Enable 0
const PCIE0: usize = BIT0;
/// Bit 1 - Pin Change Interrupt Enable 1
const PCIE1: usize = BIT1;
/// Bit 2 - Pin Change Interrupt Enable 2
const PCIE2: usize = BIT2;

/// EICRA - External Interrupt Control Register A
const EICRA_ADDR: usize = 0x69;
/// Bit 0 - Interrupt Sense Control 0 Bit 0
const ISC00: usize = BIT0;
/// Bit 1 - Interrupt Sense Control 0 Bit 1
const ISC01: usize = BIT1;
/// Bit 2 - Interrupt Sense Control 1 Bit 0
const ISC10: usize = BIT2;
/// Bit 3 - Interrupt Sense Control 1 Bit 1
const ISC11: usize = BIT3;

/// PCMSK0 - Pin Change Mask Register 0
const PCMSK0_ADDR: usize = 0x6B;
/// Bit 0 - Pin Change Enable Mask 0
const PCINT0: usize = BIT0;
/// Bit 1 - Pin Change Enable Mask 1
const PCINT1: usize = BIT1;
/// Bit 2 - Pin Change Enable Mask 2
const PCINT2: usize = BIT2;
/// Bit 3 - Pin Change Enable Mask 3
const PCINT3: usize = BIT3;
/// Bit 4 - Pin Change Enable Mask 4
const PCINT4: usize = BIT4;
/// Bit 5 - Pin Change Enable Mask 5
const PCINT5: usize = BIT5;
/// Bit 6 - Pin Change Enable Mask 6
const PCINT6: usize = BIT6;
/// Bit 7 - Pin Change Enable Mask 7
const PCINT7: usize = BIT7;

/// PCMSK1 - Pin Change Mask Register 1
const PCMSK1_ADDR: usize = 0x6C;
/// Bit 0 - Pin Change Enable Mask 8
const PCINT8: usize = BIT0;
/// Bit 1 - Pin Change Enable Mask 9
const PCINT9: usize = BIT1;
/// Bit 2 - Pin Change Enable Mask 10
const PCINT10: usize = BIT2;
/// Bit 3 - Pin Change Enable Mask 11
const PCINT11: usize = BIT3;
/// Bit 4 - Pin Change Enable Mask 12
const PCINT12: usize = BIT4;
/// Bit 5 - Pin Change Enable Mask 13
const PCINT13: usize = BIT5;
/// Bit 6 - Pin Change Enable Mask 14
const PCINT14: usize = BIT6;

/// PCMSK2 - Pin Change Mask Register 2
const PCMSK2_ADDR: usize = 0x6D;
/// Bit 0 - Pin Change Enable Mask 16
const PCINT16: usize = BIT0;
/// Bit 1 - Pin Change Enable Mask 17
const PCINT17: usize = BIT1;
/// Bit 2 - Pin Change Enable Mask 18
const PCINT18: usize = BIT2;
/// Bit 3 - Pin Change Enable Mask 19
const PCINT19: usize = BIT3;
/// Bit 4 - Pin Change Enable Mask 20
const PCINT20: usize = BIT4;
/// Bit 5 - Pin Change Enable Mask 21
const PCINT21: usize = BIT5;
/// Bit 6 - Pin Change Enable Mask 22
const PCINT22: usize = BIT6;
/// Bit 7 - Pin Change Enable Mask 23
const PCINT23: usize = BIT7;

/// TIMSK0 - Timer/Counter 0 Interrupt Mask Register
const TIMSK0_ADDR: usize = 0x6E;
/// Bit 0 - Timer/Counter0 Overflow Interrupt Enable
const TOIE0: usize = BIT0;
/// Bit 1 - Timer/Counter0 Output Compare Match A Interrupt Enable
const OCIE0A: usize = BIT1;
/// Bit 2 - Timer/Counter0 Output Compare Match B Interrupt Enable
const OCIE0B: usize = BIT2;

/// TIMSK1 - Timer/Counter 1 Interrupt Mask Register
const TIMSK1_ADDR: usize = 0x6F;
/// Bit 0 - Timer/Counter1 Overflow Interrupt Enable
const TOIE1: usize = BIT0;
/// Bit 1 - Timer/Counter1 Output Compare Match A Interrupt Enable
const OCIE1A: usize = BIT1;
/// Bit 2 - Timer/Counter1 Output Compare Match B Interrupt Enable
const OCIE1B: usize = BIT2;
/// Bit 5 - Timer/Counter1 Input Capture Interrupt Enable
const ICIE1: usize = BIT5;

/// TIMSK2 - Timer/Counter 2 Interrupt Mask Register
const TIMSK2_ADDR: usize = 0x70;
/// Bit 0 - Timer/Counter2 Overflow Interrupt Enable
const TOIE2: usize = BIT0;
/// Bit 1 - Timer/Counter2 Output Compare Match A Interrupt Enable
const OCIE2A: usize = BIT1;
/// Bit 2 - Timer/Counter2 Output Compare Match B Interrupt Enable
const OCIE2B: usize = BIT2;

/// ADCL - ADC Data Register Low Byte
const ADCL_ADDR: usize = 0x78;
/// ADCH - ADC Data Register High Byte
const ADCH_ADDR: usize = 0x79;

/// ADCSRA - ADC Control and Status Register A
const ADCSRA_ADDR: usize = 0x7A;
/// Bit 0 - ADC Prescaler Select Bit 0
const ADPS0: usize = BIT0;
/// Bit 1 - ADC Prescaler Select Bit 1
const ADPS1: usize = BIT1;
/// Bit 2 - ADC Prescaler Select Bit 2
const ADPS2: usize = BIT2;
/// Bit 3 - ADC Interrupt Enable
const ADIE: usize = BIT3;
/// Bit 4 - ADC Interrupt Flag
const ADIF: usize = BIT4;
/// Bit 5 - ADC Auto Trigger Enable
const ADATE: usize = BIT5;
/// Bit 6 - ADC Start Conversion
const ADSC: usize = BIT6;
/// Bit 7 - ADC Enable
const ADEN: usize = BIT7;

/// ADCSRB - ADC Control and Status Register B
const ADCSRB_ADDR: usize = 0x7B;
/// Bit 0 - ADC Auto Trigger Source Bit 0
const ADTS0: usize = BIT0;
/// Bit 1 - ADC Auto Trigger Source Bit 1
const ADTS1: usize = BIT1;
/// Bit 2 - ADC Auto Trigger Source Bit 2
const ADTS2: usize = BIT2;
/// Bit 6 - Analog Comparator Multiplexer Enable
const ACME: usize = BIT6;

/// ADMUX - ADC Multiplexer Selection Register
const ADMUX_ADDR: usize = 0x7C;
/// Bit 0 - Analog Channel Selection Bit 0
const MUX0: usize = BIT0;
/// Bit 1 - Analog Channel Selection Bit 1
const MUX1: usize = BIT1;
/// Bit 2 - Analog Channel Selection Bit 2
const MUX2: usize = BIT2;
/// Bit 3 - Analog Channel Selection Bit 3
const MUX3: usize = BIT3;
/// Bit 5 - ADC Left Adjust Result
const ADLAR: usize = BIT5;
/// Bit 6 - Reference Selection Bit 0
const REFS0: usize = BIT6;
/// Bit 7 - Reference Selection Bit 1
const REFS1: usize = BIT7;

/// DIDR0 - Digital Input Disable Register 0
const DIDR0_ADDR: usize = 0x7E;
/// Bit 0 - Digital Input Disable on ADC0
const ADC0D: usize = BIT0;
/// Bit 1 - Digital Input Disable on ADC1
const ADC1D: usize = BIT1;
/// Bit 2 - Digital Input Disable on ADC2
const ADC2D: usize = BIT2;
/// Bit 3 - Digital Input Disable on ADC3
const ADC3D: usize = BIT3;
/// Bit 4 - Digital Input Disable on ADC4
const ADC4D: usize = BIT4;
/// Bit 5 - Digital Input Disable on ADC5
const ADC5D: usize = BIT5;

/// DIDR1 - Digital Input Disable Register 1
const DIDR1_ADDR: usize = 0x7F;
/// Bit 0 - Digital Input Disable on AIN0
const AIN0D: usize = BIT0;
/// Bit 1 - Digital Input Disable on AIN1
const AIN1D: usize = BIT1;

/// TCCR1A - Timer/Counter 1 Control Register A
const TCCR1A_ADDR: usize = 0x80;
/// Bit 0 - Waveform Generation Mode Bit 0
const WGM10: usize = BIT0;
/// Bit 1 - Waveform Generation Mode Bit 1
const WGM11: usize = BIT1;
/// Bit 4 - Compare Output Mode for Channel B Bit 0
const COM1B0: usize = BIT4;
/// Bit 5 - Compare Output Mode for Channel B Bit 1
const COM1B1: usize = BIT5;
/// Bit 6 - Compare Output Mode for Channel A Bit 0
const COM1A0: usize = BIT6;
/// Bit 7 - Compare Output Mode for Channel A Bit 1
const COM1A1: usize = BIT7;

/// TCCR1A - Timer/Counter 1 Control Register B
const TCCR1B_ADDR: usize = 0x81;
/// Bit 0 - Clock Select Bit 0
const CS10: usize = BIT0;
/// Bit 1 - Clock Select Bit 1
const CS11: usize = BIT1;
/// Bit 2 - Clock Select Bit 2
const CS12: usize = BIT2;
/// Bit 3 - Waveform Generation Mode Bit 2
const WGM12: usize = BIT3;
/// Bit 4 - Waveform Generation Mode Bit 3
const WGM13: usize = BIT4;
/// Bit 6 - Input Capture Edge Select
const ICES1: usize = BIT6;
/// Bit 7 - Input Capture Noise Canceler
const ICNC1: usize = BIT7;

/// TCCR1A - Timer/Counter 1 Control Register C
const TCCR1C_ADDR: usize = 0x82;
/// Bit 6 - Force Output Compare for Channel B
const FOC1B: usize = BIT6;
/// Bit 7 - Force Output Compare for Channel A
const FOC1A: usize = BIT7;

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
/// Bit 0 - Waveform Generation Mode Bit 0
const WGM20: usize = BIT0;
/// Bit 1 - Waveform Generation Mode Bit 1
const WGM21: usize = BIT1;
/// Bit 4 - Compare Output Mode for Channel B Bit 0
const COM2B0: usize = BIT4;
/// Bit 5 - Compare Output Mode for Channel B Bit 1
const COM2B1: usize = BIT5;
/// Bit 6 - Compare Output Mode for Channel A Bit 0
const COM2A0: usize = BIT6;
/// Bit 7 - Compare Output Mode for Channel A Bit 1
const COM2A1: usize = BIT7;

/// TCCR2B - Timer/Counter 2 Control Register B
const TCCR2B_ADDR: usize = 0xB1;
/// Bit 0 - Clock Select Bit 0
const CS20: usize = BIT0;
/// Bit 1 - Clock Select Bit 1
const CS21: usize = BIT1;
/// Bit 2 - Clock Select Bit 2
const CS22: usize = BIT2;
/// Bit 3 - Waveform Generation Mode Bit 2
const WGM22: usize = BIT3;
/// Bit 6 - Force Output Compare for Channel B
const FOC2B: usize = BIT6;
/// Bit 7 - Force Output Compare for Channel A
const FOC2A: usize = BIT7;

/// TCNT2 - Timer/Counter 2 Register
const TCNT2_ADDR: usize = 0xB2;

/// OCR2A - Timer/Counter 2 Output Compare Register A
const OCR2A_ADDR: usize = 0xB3;

/// OCR2B - Timer/Counter 2 Output Compare
const OCR2B_ADDR: usize = 0xB4;

/// ASSR - Asynchronous Status Register
const ASSR_ADDR: usize = 0xB6;
/// Bit 0 - Timer/Counter Control Register 2B Update Busy
const TCR2BUB: usize = BIT0;
/// Bit 1 - Timer/Counter Control Register 2A Update Busy
const TCR2AUB: usize = BIT1;
/// Bit 2 - Output Compare Register 2B Update Busy
const OCR2BUB: usize = BIT2;
/// Bit 3 - Output Compare Register 2A Update Busy
const OCR2AUB: usize = BIT3;
/// Bit 4 - Timer/Counter2 Update Busy
const TCN2UB: usize = BIT4;
/// Bit 5 - Asynchronous Timer/Counter2
const AS2: usize = BIT5;
/// Bit 6 - Enable External Clock Input
const EXCLK: usize = BIT6;

/// TWBR - TWI Bit Rate Register
const TWBR_ADDR: usize = 0xB8;

/// TWSR - TWI Status Register
const TWSR_ADDR: usize = 0xB9;
/// Bit 0 - TWI Prescaler Bit 0
const TWPS0: usize = BIT0;
/// Bit 1 - TWI Prescaler Bit 1
const TWPS1: usize = BIT1;
/// Bit 3 - TWI Status Bit 3
const TWS3: usize = BIT3;
/// Bit 4 - TWI Status Bit 4
const TWS4: usize = BIT4;
/// Bit 5 - TWI Status Bit 5
const TWS5: usize = BIT5;
/// Bit 6 - TWI Status Bit 6
const TWS6: usize = BIT6;
/// Bit 7 - TWI Status Bit 7
const TWS7: usize = BIT7;

/// TWAR - TWI (Slave) Address Register
const TWAR_ADDR: usize = 0xBA;
/// Bit 0 - TWI General Call Recognition Enable
const TWGCE: usize = BIT0;
/// Bit 1 - TWI (Slave) Address Bit 0
const TWA0: usize = BIT1;
/// Bit 2 - TWI (Slave) Address Bit 1
const TWA1: usize = BIT2;
/// Bit 3 - TWI (Slave) Address Bit 2
const TWA2: usize = BIT3;
/// Bit 4 - TWI (Slave) Address Bit 3
const TWA3: usize = BIT4;
/// Bit 5 - TWI (Slave) Address Bit 4
const TWA4: usize = BIT5;
/// Bit 6 - TWI (Slave) Address Bit 5
const TWA5: usize = BIT6;
/// Bit 7 - TWI (Slave) Address Bit 6
const TWA6: usize = BIT7;

/// TWDR - TWI Data Register
const TWDR_ADDR: usize = 0xBB;

/// TWCR - TWI Control Register
const TWCR_ADDR: usize = 0xBC;
/// Bit 0 - TWI Interrupt Enable
const TWIE: usize = BIT0;
/// Bit 2 - TWI Enable
const TWEN: usize = BIT2;
/// Bit 3 - TWI Write Collision Flag
const TWWC: usize = BIT3;
/// Bit 4 - TWI Stop Condition Bit
const TWSTO: usize = BIT4;
/// Bit 5 - TWI Start Condition Bit
const TWSTA: usize = BIT5;
/// Bit 6 - TWI Enable Acknowledge Bit
const TWEA: usize = BIT6;
/// Bit 7 - TWI Interrupt Flag
const TWINT: usize = BIT7;

/// TWAMR - TWI (Slave) Address Mask Register
const TWAMR_ADDR: usize = 0xBD;
/// Bit 1 - TWI Address Mask Bit 0
const TWAM0: usize = BIT1;
/// Bit 2 - TWI Address Mask Bit 1
const TWAM1: usize = BIT2;
/// Bit 3 - TWI Address Mask Bit 2
const TWAM2: usize = BIT3;
/// Bit 4 - TWI Address Mask Bit 3
const TWAM3: usize = BIT4;
/// Bit 5 - TWI Address Mask Bit 4
const TWAM4: usize = BIT5;
/// Bit 6 - TWI Address Mask Bit 5
const TWAM5: usize = BIT6;
/// Bit 7 - TWI Address Mask Bit 6
const TWAM6: usize = BIT7;

/// UCSR0A - USART Control and Status Register A
const UCSR0A_ADDR: usize = 0xC0;
/// Bit 0 - Multi-processor Communication Mode
const MPCM0: usize = BIT0;
/// Bit 1 - Double the USART Transmission Speed
const U2X0: usize = BIT1;
/// Bit 2 - Parity Error
const UPE0: usize = BIT2;
/// Bit 3 - Data OverRun
const DOR0: usize = BIT3;
/// Bit 4 - Frame Error
const FE0: usize = BIT4;
/// Bit 5 - Data Register Empty
const UDRE0: usize = BIT5;
/// Bit 6 - Transmit Complete
const TXC0: usize = BIT6;
/// Bit 7 - Receive Complete
const RXC0: usize = BIT7;

/// UCSROB - USART Control and Status Register B
const UCSR0B_ADDR: usize = 0xC1;
/// Bit 0 - Transmit Data Bit 8
const TXB80: usize = BIT0;
/// Bit 1 - Receive Data Bit 8
const RXB80: usize = BIT1;
/// Bit 2 - Character Size Bit 2
const UCSZ02: usize = BIT2;
/// Bit 3 - Transmit Enable
const TXEN0: usize = BIT3;
/// Bit 4 - Receive Enable
const RXEN0: usize = BIT4;
/// Bit 5 - USART Data Register Empty Interrupt Enable
const UDRIE0: usize = BIT5;
/// Bit 6 - Transmit Complete Interrupt Enable
const TXCIE0: usize = BIT6;
/// Bit 7 - Receive Complete Interrupt Enable
const RXCIE0: usize = BIT7;

/// UCSROC - USART Control and Status Register C
const UCSR0C_ADDR: usize = 0xC2;
/// Bit 0 - Clock Polarity
const UCPOL0: usize = BIT0;
/// Bit 1 - Clock Phase
const UCPHA0: usize = BIT1;
/// Character Size Bit 0
const UCSZ00: usize = BIT1;
/// Bit 2 - Data Order
const UDORD0: usize = BIT2;
/// Character Size Bit 1
const UCSZ01: usize = BIT2;
/// Bit 3 - Stop Bit Select
const USBS0: usize = BIT3;
/// Bit 4 - Parity Mode Bit 0
const UPM00: usize = BIT4;
/// Bit 5 - Parity Mode Bit 1
const UPM01: usize = BIT5;
/// Bit 6 - USART Mode Select Bit 0
const UMSEL00: usize = BIT6;
/// Bit 7 - USART Mode Select Bit 1
const UMSEL01: usize = BIT7;

/// UBRR0L - USART Baud Rate Register Low Byte
const UBRR0L_ADDR: usize = 0xC4;
/// UBRR0H - USART Baud Rate Register High Byte
const UBRR0H_ADDR: usize = 0xC5;

/// UDR0 - USART I/O Data Register
const UDR0_ADDR: usize = 0xC6;
