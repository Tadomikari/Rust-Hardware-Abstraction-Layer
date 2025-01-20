use super::I2C;

const TWBR: *mut u8 = 0xB8 as *mut u8;  // TWI Bit Rate Register
const TWSR: *mut u8 = 0xB9 as *mut u8;  // TWI Status Register
const TWAR: *mut u8 = 0xBA as *mut u8;  // TWI (Slave) Address Register
const TWDR: *mut u8 = 0xBB as *mut u8;  // TWI Data Register
const TWCR: *mut u8 = 0xBC as *mut u8;  // TWI Control Register

// Control Register Bits
const TWINT: u8 = 1 << 7; // TWI Interrupt Flag
const TWSTA: u8 = 1 << 5; // TWI Start Condition Bit
const TWSTO: u8 = 1 << 4; // TWI Stop Condition Bit
const TWEN: u8 = 1 << 2; // TWI Enable Bit
const TWEA: u8 = 1 << 6; // TWI Enable Acknowledge Bit

pub struct Atmega328p;

impl I2C for Atmega328p {
    fn i2c_init(clock_speed: u32) {
        const CPU_CLOCK: u32 = 16_000_000; // CPU clock frequency
        let prescaler: u8 = 1;             // Prescaler value (can be modified if needed)
        // Calculate the TWPS bits based on the prescaler value
        let twps_bits = match prescaler {
            1 => 0b00, // TWPS bits for prescaler = 1
            4 => 0b01, // TWPS bits for prescaler = 4
            16 => 0b10, // TWPS bits for prescaler = 16
            64 => 0b11, // TWPS bits for prescaler = 64
            _ => 0b00, // Default to prescaler = 1
        };

        unsafe {
            // Set the prescaler in TWSR
            *TWSR = (*TWSR & !0b11) | twps_bits;
    
            // Calculate and set bit rate
            let bit_rate = ((CPU_CLOCK / clock_speed) - 16) / (2 * prescaler) as u32;
            if bit_rate < 10 {
                panic!("Invalid clock_speed: Bit rate too low!");
            }
            *TWBR = bit_rate as u8;
    
            // Enable TWI
            *TWCR = TWEN; // TWI Enable
        }
    }

    fn i2c_write(address: u8, data: &[u8]) {
        unsafe {
            // Sends start condition
            *TWCR = TWINT | TWSTA | TWEN;  
            while *TWCR & TWINT == 0 {}

            // Sends address
            *TWDR = (address << 1) & 0xFE; // Address and write bit
            *TWCR = TWINT | TWEN; // Clears TWINT to start transmission
            while *TWCR & TWINT == 0 {} 

            // Writes data
            for &byte in data {
                *TWDR = byte;
                *TWCR = TWINT | TWEN;
                while *TWCR & TWINT == 0 {}
            }

            // Sends stop condition
            *TWCR = TWINT | TWSTO | TWEN;
        }
    }
    
    fn i2c_read(address: u8, buffer: &mut [u8]) -> u8{
        let mut last_byte = 0;
        let buffer_len = buffer.len();
        unsafe {
            // Sends start condition
            *TWCR = TWINT | TWSTA | TWEN;
            while *TWCR & TWINT == 0 {}

            // Sends address
            *TWDR = (address << 1) | 1; // Adress and read bit
            *TWCR = TWINT | TWEN; // Clears TWINT to start transmission
            while *TWCR & TWINT == 0 {}

            // Reads data
            for (i, byte) in buffer.iter_mut().enumerate() {
                if i == buffer_len - 1{
                    *TWCR = TWINT | TWEN; // NACK for the last byte
                } else {
                    *TWCR = TWINT | TWEN | TWEA; // ACK for all other bytes
                }
                while *TWCR & TWINT == 0 {}
                *byte = *TWDR;
                last_byte = *byte; // Save the last byte read
            }
            // Sends stop condition
            *TWCR = TWINT | TWSTO | TWEN;
        }
        last_byte // returns the last byte (or zero if the buffer is empty)
    }
}
