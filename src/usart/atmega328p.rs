use super::USART;

const UBRR0H: *mut u8 = 0xC5 as *mut u8;    // High byte of the baud rate register
const UBRR0L: *mut u8 = 0xC4 as *mut u8;    // Low byte of the baud rate register
const UCSR0A: *mut u8 = 0xC0 as *mut u8;    // USART Control and Status Register A: contains the status of the USART
const UCSR0B: *mut u8 = 0xC1 as *mut u8;    // USART Control and Status Register B: activates or deactivates transmission and reception
const UCSR0C: *mut u8 = 0xC2 as *mut u8;    // USART Control and Status Register C: configures the frame format
const UDR0: *mut u8 = 0xC6 as *mut u8;      // USART I/O Data Register

const TX_ENABLE: u8 = 1 << 3; // Transmitter Enable (bit 3 of UCSR0B)
const RX_ENABLE: u8 = 1 << 4; // Receiver Enable (bit 4 of UCSR0B)
const FRAME_FORMAT: u8 = (1 << 1) | (1 << 2); // 8 data bits, 1 stop bit

pub struct Atmega328p;

impl USART for Atmega328p {
    // Initializes the USART with the given baud rate and frame format, enabling transmission and reception
    fn usart_init(baud_rate: u32) {
        let ubrr_value = (16_000_000 / (16 * baud_rate) - 1) as u16; // Calculate baud rate value
        unsafe {
            *UBRR0H = (ubrr_value >> 8) as u8;  // Sets high byte of UBRR
            *UBRR0L = ubrr_value as u8;         // Sets low byte of UBRR
            *UCSR0B = TX_ENABLE | RX_ENABLE;    
            *UCSR0C = FRAME_FORMAT;             
        }
    }

    // Waits until the transmit buffer is ready to emit data, then sends the data
    fn usart_write(data: u8) {
        unsafe {
            while *UCSR0A & (1 << 5) == 0 {} // Wait for transmit buffer bit to be set to 1 (ready to emit)
            *UDR0 = data; // Data is written into the buffer to be sent
        }
    }

    // Waits until data is received, then reads the data from the receive buffer
    fn usart_read() -> u8 {
        unsafe {
            while *UCSR0A & (1 << 7) == 0 {} // if *UCSR0A == 1, data was received
            *UDR0 // Reads data from the receive buffer
        }
    }
}
