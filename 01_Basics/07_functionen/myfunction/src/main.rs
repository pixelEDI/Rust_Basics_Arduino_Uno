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
    
//     int a = 5;
//     int b = 10;
//     int result = add(a, b);
//     Serial.println(result);

//     String message = "Hello, Arduino!";
//     printMessage(message);
// }

// void loop() {}

// // Funktion zur Addition von zwei Ganzzahlen
// int add(int x, int y) {
//     return x + y;
// }

// // Funktion zur Ausgabe einer Nachricht
// void printMessage(String msg) {
//     Serial.println(msg);
// }
// ----------------------- ARDUINO CODE ----------------------- //

// fn add(x: i32, zweitevariable:i32) -> i32 {
//     let result: i32 = x + zweitevariable;
//     return result;
// }

fn add(x: i32, zweitevariable:i32) -> i32 {
    x + zweitevariable
}

fn print_message(msg: &String){
    println!("mein String wird in der FN ausgegeben: {}", msg);

}

fn main() {

    let a: i32 = 5;
    let b: i32 = 10;
    let result: i32 = add(a, b);
    println!("{}", result);

    let mymessage = String::from("Hello, Rust!");
    print_message(&mymessage);
}
