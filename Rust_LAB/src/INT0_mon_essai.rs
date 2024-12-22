#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)] // Ajoute cette ligne pour activer les fonctionnalités expérimentales

use panic_halt as _;
use avr_device::interrupt;

static mut DP: Option<avr_device::atmega328p::Peripherals> = None; // Variable statique pour les périphériques

#[avr_device::entry]
fn main() -> ! {
    unsafe {
        DP = Some(avr_device::atmega328p::Peripherals::take().unwrap()); // Prendre les périphériques
    }

    // Configurer DDRB pour définir PB7 comme sortie
    unsafe {
        DP.as_ref().unwrap().PORTB.ddrb.write(|w| w.bits(0xF0)); // PB7 en sortie
        interrupt::enable(); // Activer les interruptions globales
        DP.as_ref().unwrap().EXINT.eimsk.write(|w| w.int0().set_bit()); // EIMSK : activer INT0
        DP.as_ref().unwrap().EXINT.eicra.write(|w| w.bits(0x02)); // Configurer INT0 pour le front descendant
    }

    loop {
        unsafe {
            DP.as_ref().unwrap().PORTB.portb.write(|w| w.pb7().set_bit()); // Mettre PB7 à 1
        }
        for _k in 0..10_000 {
            avr_device::asm::nop(); // Attendre
        }
    }
}

// Gestionnaire d'interruption pour INT0
#[interrupt(atmega328p)]
fn INT0() {
    unsafe {
        if let Some(dp) =&DP {
            dp.PORTB.portb.write(|w| w.pb7().clear_bit()); // Éteindre PB7 lors de l'interruption
        }
    }
}
