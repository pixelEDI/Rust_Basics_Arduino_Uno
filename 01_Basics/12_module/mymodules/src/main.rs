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

use mymodules::add;
use mymodules::my_functions::{blink_led, public_function, streng_geheim};

fn main() {
    // let result = add(10,5,5);
    // println!("Das Ergebnis ist: {}",result);

    // my_functions::blink_led(12);
    blink_led(5);
    public_function();

}
