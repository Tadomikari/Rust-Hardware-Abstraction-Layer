# HAL_Project (embedded Rust)

Project of : Elsa Lhayani / Lucas Lombard / Henri Mao / Gabin Lefrançois / Benoit Hua

## **Overview**
This project implements a **Hardware Abstraction Layer (HAL)** in **Rust** for the **Atmega328p** and **Cortex-M3** microcontrollers.

Its goal is to provide an interface for controlling hardware peripherals regardless of the underlying microcontroller. This allows the users to use the **GPIO**, **USART**, **SPI** and **I²C** functionalities of both targets without knowing their technical specification and registers' specifications.

## **Features**
- **General-Purpose Input/Output (GPIO):**
  - Configure any digital pin as **input** or **output**.
  - **Read** and **Write** digital signals on all digital pins.
  - Safe pin management with runtime validation.
  - Example: Read the state of a led attached to a pin and turn it off (Low) if it is High.

- **Universal Synchronous/Asynchronous Receiver/Transmitter (USART):**
  - Initialize USART communication with a chosen baud rate.
  - **Send** and **receive** data over a serial interface.
  - Example: Communicate with another microcontroller to separate tasks.

- **Serial Peripheral Interface (SPI):**
  - Control mode operation with a chosen clock speed.
  - Master and Slave mode initialization.
  - Transfers data to and from SPI peripherals.
  - Example: Interact with an SPI sensor or memory module.
 
- **Inter Integrated Circuit (I²C):**
  - Initialize with configurable clock speeds.
  - Support for data write and read operations.
  - Example: Short distance communication between two controlers using only two wires.
 
## Supported Architectures
The HAL Project supports the following architectures:
- **ATmega328p**: Optimized for AVR-based microcontrollers.
- **Cortex-M3**: Designed for ARM-based microcontrollers.
Select the target architecture during compilation by enabling the respective feature.

## **Project Structure**
The project is modular and organized as follows:
```
hal_project/
├── Cargo.toml           # Rust project configuration
├── src/
│   ├── main.rs          # Main function with examples for each feature
│   ├── lib.rs           # Exports all modules
│   ├── gpio/            # GPIO module
│   │   ├── mod.rs       # Interface for GPIO
│   │   ├── atmega328p.rs # GPIO implementation for Atmega328p
│   │   └── cortex_m3.rs # GPIO implementation for Cortex-M3
│   ├── usart/           # USART module
│   │   ├── mod.rs       # Interface for USART
│   │   ├── atmega328p.rs # USART implementation for Atmega328p
│   │   └── cortex_m3.rs # USART implementation for Cortex-M3
│   ├── spi/             # SPI module
│   │   ├── mod.rs       # Interface for SPI
│   │   ├── atmega328p.rs # SPI implementation for Atmega328p
│   │   └── cortex_m3.rs # SPI implementation for Cortex-M3
├   ├──I2C/           # I2C module
│       ├── mod.rs       # Interface for I2C
│       ├── atmega328p.rs # I2C implementation for Atmega328p
│       └── cortex_m3.rs # I2C implementation for Cortex-M3

```

## Installation
### **1. Clone the repository:**
   ```bash
   git clone https://github.com/your-username/hal_project.git
   cd hal_project
   ```
### **2. Install dependencies:**
   - for **Atmega328p**:
     ```bash
     rustup target add avr-atmega328p
     sudo apt install avr-gcc avr-libc
     ```
   - for **Cortex M3**
     ```bash
     rustup target add thumbv7m-none-eabi
     ```
     
## **Usage**
### **1. Requirements**
- A Rust compiler (Install with `rustup`):  
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### **2. Build the Project**
To compile the project for a specific microcontroller, use Rust's feature flags:

- For **Atmega328p**:
  ```bash
  cargo build --release --target avr-specs/avr-atmega328p.json --features atmega328p
  ```

- For **Cortex-M3**:
  ```bash
  cargo build --release --target thumbv7m-none-eabi --features cortex_m3
  ```

### **4. Flash the Firmware**
- **For Atmega328p (Arduino Uno)**, use `avrdude` to flash the generated `.hex` file:
  ```bash
  avrdude -c arduino -p m328p -P /dev/ttyUSB0 -b 115200 -U flash:w:target.hex:i
  ```
  
- **For Cortex-M3** (e.g., STM32), use `openocd` to flash the firmware:
  ```bash
  openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg -c "program target.hex verify reset exit"
  ```

## **5. Verifying Functionality**
To verify that the project is working, you can:
1. **GPIO**: Connect a resistor and a LED to a GPIO pin (e.g., pin 2) configured as an output and check if it can be turned on and off.
2. **USART**: Use the `usart_write` function to send data and the `usart_read` function to receive it. Verify that the received data matches the sent data.
3. **SPI**: Use the `spi_transfer` function to send and receive data simultaneously. Verify that the received data matches the expected data based on the slave's behavior.
4. **I²C**: Connect an I2C slave device (e.g., a pressure sensor) to the microcontroller. Use the `i2c_write` function to send data to the slave and the `i2c_read` function to read data back.

### **6. Running Tests with `cargo test`**
You can run unit tests (in software) to check the logic behind the GPIO, USART, and SPI modules:
- **For Atmega328p**:
  ```bash
  cargo test --release --target avr-specs/avr-atmega328p.json --features atmega328p
  ```

- **For Cortex-M3**:
  ```bash
  cargo test --release --target thumbv7m-none-eabi --features cortex_m3
  ```

These tests will verify that the logic of configuring pins, reading and writing data, and sending/receiving messages is correct.

[CORRECTION GPIO] (Don't hesitate to remove this part)
Consider subdividing your project into separate modules. 
You can only use the I/O registers of port B here (and not the C port for example).
It would be nice to have something to prevent modifying the register in an incoherent way. For example, if I do ``` gpio.write_pin(60, PinValue::High);```, it won't bug during the compilation, but it may generate a problem on your hardware.
