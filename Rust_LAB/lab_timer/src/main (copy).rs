#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use avr_device::interrupt;
use core::sync::atomic::{AtomicBool, Ordering};

mod atmega_328p_ports;
use crate::atmega_328p_ports::*;

const PB7: u8 = 7;
const PB6: u8 = 6;
const ADSC: u8 = 6;

static SEM: AtomicBool = AtomicBool::new(false); // Semaphore to manage data transmission
static mut ADC_RESULT: u16 = 0; // ADC result storage

// Main entry point
#[avr_device::entry]
fn main() -> ! {
    config_timer();
    config_uart0();
    config_adc();

    loop {
        if SEM.load(Ordering::SeqCst) {
            SEM.store(false, Ordering::SeqCst); // Clear the semaphore
            
            unsafe {
                // ADC data is available, process it (e.g., send it via UART)
                // Send the ADC result via UART
                send_uart(ADC_RESULT);
            }
        }
    }
}

// Timer configuration (TIMER1)
fn config_timer() {
    unsafe {
        DDRB.write(0xFF); // PortB as OUTPUT (for toggling PB7)
        TCCR1A.write(0);  // Timer mode (Normal mode)
        TCCR1B.write(2);  // Prescaler: divide by 8
        TIMSK1.write(1);  // Enable overflow interrupt (OVF)
        TCNT1.write(55535); // Timer initial value (for overflow interval)
        interrupt::enable(); // Enable global interrupts
    }
}

// UART0 configuration
fn config_uart0() {
    unsafe {
        UBRR0.write(51);   // Baud rate 9600 (for 16 MHz clock)
        UCSR0C.write(6);    // 8 data bits, 1 stop bit
        UCSR0B.write(0x18); // Enable TX and RX
    }
}

// ADC configuration
fn config_adc() {
    unsafe {
        // Set ADMUX: Select channel 0 (ADC0) and Vref = 5V
        ADMUX.write(0b01000000); // Channel 0 and Vref = 5V
        // Set ADCSRA: Enable ADC, enable interrupt, start conversion
        ADCSRA.write(0b10001011); // ADC Enable, ADC Start Conversion, ADC Interrupt Enable
    }
}

// Timer1 overflow interrupt handler
#[interrupt(atmega328p)]
fn TIMER1_OVF() {
    unsafe {
        // Toggle PB7 (LED)
        PORTB.write(PORTB.read() ^ (1 << PB7));
        
        // Reload the timer with the initial value
        TCNT1.write(3035); // Adjust the reload value as needed for timing
        
        // Start the ADC conversion by setting the ADSC bit
        ADCSRA.write(ADCSRA.read() | (1 << ADSC)); // Start conversion
    }
}

// ADC conversion complete interrupt handler
#[interrupt(atmega328p)]
fn ADC() {
    // Set the semaphore to allow data transmission (ADC is done)
    SEM.store(true, Ordering::SeqCst);
    
    unsafe {
        // Read the ADC result from ADCL and ADCH registers
        let low_byte = ADCL.read();
        let high_byte = ADCH.read();
        ADC_RESULT = ((high_byte as u16) << 8) | low_byte as u16;
    }
}

// Assuming UDRE0 is a constant defined somewhere, like:
const UDRE0: u8 = 5;  // Example bit position, adjust this as necessary

fn send_uart(value: u16) {
    unsafe {
        // Wait for the transmit buffer to be empty
        while (UCSR0A.read() & (1 << UDRE0)) == 0 {}

        // Send the lower byte of the ADC result
        UDR0.write((value & 0xFF) as u8);

        // Wait for the transmit buffer to be empty again
        while (UCSR0A.read() & (1 << UDRE0)) == 0 {}

        // Send the higher byte of the ADC result
        UDR0.write(((value >> 8) & 0xFF) as u8);
    }
}


