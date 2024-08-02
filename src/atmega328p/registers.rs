//! Register/Port Description
//! Datasheet page 275 et seqq.

/// Constants
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

/// The Port B is 8-bit wide
/// PINB - The Port B Input Pins Address (Read Only)
const PINB_ADDR: usize = 0x23;
const PINB_ADDR_IO: usize = 0x03;
/// DDRB - The Port B Data Direction Register
const DDRB_ADDR: usize = 0x24;
const DDRB_ADDR_IO: usize = 0x04;
/// PORTB - The Port B Data Register
const PORTB_ADDR: usize = 0x25;
const PORTB_ADDR_IO: usize = 0x05;

/// The Port C is 7-bit wide
/// PINC - The Port C Input Pins Address (Read Only)
const PINC_ADDR: usize = 0x26;
const PINC_ADDR_IO: usize = 0x06;
/// DDRC - The Port C Data Direction Register
const DDRC_ADDR: usize = 0x27;
const DDRC_ADDR_IO: usize = 0x07;
/// PORTC - The Port C Data Register
const PORTC_ADDR: usize = 0x28;
const PORTC_ADDR_IO: usize = 0x08;

/// The Port D is 8-bit wide
/// PIND - The Port D Input Pins Address (Read Only)
const PIND_ADDR: usize = 0x29;
const PIND_ADDR_IO: usize = 0x09;
/// DDRD - The Port D Data Direction Register
const DDRD_ADDR: usize = 0x2A;
const DDRD_ADDR_IO: usize = 0x0A;
/// PORTD - The Port D Data Register
const PORTD_ADDR: usize = 0x2B;
const PORTD_ADDR_IO: usize = 0x0B;

/// The Timer/Counter 0 is 8-bit wide
/// TIFR0 - Timer/Counter 0 Interrupt Flag Register
const TIFR0_ADDR: usize = 0x35;
const TIFR0_ADDR_IO: usize = 0x15;

/// The Timer/Counter 1 is 16-bit wide
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

/// The Timer/Counter 2 is 8-bit wide
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
