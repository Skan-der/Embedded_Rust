#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)] // Pour activer les fonctionnalités expérimentales

use panic_halt as _;
use avr_device::interrupt;

// Importation du module externe atmega_328p_port
mod atmega_328p_ports;
use crate::atmega_328p_ports::*; 

// Constantes des broches
const PB7: u8 = 7;
const PB6: u8 = 6;
const ADSC:u8 = 6 ;
static mut SEM: bool = false;

#[avr_device::entry]
fn main() -> ! {
    config_timer();
    config_uart0();
    config_adc();
    
    unsafe {
        // Configurer PORTB comme sortie
        DDRB.write(0xFF);
        // Configurer le timer (Mode Normal)
        TCCR1A.write(0);
        TCCR1B.write(1); // Prescaler
        TIMSK1.write(1); // Activer l'interruption de débordement du timer
        PORTD.write(4);  // Activer PD2 (INT0)
    }
    
    loop {
        unsafe {
            if SEM {
				//afficher la val analogique 
				let mut adc7:u16 =  ((ADCH.read()as u16 ) << 8 ) | (ADCL.read()as u16 )   ;
				adc7 = (adc7 * 5000) >> 10;
				//affiche(adc7);//print & uart 
                // Fonction équivalente à printf
               /* UDR0.write(66); // Envoyer le caractère 'B'
                while (UCSR0A.read() & 0b01000000) == 0 {} // Attendre que le buffer soit vide
                UDR0.write(10); // Envoyer un retour à la ligne (LF)
                while (UCSR0A.read() & 0b01000000) == 0 {} // Attendre la transmission*/
                
                
                SEM = false;
            }
        }
    }
}

// Configuration du timer 500ms à faire 
fn config_timer() {
    unsafe {
        DDRB.write(0xFF); // Configurer PORTB en sortie
        TCCR1A.write(0);  // Mode du timer (Normal)
        TCCR1B.write(2);  // Prescaler à 8
        TIMSK1.write(1);  // Activer l'interruption de débordement du timer
        TCNT1.write(55535); // Initialiser le compteur
        PORTD.write(4);   // Activer PD2 (INT0)
        interrupt::enable(); // Activer les interruptions globales (SREG |= 0x80 en C)
    }
}

// Configuration de l'UART
fn config_uart0() {
    unsafe {
        UCSR0C.write(0b00000110); // Format de trame : 8 bits de données, 1 bit d'arrêt
        UBRR0.write(51); // Baudrate de 9600 bps
        UCSR0B.write(0x18); // Activer TX et RX
    }
}

fn config_adc() {
	
	unsafe {
		ADMUX.write(0x47) ;
		ADCSRA.write(0x88);
		
		}	
}



// Gestionnaire d'interruption pour le débordement du TIMER1
#[interrupt(atmega328p)]
fn TIMER1_OVF() {
    unsafe {
        // Lire la valeur actuelle de PORTB
        let pb: u8 = PORTB.read();
        // Inverser l'état de la LED sur PB6
        PORTB.write(pb ^ (1 << PB6));
        // Réinitialiser le compteur
        TCNT1.write(55535);
        // Déclencher l'indicateur SEM
       
        //Lancer une new conversion 
        ADCSRA.write(ADCSRA.read() | (1 << ADSC));
    }
}
#[interrupt(atmega328p)]
fn ADC() {
	  		
		unsafe { SEM = true };
	}
