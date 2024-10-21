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
// struct Point {
//     int x;
//     int y;
// };

// void setup() {
//     Serial.begin(9600);
    
//     Point p = {10, 20};
//     Serial.print("X: ");
//     Serial.println(p.x);
//     Serial.print("Y: ");
//     Serial.println(p.y);

//     Point p2 = {100, 200};
//     Serial.print("X: ");
//     Serial.println(p2.x);
//     Serial.print("Y: ");
//     Serial.println(p2.y);

// }

// void loop() {}
// ----------------------- ARDUINO CODE ----------------------- //


fn main() {
    // Eigenschaften von `struct`:
    // *Benannte Felder**: Jedes Feld hat einen Typ und Namen.
    //  **Typensicherheit**: Rust überprüft die Typen aller Felder zur Compile-Zeit.

    struct Point{
        x: i32,
        y: f32,
        my_string: String,
    }

    let p = Point {x:10, y:20.3, my_string: String::from("Achse 1")};
    println!("X {}, y {} und my_string {}",p.x, p.y, p.my_string);

    let p2 = Point {x:100, y:200.3, my_string: String::from("Achse 2")};
    println!("X {}, y {} und my_string {}",p2.x, p2.y, p2.my_string);

}
