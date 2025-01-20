use super::SPI;

const SPI1_BASE: u32 = 0x40013000u32; // Base address of SPI1 peripheral
const SPI1_CR1: *mut u32 = (SPI1_BASE + 0x00) as *mut u32; // Control Register 1
const SPI1_SR: *mut u32 = (SPI1_BASE + 0x08) as *mut u32;  // Status Register
const SPI1_DR: *mut u32 = (SPI1_BASE + 0x0C) as *mut u32;  // Data Register

pub struct CortexM3;

impl SPI for CortexM3 {

    // Initializes SPI1 in master mode with a clock prescaler of fPCLK/8
    fn spi_init_master() {
        const MASTER_MODE: u32 = 1 << 2;      //Sets SPI to Master mode
        const SPI_ENABLE: u32 = 1 << 6;       //Enables the SPI
        const CLOCK_DIV8: u32 = 0b011 << 3;  //Sets baudrate to clockfrequency/8

        unsafe {
            *SPI1_CR1 = MASTER_MODE | CLOCK_DIV8; // Configures SPI1
            *SPI1_CR1 |= SPI_ENABLE;              // Enables SPI1
        }
    }

    // Initializes SPI1 in slave mode
    fn spi_init_slave() {
        const SLAVE_MODE_MASK: u32 = !(1 << 2); // Clears MSTR bit (bit 2) to set slave mode
        const SPI_ENABLE: u32 = 1 << 6;        //Enables the SPI

        unsafe {
            *SPI1_CR1 &= SLAVE_MODE_MASK;       // Configures SPI1 as slave
            *SPI1_CR1 |= SPI_ENABLE;             // Enables SPI1
        }
    }

    fn spi_write(data: u8) {
        unsafe {
            while *SPI1_SR & (1 << 1) == 0 {}   // Waits until the transmit buffer is empty (until TXE flag is set)
            *SPI1_DR = data as u32;             // Writes data to the Data Register to start transmission
        }
    }

    fn spi_read() -> u8 {
        unsafe {
            while *SPI1_SR & (1 << 0) == 0 {}   //Waits until there is data in the receive buffer (until RXNE flag is set)
            *SPI1_DR as u8  //Reads and returns received data from the Data Register
        }
    }

    // Simultaneously writes and reads data in slave mode
    fn spi_transfer(data: u8) -> u8 {
        unsafe {
            while *SPI1_SR & (1 << 1) == 0 {}  // Wait until TXE flag is set
            *SPI1_DR = data as u32;            // Write data to be sent
            while *SPI1_SR & (1 << 0) == 0 {}  // Wait until RXNE flag is set
            *SPI1_DR as u8                     // Read and return received data
        }
    }
    
}
