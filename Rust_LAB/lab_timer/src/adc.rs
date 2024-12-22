#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)] // Activate experimental features

use panic_halt as _;
use avr_device::interrupt;

mod atmega_328p_ports;
use crate::atmega_328p_ports::*;

const PB7: u8 = 7;
const PB6: u8 = 6;
const ADSC: u8 = 6; // ADC Start Conversion bit

#[avr_device::entry]
fn main() -> ! {
    config_uart0();
    config_adc();
    loop {

    // Start a new conversion by setting the ADSC bit in ADCSRA
    unsafe {
        ADCSRA.write(ADCSRA.read() | (1 << ADSC)); // Start ADC conversion
    }

    // Read the result
    let mut adc7: u16 = ((ADCH.read() as u16) << 8) | (ADCL.read() as u16);
    adc7 = (adc7 * 5000) >> 10; // Convert to voltage (0 to 5V)
    
    // Handle the ADC result (for example, store it, print it, etc.)
		}

    }
}


fn config_uart0() {
    unsafe {
        UBRR0.write(51); // Baud rate 9600
        UCSR0C.write(6); // 8 data bits, 1 stop bit
        UCSR0B.write(0x18); // Enable TX and RX
    }
}

fn config_adc() {
	
	unsafe {
		ADMUX.write(0x47) ;
		ADCSRA.write(0x88);
		
		}	
}

// Interrupt handler
#[interrupt(atmega328p)]
fn ADC() {
    let adc_value = adc7; // The ADC value you obtained from the conversion

    // Convert ADC value to string for USART transmission
    let message = format!("ADC Value: {}\r\n", adc_value); // Include a newline

    unsafe {
        // Toggle PB6 (optional)
        PORTB.write(PORTB.read() ^ (1 << PB6));

        // Send each byte of the message over USART
        for &byte in message.as_bytes() {
            UDR0.write(byte); // Write each byte to the USART data register
            // Add a small delay to allow the USART to transmit each byte
            for _k in 0..1000 {
                avr_device::asm::nop();
            }
        }
    }
}

