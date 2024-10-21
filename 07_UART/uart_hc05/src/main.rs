//          _          _ ______ _____ _____
//         (_)        | |  ____|  __ \_   _|
//    _ __  ___  _____| | |__  | |  | || |
//   | '_ \| \ \/ / _ \ |  __| | |  | || |
//   | |_) | |>  <  __/ | |____| |__| || |_
//   | .__/|_/_/\_\___|_|______|_____/_____|
//   | |
//   |_|

//  https://links.pixeledi.eu
//  Rust exercises | 11.2024#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use arduino_hal::prelude::*;
use embedded_hal::digital::{self, InputPin, OutputPin};
use embedded_hal_v0::serial::Read;
use core::str;

mod millis;
use millis::{millis_init, millis};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let mut led = pins.d11.into_output();
    let mut buff: [u8; 32] = [0;32]; // 32 elemente mit dem Wert 0
    let mut index = 0;
    

    millis_init(dp.TC0);
    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    loop {

        let bvalue = nb::block!(serial.read()).unwrap_infallible();

        if bvalue != b'\r' && bvalue != b'\n' {
            if index < buff.len() {
                buff[index] = bvalue;
                index += 1;
            }
        } else if index > 0 {
            if let Ok(cmd) = str::from_utf8(&buff[..index]){
                ufmt::uwriteln!(&mut serial, "Received: {}\r", cmd).unwrap_infallible();

                if cmd == "on"{
                    led.set_high();
                } else if cmd == "off"{
                    led.set_low();
                }

            }

            buff = [0;32];
            index = 0;
            
        }

    }
}
