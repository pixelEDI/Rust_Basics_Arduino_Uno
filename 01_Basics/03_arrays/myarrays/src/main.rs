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
// void setup() {
//     Serial.begin(9600);
    
//     int numbers[] = {1, 2, 3, 4, 5};
//     int sum = 0;

//     for (int i = 0; i < 5; i++) {
//         sum += numbers[i];
//     }
//     Serial.println(sum);
// }

// void loop() {}

// ----------------------- ARDUINO CODE ----------------------- //


fn main() {

    // In Rust sind Arrays feste Datentypen mit einer festen Größe, die zur Compile-Zeit bekannt sein muss.
    // Arrays in Rust sind immer auf dem Stack gespeichert.

    // Unterschiede zu Arduino C++:
    // **Typensicherheit**: Rust verlangt, dass der Typ und die Größe des Arrays zur Compile-Zeit bekannt sind.
    // **Größe**: In C++ können Arrays dynamisch (z.B. mit `new`) erstellt werden, während Rust Arrays immer eine feste Größe haben müssen.
    
    let numbers = [1, 2, 3, 4, 5];
    let mut sum = 0;

    for &number in &numbers {
        sum += number;
    }

    println!("{}", sum);
    println!("{}", numbers[2]); 

    // nicht vorgesehen weil array fixe grösse hat
    // numbers.push(6);
    // println!("{:?}", numbers);
  
}
