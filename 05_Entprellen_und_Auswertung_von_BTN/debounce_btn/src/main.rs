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

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    millis_init(dp.TC0);
    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    let digital1 = pins.d10.into_floating_input();
    let digital2 = pins.d11.into_floating_input();

    let mut btn1 = Button::new(digital1, ButtonState::Idle);
    let mut btn2 = Button::new(digital2, ButtonState::Idle);
    
    let mut led1 = pins.d2.into_output();
    let mut led2 = pins.d3.into_output();

    loop {

        let current_millis = millis();


        match btn1.update(current_millis) {
            ButtonState::Short_press => {
                ufmt::uwriteln!(&mut serial, "btn1 short: {}", btn1.counter).unwrap();
                led1.toggle();
            }
            ButtonState::Long_press => {
                ufmt::uwriteln!(&mut serial, "btn1 looong: {}", btn1.counter).unwrap();
                led1.set_low();
                
            }
            ButtonState::Reset => {
                // ufmt::uwriteln!(&mut serial, "Reset").unwrap();
            }
            _ => { } // errorhandling

            // 1 => ufmt::uwriteln!(&mut serial, "btn1 1: {}", btn1.counter).unwrap(),
            // 2 => ufmt::uwriteln!(&mut serial, "btn1 2 loong: {}", btn1.counter).unwrap(),
            // _ => { } // errorhandling
        }

        match btn2.update(current_millis) {
            ButtonState::Short_press => {
                ufmt::uwriteln!(&mut serial, "btn2 short: {}", btn2.counter).unwrap();
                led2.toggle();
            }
            ButtonState::Long_press => {
                ufmt::uwriteln!(&mut serial, "btn2 looong: {}", btn2.counter).unwrap();
                led2.set_low();
            }
            ButtonState::Reset => {
                // ufmt::uwriteln!(&mut serial, "Reset").unwrap();
            }
            _ => { } // errorhandling
        }

      



        // btn1.update(current_millis);
        // btn2.update(current_millis);

        // Szenario 2
        // if btn1.is_pressing() {
        //     let cnt1 = btn1.counter;
        //     ufmt::uwriteln!(&mut serial, "//cnt1// {}", cnt1).unwrap_infallible();
        // }
        // else if btn2.is_pressing(){
        //     let cnt2 = btn2.counter;
        //     ufmt::uwriteln!(&mut serial, "--cnt2-- {}", cnt2).unwrap_infallible();
        // }


        // Szenario 1
        // let state_string = btn1.state.which_state();
        // let cnt1 = btn1.counter;

        // ufmt::uwriteln!(&mut serial, "state btn {}", state_string).unwrap_infallible();
        // ufmt::uwriteln!(&mut serial, "cnt1 {}", cnt1).unwrap_infallible();

        arduino_hal::delay_ms(100);
    }
}
