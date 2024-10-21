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
use arduino_hal::{pac::fuse::low::CKOUT_R, prelude::*};
use embedded_hal::digital::{self, InputPin, OutputPin};
use embedded_hal_v0::serial::Read;

mod millis;
use millis::{millis_init, millis};

mod btn;
use btn::{ButtonState, Button};

const BH1750_ADDRESS: u8 = 0x23; // BH1750 I2C Adresse
const BH1750_COMMAND: u8 = 0x10;  // Start measurement in high-resolution mode

fn start_measurment(i2c: &mut arduino_hal::I2c) -> bool {
    if let Err(err) = i2c.write(BH1750_ADDRESS, &[BH1750_COMMAND]){
        return false;
    }
    return true;
}

fn read_lux(i2c: &mut arduino_hal::I2c) -> Option<u16>{
    let mut buff = [0u8;2];

    if i2c.read(BH1750_ADDRESS, &mut buff).is_err() {
        return None;
    }

    let lux_raw = ((buff[0] as u16) << 8) | (buff[1] as u16);
    let lux = lux_raw / 2;

    Some(lux)

}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d2.into_output();

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(), // sda
        pins.a5.into_pull_up_input(), // scl
        50000,
    );

    let address: u8 = 0b0100010; // hex 0x23

    millis_init(dp.TC0);
    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };
    
    loop {

        if start_measurment(&mut i2c){

        // Delay for measurement time (max 180ms)
        arduino_hal::delay_ms(180);

        match read_lux(&mut i2c){
            Some(lux) => {
                ufmt::uwriteln!(&mut serial, "lux: {}", lux).unwrap_infallible();

                if lux > 200 {
                    led.set_low();
                } else {
                    led.set_high();
                }

            }
            None => {
                ufmt::uwriteln!(&mut serial, "Failed to read Lux value").unwrap_infallible();
            }
        }

        }

    // Szenario 1
    //    match i2c.ping_device(address, arduino_hal::i2c::Direction::Write) {
    //         Ok(true) => {
    //             led.set_high();
    //             ufmt::uwriteln!(&mut serial, "gefunden").unwrap_infallible();
    //         }
    //         Ok(false) => {
    //             led.set_low();
    //             ufmt::uwriteln!(&mut serial, "leider kein I2C Gerät gefunden").unwrap_infallible();
    //         }
    //         Err(err) => {
    //             ufmt::uwriteln!(&mut serial, "error {:?}!\r", err).unwrap_infallible();
    //         }
    //    }
    //    arduino_hal::delay_ms(1000);
    }

      
}
