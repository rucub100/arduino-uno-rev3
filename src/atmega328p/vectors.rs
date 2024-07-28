/// Interrupt Vectors in ATmega328P

#[allow(dead_code)]
const IVT_SIZE: usize = 0x1a;

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

/// TIMER1 OVF - Timer/Counter1 Overflow
#[no_mangle]
#[export_name = "__vector_13"]
pub extern "avr-interrupt" fn timer1_ovf_isr() {
    // Your ISR code here
}
