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
use panic_halt as _;
use arduino_hal::adc;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let a2 = pins.a2.into_analog_input(&mut adc);

    loop {
        let a2_value = a2.analog_read(&mut adc);

        ufmt::uwriteln!(&mut serial, "Wert von a2: {}",a2_value).unwrap_infallible();
        arduino_hal::delay_ms(20);
    }
}