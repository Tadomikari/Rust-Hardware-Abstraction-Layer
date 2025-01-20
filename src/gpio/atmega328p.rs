use super::{PinMode, PinValue, GPIO};

// Memory addresses for registers controlling the Data Direction (DDRB), Output (PORTB), and Input (PINB) of PORTB
const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *mut u8 = 0x23 as *mut u8;

pub struct Atmega328p;

// We will indicate volatile access for memory-mapped registers to ensure they are accessed in a way that prevents 
// compiler optimizations from reordering or omitting them

impl GPIO for Atmega328p{

    // Sets the pin as input or output by respectively clearing or setting the corresponding bit in DDRB
    fn configure_pin(pin: u8, mode: PinMode) {
        unsafe {
            match mode {
                PinMode::Input => core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) & !(1 << pin)),
                PinMode::Output => core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | (1 << pin)),
            }
        }
    }

    // Controls the output state (HIGH/LOW) of a pin by setting or clearing the corresponding bit in PORTB
    fn write_pin(pin: u8, value: PinValue) {
        unsafe {
            match value {
                PinValue::High => core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) | (1 << pin)),
                PinValue::Low => core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) & !(1 << pin)),
            }
        }
    }

    // Reads the state (HIGH/LOW) of a pin by checking its bit in PINB
    fn read_pin(pin: u8) -> PinValue {
        unsafe {
            if core::ptr::read_volatile(PINB) & (1 << pin) != 0 {
                PinValue::High
            } else {
                PinValue::Low
            }
        }
    }
}
