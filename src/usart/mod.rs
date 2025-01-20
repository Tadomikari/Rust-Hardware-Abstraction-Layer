pub mod atmega328p;
pub mod cortex_m3;

// USART trait defines the interface for USART operations
pub trait USART {
    fn usart_init(baud_rate: u32);
    fn usart_write(data: u8);
    fn usart_read() -> u8;
}

#[cfg(feature = "atmega328p")]
pub type ActiveUSART = atmega328p::Atmega328p;

#[cfg(feature = "cortex_m3")]
pub type ActiveUSART = cortex_m3::CortexM3;

// Public functions to initialize, write, and read using USART
pub fn usart_init(baud_rate: u32) {
    ActiveUSART::usart_init(baud_rate);
}

pub fn usart_write(data: u8) {
    ActiveUSART::usart_write(data);
}

pub fn usart_read() -> u8 {
    ActiveUSART::usart_read()
}

