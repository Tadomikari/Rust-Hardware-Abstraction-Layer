use super::I2C;

const I2C_CR1: *mut u32 = 0x40005400u32 as *mut u32;
const I2C_CR2: *mut u32 = 0x40005404u32 as *mut u32;
const I2C_DR: *mut u32 = 0x40005410u32 as *mut u32;
const I2C_SR1: *mut u32 = 0x40005414u32 as *mut u32;
const I2C_SR2: *mut u32 = 0x40005418u32 as *mut u32;

// Control Register Bits
const I2C_CR1_PE: u32 = 1 << 0;    // Peripheral Enable
const I2C_CR1_START: u32 = 1 << 8; // Start Generation
const I2C_CR1_STOP: u32 = 1 << 9;  // Stop Generation
const I2C_CR1_ACK: u32 = 1 << 10; // Acknowledge Enable Bit

// Status Register Bits
const I2C_SR1_SB: u32 = 1 << 0;    // Start Bit
const I2C_SR1_ADDR: u32 = 1 << 1;  // Address Sent/Matched
const I2C_SR1_TXE: u32 = 1 << 7;   // Transmit Data Register Empty
const I2C_SR1_RXNE: u32 = 1 << 6;  // Receive Data Register Not Empty

pub struct CortexM3;

impl I2C for CortexM3 {
    fn i2c_init(clock_speed: u32) {
        const MAX_FREQ_MHZ: u32 = 36; // Maximum clock frequency in MHz for I2C
        let freq = clock_speed / 1_000_000;
    
        if freq > MAX_FREQ_MHZ {
            panic!("Clock speed too high for I2C peripheral!");
        }
    
        unsafe {
            *I2C_CR2 = freq & 0x3F; // Set frequency
            *I2C_CR1 |= I2C_CR1_PE; // Enable I2C
        }
    }
    

    fn i2c_write(address: u8, data: &[u8]) {
        unsafe {
            // Genreates start condition
            *I2C_CR1 |= I2C_CR1_START;
            while *I2C_SR1 & I2C_SR1_SB == 0 {}

            // Sends Slave Address with Write Bit
            *I2C_DR = (address << 1) as u32;
            while *I2C_SR1 & I2C_SR1_ADDR == 0 {}
            let _ = *I2C_SR2; // Clear ADDR bit by reading SR2

            // Writes data
            for &byte in data {
                *I2C_DR = byte as u32;
                while *I2C_SR1 & I2C_SR1_TXE == 0 {}
            }

            // Stop condition
            *I2C_CR1 |= I2C_CR1_STOP;
        }
    }
    

    fn i2c_read(address: u8, buffer: &mut [u8]) -> u8 {
        let mut last_byte = 0;
        let buffer_len = buffer.len();
        unsafe {
            // Start condition
            *I2C_CR1 |= I2C_CR1_START;
            while *I2C_SR1 & I2C_SR1_SB == 0 {}

            // Sends Slave Address with Read Bit
            *I2C_DR = ((address << 1) | 1) as u32;
            while *I2C_SR1 & I2C_SR1_ADDR == 0 {}
            let _ = *I2C_SR2;

            // Reads data
            for (i, byte) in buffer.iter_mut().enumerate() {
                if i == buffer_len - 1 {
                    *I2C_CR1 &= !I2C_CR1_ACK; // NACK for the last byte
                } else {
                    *I2C_CR1 |= I2C_CR1_ACK; // ACK for other bytes
                }
                while *I2C_SR1 & I2C_SR1_RXNE == 0 {}
                *byte = *I2C_DR as u8;
                last_byte = *byte; // Save the last byte read
            }
            // Stop condition
            *I2C_CR1 |= I2C_CR1_STOP;
        }
        last_byte // returns the last byte (or zero if the buffer is empty)
    }
 
    
            
    
}
