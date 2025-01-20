use super::SPI;

const SPCR: *mut u8 = 0x4C as *mut u8; // SPI Control Register
const SPSR: *mut u8 = 0x4D as *mut u8; // SPI Status Register
const SPDR: *mut u8 = 0x4E as *mut u8; // SPI Data Register

pub struct Atmega328p;

impl SPI for Atmega328p {
    // Initialize SPI as master
    fn spi_init_master() {
        const SPI_ENABLE: u8 = 1 << 6; // SPI Enable
        const SPI_MASTER: u8 = 1 << 4; // SPI Master Mode
        const SPI_CLOCK_DIV16: u8 = 1 << 1; // Clock rate = clockfrequency/16

        unsafe {
            *SPCR = SPI_ENABLE | SPI_MASTER | SPI_CLOCK_DIV16; //Configures SPI Control Register
            *SPSR = 0; //Clears SPI Status Register
        }
    }

    // Initialize SPI as slave
    fn spi_init_slave() {
        const SPI_ENABLE: u8 = 1 << 6; // SPI Enable
        const SPI_SLAVE: u8 = 0; // Clear MSTR bit for slave mode

        unsafe {
            *SPCR = SPI_ENABLE | SPI_SLAVE; //Configures SPI Control Register
            *SPSR = 0; //Clears SPI Status Register
        }
    }

    fn spi_write(data: u8) {
        unsafe {
            *SPDR = data; //Loads data into the SPI Data Register to start transmission
            while !is_transmission_complete() {}
        }
    }

    fn spi_read() -> u8 {
        unsafe {
            while !is_transmission_complete() {}
            *SPDR //Returns received data from the SPI Data Register
        }
    }

    // Simultaneously writes and reads data in slave mode
    fn spi_transfer(data: u8) -> u8 {
        unsafe {
            *SPDR = data; //Loads data into the SPI Data Register to start transmission
            while !is_transmission_complete() {}
            *SPDR //Returns received data from the SPI Data Register
        }
    }
}

fn is_transmission_complete() -> bool {
    unsafe { *SPSR & (1 << 7) != 0 } //Waits for the SPI Interrupt Flag to be set, indicating complete transmission
}
