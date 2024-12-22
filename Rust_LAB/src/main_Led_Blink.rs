#![no_std]
#![no_main]

use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
     let dp = avr_device::atmega328p::Peripherals::take().unwrap();
     //DDRB=0x
     unsafe{ dp.PORTB.ddrb.write(|w| w.bits(0xF0));}
            
    loop {
        unsafe {dp.PORTB.portb.write(|w| w.pb7().set_bit());}
        for _k in 0..10_000 {avr_device::asm::nop();};
        unsafe {dp.PORTB.portb.write(|w| w.pb7().clear_bit());}
        for _k in 0..10_000 {avr_device::asm::nop();}
            
        }
}

