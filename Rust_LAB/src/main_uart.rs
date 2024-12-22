#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)] // Ajoute cette ligne pour activer les fonctionnalités expérimentales

use panic_halt as _;
use avr_device::interrupt;

//const PORTB:*mut u8=0x25 as *mut _ ;
//const PORTD:*mut u8=0x2B as *mut _ ;
//const DDRB:*mut u8=0x24 as *mut _ ;
//const DDRD:*mut u8=0x2A as *mut _ ;
//const EICRA:*mut u8=0x69 as *mut _ ;
//const EIMSK:*mut u8=0x3D as *mut _ ;
mod atmega_328p_ports ;
use crate::atmega_328p_ports::* ;

//const PD2:u8= 2;
const PB7:u8= 7;
const PB6:u8= 6;

#[avr_device::entry]
fn main() -> ! {
		config_timer() ;
     loop {
        unsafe {PORTB.write(PORTB.read()|(1<<PB7));}
        for _k in 0..10_000 {avr_device::asm::nop();};
        unsafe {PORTB.write(PORTB.read()& !(1<<PB7));}
        for _k in 0..10_000 {avr_device::asm::nop();}
            
        }
}

fn config_timer()
{
	unsafe {
		DDRB.write(0xFF) ;// Portb OUTPUT
		TCCR1A.write(0);//mode de timer
	    TCCR1B.write(10);//Prescaler
		TIMSK1.write(1);//OVF_Interruption
		OCR1A.write(10000) ;
		interrupt::enable();// SREG |= (1 << 7) en C.
		}
	
	}




// Gestionnaire d'interruption 
#[interrupt(atmega328p)]
fn TIMER1_OVF() 
{
 unsafe{let mut  pb:u8 = PORTB.read() ;
 PORTB.write(pb^(1<<PB6));}
   
}
