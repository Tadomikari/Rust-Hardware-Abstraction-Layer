use super::{PinMode, PinValue, GPIO};

const GPIOA_MODER: *mut u32 = 0x48000000u32 as *mut u32; // Mode register
const GPIOA_ODR: *mut u32 = 0x48000014u32 as *mut u32;   // Output data register
const GPIOA_IDR: *mut u32 = 0x48000010u32 as *mut u32;   // Input data register

pub struct CortexM3;

// We will indicate volatile access for memory-mapped registers to ensure they are accessed in a way that prevents 
// compiler optimizations from reordering or omitting them

impl GPIO for CortexM3 {

    // Sets the pin as input or output by respectively clearing or setting the corresponding bit in MODER
    fn configure_pin(pin: u8, mode: PinMode) {
        unsafe {
            let shift = pin * 2; // Each pin uses 2 bits in the MODER register
            match mode {
                PinMode::Input => {
                    // Clears the 2 bits for the pin to set it as input (00)
                    core::ptr::write_volatile(
                        GPIOA_MODER,
                        core::ptr::read_volatile(GPIOA_MODER) & !(0b11 << shift),
                    );
                }
                PinMode::Output => {
                    // Sets the 2 bits for the pin to configure it as output (01)
                    core::ptr::write_volatile(
                        GPIOA_MODER,
                        (core::ptr::read_volatile(GPIOA_MODER) & !(0b11 << shift)) | (0b01 << shift),
                    );
                }
            }
        }
    }

    // Writes a HIGH or LOW value to the specified pin by modifying the GPIOA_ODR register
    fn write_pin(pin: u8, value: PinValue) {
        unsafe {
            match value {
                PinValue::High => {
                    // Sets the corresponding bit in the ODR register to set the pin to HIGH
                    core::ptr::write_volatile(
                        GPIOA_ODR,
                        core::ptr::read_volatile(GPIOA_ODR) | (1 << pin),
                    );
                }
                PinValue::Low => {
                    // Clears the corresponding bit in the ODR register to set the pin to LOW
                    core::ptr::write_volatile(
                        GPIOA_ODR,
                        core::ptr::read_volatile(GPIOA_ODR) & !(1 << pin),
                    );
                }
            }
        }
    }

    // Reads the state (HIGH or LOW) of the specified pin from the GPIOA_IDR register
    fn read_pin(pin: u8) -> PinValue {
        unsafe {
            if core::ptr::read_volatile(GPIOA_IDR) & (1 << pin) != 0 {
                PinValue::High
            } else {
                PinValue::Low
            }
        }
    }
}
