/// Interrupt Vectors in ATmega328P
/// See page 49 in ATmega328P datasheet

/// Reset + 25 Interrupt Vectors
#[allow(dead_code)]
const IVT_SIZE: usize = 26;

// /// INT0 - External Interrupt Request 0
// #[no_mangle]
// #[export_name = "__vector_1"]
// pub extern "avr-interrupt" fn int0_isr() {
//     // Your ISR code here
// }

// /// INT1 - External Interrupt Request 1
// #[no_mangle]
// #[export_name = "__vector_2"]
// pub extern "avr-interrupt" fn int1_isr() {
//     // Your ISR code here
// }

// /// PCINT0 - Pin Change Interrupt Request 0
// #[no_mangle]
// #[export_name = "__vector_3"]
// pub extern "avr-interrupt" fn pcint0_isr() {
//     // Your ISR code here
// }

// /// PCINT1 - Pin Change Interrupt Request 1
// #[no_mangle]
// #[export_name = "__vector_4"]
// pub extern "avr-interrupt" fn pcint1_isr() {
//     // Your ISR code here
// }

// /// PCINT2 - Pin Change Interrupt Request 2
// #[no_mangle]
// #[export_name = "__vector_5"]
// pub extern "avr-interrupt" fn pcint2_isr() {
//     // Your ISR code here
// }

// /// WDT - Watchdog Time-out Interrupt
// #[no_mangle]
// #[export_name = "__vector_6"]
// pub extern "avr-interrupt" fn wdt_isr() {
//     // Your ISR code here
// }

// /// TIMER2 COMPA - Timer/Counter2 Compare Match A
// #[no_mangle]
// #[export_name = "__vector_7"]
// pub extern "avr-interrupt" fn timer2_compa_isr() {
//     // Your ISR code here
// }

// /// TIMER2 COMPB - Timer/Counter2 Compare Match B
// #[no_mangle]
// #[export_name = "__vector_8"]
// pub extern "avr-interrupt" fn timer2_compb_isr() {
//     // Your ISR code here
// }

// /// TIMER2 OVF - Timer/Counter2 Overflow
// #[no_mangle]
// #[export_name = "__vector_9"]
// pub extern "avr-interrupt" fn timer2_ovf_isr() {
//     // Your ISR code here
// }

// /// TIMER1 CAPT - Timer/Counter1 Capture Event
// #[no_mangle]
// #[export_name = "__vector_10"]
// pub extern "avr-interrupt" fn timer1_capt_isr() {
//     // Your ISR code here
// }

// /// TIMER1 COMPA - Timer/Counter1 Compare Match A
// #[no_mangle]
// #[export_name = "__vector_11"]
// pub extern "avr-interrupt" fn timer1_compa_isr() {
//     // Your ISR code here
// }

// /// TIMER1 COMPB - Timer/Counter1 Compare Match B
// #[no_mangle]
// #[export_name = "__vector_12"]
// pub extern "avr-interrupt" fn timer1_compb_isr() {
//     // Your ISR code here
// }

// /// TIMER1 OVF - Timer/Counter1 Overflow
// #[no_mangle]
// #[export_name = "__vector_13"]
// pub extern "avr-interrupt" fn timer1_ovf_isr() {
//     // Your ISR code here
// }

// /// TIMER0 COMPA - Timer/Counter0 Compare Match A
// #[no_mangle]
// #[export_name = "__vector_14"]
// pub extern "avr-interrupt" fn timer0_compa_isr() {
//     // Your ISR code here
// }

// /// TIMER0 COMPB - Timer/Counter0 Compare Match B
// #[no_mangle]
// #[export_name = "__vector_15"]
// pub extern "avr-interrupt" fn timer0_compb_isr() {
//     // Your ISR code here
// }

// /// TIMER0 OVF - Timer/Counter0 Overflow
// #[no_mangle]
// #[export_name = "__vector_16"]
// pub extern "avr-interrupt" fn timer0_ovf_isr() {
//     // Your ISR code here
// }

// /// SPI, STC - SPI Serial Transfer Complete
// #[no_mangle]
// #[export_name = "__vector_17"]
// pub extern "avr-interrupt" fn spi_stc_isr() {
//     // Your ISR code here
// }

// /// USART, RX - USART Rx Complete
// #[no_mangle]
// #[export_name = "__vector_18"]
// pub extern "avr-interrupt" fn usart_rx_isr() {
//     // Your ISR code here
// }

// /// USART, UDRE - USART Data Register Empty
// #[no_mangle]
// #[export_name = "__vector_19"]
// pub extern "avr-interrupt" fn usart_udre_isr() {
//     // Your ISR code here
// }

// /// USART, TX - USART Tx Complete
// #[no_mangle]
// #[export_name = "__vector_20"]
// pub extern "avr-interrupt" fn usart_tx_isr() {
//     // Your ISR code here
// }

// /// ADC - ADC Conversion Complete
// #[no_mangle]
// #[export_name = "__vector_21"]
// pub extern "avr-interrupt" fn adc_isr() {
//     // Your ISR code here
// }

// /// EE READY - EEPROM Ready
// #[no_mangle]
// #[export_name = "__vector_22"]
// pub extern "avr-interrupt" fn ee_ready_isr() {
//     // Your ISR code here
// }

// /// ANALOG COMP - Analog Comparator
// #[no_mangle]
// #[export_name = "__vector_23"]
// pub extern "avr-interrupt" fn analog_comp_isr() {
//     // Your ISR code here
// }

// /// TWI - 2-wire Serial Interface (I2C)
// #[no_mangle]
// #[export_name = "__vector_24"]
// pub extern "avr-interrupt" fn twi_isr() {
//     // Your ISR code here
// }

// /// SPM READY - Store Program Memory Ready
// #[no_mangle]
// #[export_name = "__vector_25"]
// pub extern "avr-interrupt" fn spm_ready_isr() {
//     // Your ISR code here
// }
