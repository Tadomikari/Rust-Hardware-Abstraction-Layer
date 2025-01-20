#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod gpio;
pub mod usart;
pub mod spi;
pub mod i2c;

