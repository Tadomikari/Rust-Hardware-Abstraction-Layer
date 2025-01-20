pub mod atmega328p;
pub mod cortex_m3;

pub trait I2C {
    fn i2c_init(clock_speed: u32);
    fn i2c_write(address: u8, data: &[u8]);
    fn i2c_read(address: u8, buffer: &mut [u8]) -> u8;
}

#[cfg(feature = "atmega328p")]
pub type ActiveSPI = atmega328p::Atmega328p;

#[cfg(feature = "cortex_m3")]
pub type ActiveSPI = cortex_m3::CortexM3;

pub fn i2c_init(clock_speed: u32) {
    ActiveSPI::i2c_init(clock_speed);
}

pub fn i2c_write(address: u8, data: &[u8]) {
    ActiveSPI::i2c_write(address, data);
}

pub fn i2c_read(address: u8, buffer: &mut [u8]) -> u8 {
    ActiveSPI::i2c_read(address, buffer)
}


