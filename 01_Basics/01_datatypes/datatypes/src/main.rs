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

// copy code into https://wokwi.com/projects/new/arduino-uno

// void setup() {
//     Serial.begin(9600);
    
//     int myInt = 42;
//     float myFloat = 3.14;
//     char myChar = 'A';
//     byte myByte = 255;               // Byte-Datentyp
//     bool myBool = true;             // Boolescher Datentyp
//     String myString = "Hallo, Arduino!"; // String-Datentyp

//     Serial.println(myInt);
//     Serial.println(myFloat);
//     Serial.println(myChar);
//     Serial.println(myByte);
//     Serial.println(myBool);
//     Serial.println(myString);
// }

// void loop() {
//     // Nichts zu tun hier
// }
// ----------------------- ARDUINO CODE ----------------------- //


fn main() {

    // - Deklarieren: Den Namen und Typ einer Variable angeben, ohne ihr einen Wert zuzuweisen.
    // - Initialisieren: Einer Variable einen Wert zuweisen, wodurch sie bereit zur Verwendung wird.


    // In Rust müssen Variablen initialisiert werden, weil:
    // - Sicherheitsgarantie: Verhindert undefiniertes Verhalten.
    // - Kompilierungszeitfehler: Erkennen von Bugs vor der Laufzeit.
    // - Unveränderlichkeit: Standardmäßig sind Variablen unveränderlich.
    // - Ownership und Borrowing: Klare Kontrolle über Werte und Besitz.


    let my_int: i16 = 42;
    // let my_int: u8=300;
    let my_float: f32 = 3.14;
    let my_char: char = 'A';
    let my_byte: u8 = 255;
    let my_bool = true;
    let my_string = String::from("Hallo Rust");
    let my_tupples: (i32, f32, char) = (42, 3.14, 'B');

    println!("my int: {}", my_int);
    println!("{}", my_float);
    println!("{}", my_char);
    println!("{}", my_byte);
    println!("{}", my_bool);
    println!("{}", my_string);

    println!("{:?}", my_tupples);
    let (my_int, my_float, my_char) = my_tupples;

    println!("Integer: {}", my_int);
    println!("Float: {}", my_float);
    println!("Char: {}", my_char);

    // In Rust sind Tuples, Structs und Enums verschiedene Datentypen.
    //
    // Tuples sind geordnete Sammlungen von Werten unterschiedlicher Typen und ermöglichen 
    // den Zugriff über Indizes, z.B. (1, "Text"). Sie sind nützlich für kleine, heterogene 
    // Datensammlungen Bsp. Kooridnaten mit lat long
    //
    // Structs hingegen sind benannte Datenstrukturen, die Felder mit Typen enthalten, und 
    // ermöglichen den Zugriff über die Feldnamen, wie `person.name`. Sie bieten mehr 
    // Klarheit und sind ideal für komplexere Datenmodelle.
    //
    // Enums erlauben es, verschiedene Variationen eines Datentyps zu definieren, wobei 
    // jede Variante unterschiedliche Daten haben kann. Zum Beispiel kann ein Enum wie 
    // `enum Status { Active, Inactive(u32) }` definiert werden, um unterschiedliche 
    // Statusarten zu repräsentieren.

}
