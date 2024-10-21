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

// ----------------------- ARDUINO CODE ----------------------- //
// enum State {
//     ON,
//     OFF,
//     ERROR
// };

// void setup() {
//     Serial.begin(9600);

//     State currentState = ON;

//     if (currentState == ON) {
//         Serial.println("State is ON");
//     }
// }

// void loop() {}
// ----------------------- ARDUINO CODE ----------------------- //

fn main() {

// `enum` in Rust erlaubt es, mehrere Varianten eines Typs zu definieren.
// Beispiel: 
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// Eigenschaften von `enum`:
// 1. **Mehrere Möglichkeiten**: Eine Variable kann eine von mehreren festgelegten Varianten annehmen.
// 2. **Sicher**: Rust sorgt dafür, dass du nur gültige Varianten verwendest.
// 3. **Zusätzliche Daten**: Varianten können auch Daten enthalten, z.B. `enum Message { Text(String), Number(i32) }`.

    enum mystate {
        On,
        Off,
        Error,
    }

    let current_state = mystate::On;

    match current_state{
        mystate::Off => println!("State is off"),
        _ => {}
    }

}
