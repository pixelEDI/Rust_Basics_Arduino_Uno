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

use arduino_hal::simple_pwm::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    // let mut pwm_led = pins.d5.into_output().into_pwm(&timer0);
    
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let mut pwm_led = pins.d11.into_output().into_pwm(&timer2);
    
    // Der Prescaler im Kontext von PWM (Pulsweitenmodulation) und Timern gibt an, 
    // um wie viel der Timer-Clock-Zyklus geteilt wird, bevor die Zählung beginnt.
    // Dies ist wichtig, um die Frequenz der PWM-Signale zu steuern.
    //
    // Im folgenden Beispiel wird der Timer0 mit einem Prescaler von 64 initialisiert:
    //
    // let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    //
    // Das bedeutet, dass der Timer nur jeden 64. Taktzyklus des Haupttakts (Timer-Clock) zählt. 
    // Dadurch wird die Frequenz des PWM-Signals reduziert, was nützlich ist, wenn 
    // eine niedrigere PWM-Frequenz benötigt wird oder die Auflösung bei einer gegebenen Frequenz 
    // erhöht werden soll.
    //
    // Zusammengefasst:
    // - Prescaler: Teilt den Haupttakt des Timers.
    // - Prescale64: Der Timer zählt nur jeden 64. Takt, was die Frequenz der PWM-Signale senkt.

    
    pwm_led.enable();
     loop {

        for x in (0..=255).chain((0..=254).rev()){
            pwm_led.set_duty(x);
            arduino_hal::delay_ms(5);
        }

     }
}
