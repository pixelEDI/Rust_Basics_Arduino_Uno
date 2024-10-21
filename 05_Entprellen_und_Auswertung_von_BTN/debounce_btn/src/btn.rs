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

pub enum ButtonState {
    Short_press,
    Long_press,
    Idle,
    Reset,
}

impl ButtonState{
    pub fn which_state(&self) -> &'static str {
        match self {
            ButtonState::Short_press => "Short_press",
            ButtonState::Long_press => "Long_press",
            ButtonState::Idle => "Idle",
            ButtonState::Reset => "Reset",
        }
    }
}

pub struct Button<PIN>{
    state: ButtonState,
    pin: PIN,
    pub counter: u16,
    button_clicked_ms: u32,
    last_state: bool,
}

impl<PIN> Button<PIN>
where 
    PIN: InputPin,
    {
        pub fn new(pin: PIN, initial_state: ButtonState) -> Self {
            Self { 
                state: initial_state,
                pin,
                counter: 0,
                button_clicked_ms: 0,
                last_state: false,
             }
        }

        pub fn update(&mut self, current_time: u32) -> ButtonState {
            let prev_state = self.last_state;
            
            let state = match self.pin.is_high(){
                Ok(is_high) => is_high,
                Err(_) => false,
            };

            let mut result = ButtonState::Reset;

            //Debounce Logik
            if prev_state == false && state == true {
                //Button wurde gedrückt
                self.button_clicked_ms = current_time;
                result = ButtonState::Reset;
            } else if prev_state == true && state == false {
                let elapsed_time = current_time - self.button_clicked_ms;

                if elapsed_time < 50 {
                    //debounce - nix soll passieren
                    result = ButtonState::Reset;
                } else if elapsed_time >= 50 && elapsed_time < 1000 {
                    self.counter +=1;
                    result = ButtonState::Short_press;

                } else if elapsed_time >= 1000 {
                    self.counter -=1;
                    result = ButtonState::Long_press;

                }
            }

            self.last_state = state;
            result

        }

        // pub fn update(&mut self){
        //     match self.pin.is_high() {
        //         Ok(true) => {
        //             self.state = ButtonState::Pressing;
        //             self.counter += 1;
        //         }
        //         Ok(false) => {
        //             self.state = ButtonState::Idle;
        //         }
        //         Err(_) => {
        //             //error handling 
        //         }
        //     }
        // }

        // pub fn is_pressing(&self) -> bool {
        //     matches!(self.state, ButtonState::Pressing)
        // }
    }
