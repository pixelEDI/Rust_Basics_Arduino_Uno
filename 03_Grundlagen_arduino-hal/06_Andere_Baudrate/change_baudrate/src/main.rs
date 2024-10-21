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

use arduino_hal::prelude::*;
use arduino_hal::Peripherals;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
   
    let mut digital1 = pins.d2.into_floating_input();

    loop {
       
        ufmt::uwriteln!(&mut serial, "pin is high").unwrap_infallible();
        arduino_hal::delay_ms(500);
        ufmt::uwriteln!(&mut serial, "pin is low").unwrap_infallible();
        arduino_hal::delay_ms(500);
    }
}
