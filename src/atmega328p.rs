//! # ATmega328P
//!
//! The ATmega328P is an 8-bit AVR Microcontroller with the following features:
//!
//! - **32K Bytes In-System Programmable Flash**
//! - **1K Bytes EEPROM**
//! - **2K Bytes Internal SRAM**
//! - **Write/Erase Cycles**: 10,000 flash / 100,000 EEPROM
//! - **Optional Boot Code Section**
//!
//! ## Peripherals
//!
//! - **Two 8-bit Timer/Counters** with separate prescaler and compare mode
//! - **One 16-bit Timer/Counter** with separate prescaler, compare mode, and capture mode
//! - **RTC** with separate oscillator
//! - **6 PWM Channels**
//! - **ADC**
//! - **USART, SPI, TWI (I2C)**
//! - **Watchdog Timer**
//! - **Analog Comparator**
//! - **Interrupts** (internal and external)
//!
//! ## Special Microcontroller Features
//!
//! - **Power-on Reset** and **Programmable Brown-out Detection**
//! - **Six Sleep Modes**

mod vectors;
