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
use arduino_hal::prelude::*;
use embedded_hal::digital::{self, InputPin, OutputPin};
use embedded_hal_v0::serial::Read;

mod millis;
use millis::{millis_init, millis};

pub struct Led<PIN> {
    pin: PIN,
    interval: u32,
    last_toggle: u32,
    state: bool,
}

impl<PIN> Led<PIN>
where 
    PIN: OutputPin,
    {
        fn new(pin: PIN, interval: u32) -> Self{
            Self {
                pin,
                interval,
                last_toggle: 0,
                state: false,
            }
        }

        pub fn on(&mut self){
            self.pin.set_high();
        }

        pub fn off(&mut self){
            self.pin.set_low();
        }

        // pub fn blink(&mut self){
        //     self.on();
        //     arduino_hal::delay_ms(self.interval);
        //     self.off();
        //     arduino_hal::delay_ms(self.interval);
        // }

        pub fn blink_nonblocking(&mut self, current_millis: u32){
            if current_millis - self.last_toggle >= self.interval {
                self.state = !self.state;

                if self.state {
                    self.off();
                } else {
                    self.on();
                }

                self.last_toggle = current_millis;
            }
        }
    }

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    millis_init(dp.TC0);
    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    let ledpin1 = pins.d2.into_output();
    let ledpin2 = pins.d3.into_output();
    let ledpin3 = pins.d4.into_output();

    let mut led1 = Led::new(ledpin1, 100);
    let mut led2 = Led::new(ledpin2, 50);
    let mut led3 = Led::new(ledpin3, 2000);

    loop {

    let current_millis = millis();   
    led1.blink_nonblocking(current_millis);
    led2.blink_nonblocking(current_millis);
    led3.blink_nonblocking(current_millis);


    // Szenario 2
    // led1.blink();

    // Szenario 1
    //    led1.on();
    //    arduino_hal::delay_ms(500);
    //    led1.off();
    //    arduino_hal::delay_ms(500);
    }
}
