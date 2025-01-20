pub mod atmega328p;
pub mod cortex_m3;

pub trait SPI {
    fn spi_init_master();
    fn spi_init_slave();
    fn spi_write(data: u8);
    fn spi_read() -> u8;
    fn spi_transfer(data: u8) -> u8 {
        Self::spi_write(data);
        Self::spi_read()
    }
}

#[cfg(feature = "atmega328p")]
pub type ActiveSPI = atmega328p::Atmega328p;

#[cfg(feature = "cortex_m3")]
pub type ActiveSPI = cortex_m3::CortexM3;

pub fn spi_init_master() {
    ActiveSPI::spi_init_master();
}

pub fn spi_init_slave() {
    ActiveSPI::spi_init_slave();
}

pub fn spi_write(data: u8) {
    ActiveSPI::spi_write(data);
}

pub fn spi_read() -> u8 {
    ActiveSPI::spi_read()
}

pub fn spi_transfer(data: u8) -> u8 {
    ActiveSPI::spi_transfer(data)
}