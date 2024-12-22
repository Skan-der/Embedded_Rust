#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use avr_device::interrupt;
use core::sync::atomic::{AtomicBool, Ordering};

// External module
mod atmega_328p_ports;
use crate::atmega_328p_ports::*;

// Pin definitions
const PB7: u8 = 7;
const ADSC: u8 = 6;

static SEM: AtomicBool = AtomicBool::new(false); // Semaphore for ADC data ready
static mut ADC_RESULT: u16 = 0; // Global variable for ADC result

#[avr_device::entry]
fn main() -> ! {
    config_timer();
    config_uart0();
    config_adc();

    unsafe {
        // Configure PORTB as output
        DDRB.write(0xFF);
        // Timer in Normal Mode
        TCCR1A.write(0);
        TCCR1B.write(1); // Prescaler
        TIMSK1.write(1); // Enable Timer Overflow Interrupt
        PORTD.write(4); // Enable PD2 (INT0)
    }

    loop {
        unsafe {
            if SEM.load(Ordering::SeqCst) {
                SEM.store(false, Ordering::SeqCst); // Clear semaphore

                // Read ADC value
                let adc_value: u16 = ((ADCH.read() as u16) << 8) | (ADCL.read() as u16);

                // Convert ADC value to string and send over UART
                let mut buffer = [0u8; 6];
                let value_str = int_to_str(adc_value, &mut buffer);
                send_uart("The value is ");
                send_uart(value_str);
                send_uart("\r\n");

                SEM.store(true, Ordering::SeqCst); // Set semaphore to indicate data is ready
            }
        }
    }
}

// Timer configuration
fn config_timer() {
    unsafe {
        DDRB.write(0xFF); // Set PORTB as output
        TCCR1A.write(0);  // Timer Normal Mode
        TCCR1B.write(2);  // Prescaler to 8
        TIMSK1.write(1);  // Enable Timer Overflow Interrupt
        TCNT1.write(55535); // Initialize counter
        PORTD.write(4);   // Enable PD2 (INT0)
        interrupt::enable(); // Enable global interrupts
    }
}

// UART configuration
fn config_uart0() {
    unsafe {
        UCSR0C.write(0b00000110); // Frame format: 8 data bits, 1 stop bit
        UBRR0.write(51); // Baud rate: 9600 bps
        UCSR0B.write(0x18); // Enable TX and RX
    }
}

// ADC configuration
fn config_adc() {
    unsafe {
        ADMUX.write(0x47);  // ADC configuration
        ADCSRA.write(0x88); // Enable ADC
    }
}

// Send a string over UART
fn send_uart(message: &str) {
    for &byte in message.as_bytes() {
        unsafe {
            UDR0.write(byte); // Send byte
        }
        for _ in 0..1000 {
            avr_device::asm::nop(); // Small delay
        }
    }
}

// Convert an integer to a string
fn int_to_str(mut value: u16, buffer: &mut [u8]) -> &str {
    let mut index = buffer.len() - 1;

    buffer[index] = 0; // Null-terminate
    index -= 1;

    if value == 0 {
        buffer[index] = b'0';
        return core::str::from_utf8(&buffer[index..]).unwrap();
    }

    while value > 0 {
        buffer[index] = b'0' + (value % 10) as u8;
        value /= 10;
        index -= 1;
    }

    core::str::from_utf8(&buffer[index + 1..]).unwrap()
}

// Timer Overflow Interrupt Handler
#[interrupt(atmega328p)]
fn TIMER1_OVF() {
    unsafe {
        PORTB.write(PORTB.read() ^ (1 << PB7)); // Toggle PB7
        TCNT1.write(55535); // Reload counter value
        ADCSRA.write(ADCSRA.read() | (1 << ADSC)); // Start ADC conversion
    }
}

// ADC Interrupt Handler
#[interrupt(atmega328p)]
fn ADC() {
    SEM.store(true, Ordering::SeqCst); // Indicate data is ready
}
