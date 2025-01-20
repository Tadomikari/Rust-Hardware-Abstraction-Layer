#![no_std]
#![no_main]

#[cfg(feature = "cortex_m3")]
use cortex_m_rt::entry;
#[cfg(feature = "cortex_m3")]
use cortex_m::asm;

#[cfg(feature = "atmega328p")]
use avr_device::asm::nop;

use hal_project::gpio::{configure_pin, read_pin, write_pin, PinMode, PinValue};
use hal_project::usart::{usart_init, usart_write, usart_read};
use hal_project::spi::{spi_init_master, spi_init_slave, spi_write, spi_read, spi_transfer};
use hal_project::i2c::{i2c_init, i2c_write, i2c_read};

// Entry point is conditional
#[cfg(feature = "cortex_m3")]
#[entry]
fn main() -> ! {
    unified_main()
}

#[cfg(feature = "atmega328p")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    unified_main()
}

// Shared main logic
fn unified_main() -> ! {

     // GPIO Example
     if let Ok(pin) = GpioPin::new(2) {
        configure_pin(pin.number(), PinMode::Output); // Configure pin 2 as output
        write_pin(pin.number(), PinValue::High);      // Set pin 2 to HIGH
        let gpio_state = read_pin(pin.number());      // Read the state of pin 2
        if let PinValue::High = gpio_state {
            write_pin(pin.number(), PinValue::Low);   // Turns pin 2 state to Low if it is High
        }
    }

    // USART Example
    usart_init(9600); // Initialize USART with 9600 baud
    usart_write(0x31); // Write '1' (ASCII 0x31)
    let received = usart_read(); // Read received data
    usart_write(received); // Echo back received data

    // SPI Example (Master Mode)
    spi_init_master(); // Initialize SPI in master mode
    spi_write(0x55);   // Send data
    let spi_data = spi_read(); // Read a byte
    let spi_response = spi_transfer(0x42); // Simultaneously write and read
    if spi_response != 0x00 {
        let _ = spi_response; // Could be replaced with logic to add consequences to the response
    }

    // SPI Example (Slave Mode)
    spi_init_slave(); // Initialize SPI in slave mode
    let slave_response = spi_transfer(0x00); // Send and receive data
    if slave_response != 0x00 {
        let _ = slave_response; // Could be replaced with logic to add consequences to the response
    }

    // I2C Example
    i2c_init(100_000); // Initialize I2C at 100 kHz
    i2c_write(0x42, &[0x01, 0x02, 0x03]); // Write data to slave
    let mut i2c_data = [0u8; 3];
    let i2c_result = i2c_read(0x42, &mut i2c_data); // Read data from slave
    if i2c_result == 0 {
        let _ = i2c_data; // Could be replaced with logic to add consequences to twhat was read
    }

    // Infinite loop to keep the program active
    loop {
        #[cfg(feature = "cortex_m3")]
        asm::nop();

        #[cfg(feature = "atmega328p")]
        nop();
    }
}

// Safe wrapper for GPIO pin numbers
struct GpioPin(u8);

impl GpioPin {
    fn new(pin: u8) -> Result<Self, &'static str> {
        if pin < 32 {
            Ok(Self(pin))
        } else {
            Err("Invalid GPIO pin: must be between 0 and 31")
        }
    }

    fn number(&self) -> u8 {
        self.0
    }
}
