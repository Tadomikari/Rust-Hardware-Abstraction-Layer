pub mod atmega328p;
pub mod cortex_m3;

pub enum PinMode {
    Input,
    Output,
}

pub enum PinValue {
    High,
    Low,
}

pub trait GPIO {
    fn configure_pin(pin: u8, mode: PinMode);
    fn read_pin(pin: u8) -> PinValue;
    fn write_pin(pin: u8, value: PinValue);
}

#[cfg(feature = "atmega328p")]
pub type ActiveGPIO = atmega328p::Atmega328p;

#[cfg(feature = "cortex_m3")]
pub type ActiveGPIO = cortex_m3::CortexM3;

pub fn configure_pin(pin: u8, mode: PinMode) {
    ActiveGPIO::configure_pin(pin, mode);
}

pub fn read_pin(pin: u8) -> PinValue {
    ActiveGPIO::read_pin(pin)
}

pub fn write_pin(pin: u8, value: PinValue) {
    ActiveGPIO::write_pin(pin, value);
}

