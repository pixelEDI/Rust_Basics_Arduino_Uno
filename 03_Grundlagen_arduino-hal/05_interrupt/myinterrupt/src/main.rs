//          _          _ ______ _____ _____
//         (_)        | |  ____|  __ \_   _|
//    _ __  ___  _____| | |__  | |  | || |
//   | '_ \| \ \/ / _ \ |  __| | |  | || |
//   | |_) | |>  <  __/ | |____| |__| || |_
//   | .__/|_/_/\_\___|_|______|_____/_____|
//   | |
//   |_|

//  https://links.pixeledi.eu
//  Rust exercises | 11.2024



#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;

use core::sync::atomic::{AtomicBool, Ordering};
use arduino_hal::prelude::*;

static PIN_CHANGED: AtomicBool = AtomicBool::new(false);

// This function is called on change of pin 2
#[avr_device::interrupt(atmega328p)]
#[allow(non_snake_case)]
fn PCINT2() {
    PIN_CHANGED.store(true, Ordering::SeqCst);
}

fn rotate(flag: &AtomicBool) -> bool {
    avr_device::interrupt::free(|_cs| {
        if flag.load(Ordering::SeqCst) {
            flag.store(false, Ordering::SeqCst);
            true
        } else {
            false
        }
    })
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d11.into_output();

     // Enable the PCINT2 pin change interrupt
     dp.EXINT.pcicr.write(|w| unsafe { w.bits(0b100) });

     // Enable pin change interrupts on PCINT18 which is pin PD2 (= d2)
     dp.EXINT.pcmsk2.write(|w| w.bits(0b100));
 
     // From this point on an interrupt can happen
     unsafe { avr_device::interrupt::enable() };
    
//   unsafe ist nötig, weil du mit dem Befehl avr_device::interrupt::enable() 
//   direkt die Hardware steuerst, also die Interrupts aktivierst. Rust kann 
//   nicht von allein prüfen, ob das sicher ist, weil solche Operationen Risiken 
//   bergen (z.B. dass etwas Unerwartetes passiert, wenn ein Interrupt während eines 
//     kritischen Teils des Codes ausgelöst wird). Deshalb musst du Rust mit unsafe sagen: 
//     "Ich weiß, was ich tue, auch wenn das System es nicht komplett überprüfen kann."

    loop {
        if rotate(&PIN_CHANGED){
            led.set_high();
        } else {
            arduino_hal::delay_ms(1000);
            led.set_low();
        }
  
        
    }
}