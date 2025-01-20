use super::USART;

const USART2_SR: *mut u32 = 0x40004400u32 as *mut u32; // Status Register
const USART2_DR: *mut u32 = 0x40004404u32 as *mut u32;   // Data Register
const USART2_BRR: *mut u32 = 0x40004408u32 as *mut u32;  // Baud Rate Register
const USART2_CR1: *mut u32 = 0x4000440Cu32 as *mut u32;  // Control Register 1

const TXE_BIT: u32 = 1 << 7;    // Transmit Data Register Empty bit from SR
const RXNE_BIT: u32 = 1 << 5;   // Read Data Register Not Empty bit from SR

pub struct CortexM3;

impl USART for CortexM3 {
    // Initializes the USART with the given baud rate, enabling transmission and reception
    fn usart_init(baud_rate: u32) {
        let baud_div = 16_000_000 / baud_rate;  //16_000_000 is the clock rate
        unsafe {
            *USART2_BRR = baud_div; //We set the baud rate
            *USART2_CR1 = (1 << 3) | (1 << 2) | (1 << 13);  //Enables transmission (TX), reception (RX) and USART
        }
    }

    // Waits until Transmit Data Register Empty bit is 1 to write data in DR
    fn usart_write(data: u8) {
        unsafe {
            while *USART2_SR & TXE_BIT == 0 {} 
            *USART2_DR = data as u32;
        }
    }

    // Waits until Read Data Register Not Empty bit is 1 to read data from DR
    fn usart_read() -> u8 {
        unsafe {
            while *USART2_SR & RXNE_BIT == 0 {}
            *USART2_DR as u8 
        }
    }
}
