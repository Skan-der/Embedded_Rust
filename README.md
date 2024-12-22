# AVR UART and ADC Communication Project

This project demonstrates UART communication and ADC integration using an AVR microcontroller. The project is developed in Rust and tested through simulation on Proteus.

## Features

- UART configuration and communication.
- ADC data acquisition and conversion.
- String manipulation and data transmission over UART.
- Integration of hardware interrupts for efficient processing.
- Simulation setup using Proteus.

## Tools Used

- **Programming Language**: Rust
- **Microcontroller**: AVR (ATmega328P)
- **Simulation Software**: Proteus
- **Code Editor**: Geany

## Circuit Configuration

The circuit design adheres to the guidelines provided in the ATmega328P datasheet for proper ADC functionality. Refer to the datasheet for detailed schematics and pin configurations.

## How It Works

1. **UART Communication**: Initializes UART with a baud rate of 9600 and sends formatted data to a serial terminal.
2. **ADC Integration**: Reads analog signals from ADC, converts them into digital values, and transmits the results via UART.
3. **Interrupt Handling**: Utilizes timer and ADC interrupts to manage data flow efficiently.


