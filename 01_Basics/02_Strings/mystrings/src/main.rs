
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


    // // Original String
    // String myString = "Hello, Rust!";

    // // Umkehren des Strings
    // String reversedString = "";
    // for (int i = myString.length() - 1; i >= 0; i--) {
    //     reversedString += myString[i];
    // }
    // Serial.println(reversedString);

    // // Entfernen des letzten Zeichens
    // myString.remove(myString.length() - 1);
    // Serial.println(myString);

    // // Ersetzen von "Hello" durch "Howdie"
    // String newString = myString;
    // newString.replace("Hello", "Howdie");
    // Serial.println(newString);

    // // Aufteilen des Strings
    // int index = myString.indexOf(", ");
    // String firstPart = myString.substring(0, index);
    // String secondPart = myString.substring(index + 2);
    // String parts[] = {firstPart, secondPart};
    // Serial.print("Parts: ");
    // Serial.print(parts[0]);
    // Serial.print(", ");
    // Serial.println(parts[1]);

    // // Länge des Strings
    // int length = myString.length();
    // Serial.print("Länge: ");
    // Serial.println(length);

    // // Umwandeln in Großbuchstaben
    // String upperString = myString;
    // upperString.toUpperCase();
    // Serial.println(upperString);

    // // Umwandeln in Kleinbuchstaben
    // String lowerString = myString;
    // lowerString.toLowerCase();
    // Serial.println(lowerString);

    // // Überprüfen, ob der String "Hello" enthält
    // bool containsHello = myString.indexOf("Hello") != -1;
    // Serial.println(containsHello ? "true" : "false");

    // // Trimmed String
    // String trimmedString = myString;
    // trimmedString.trim();
    // Serial.print("'"); 
    // Serial.print(trimmedString);
    // Serial.println("'");
// }

// void loop() {}

// ----------------------- ARDUINO CODE ----------------------- //

fn main() {

    // `let my_str: &str = "Hello, Rust!";` 
    // - Typ: &str (unveränderlich)
    // - Speicherort: Stack (schneller Speicher)
    // - Geschwindigkeit: Schneller, da der Stack direkt adressierbar ist.

    // `let my_string: String = String::from("Hello, Rust!");` 
    // - Typ: String (veränderlich)
    // - Speicherort: Heap (langsamer Speicher)
    // - Geschwindigkeit: Langsamer, da Heap-Speicher dynamisch verwaltet werden muss.


    let mut my_string = String::from("Hello, Rust!");
    let reversed_string: String = my_string.chars().rev().collect();

    println!("{}", reversed_string);

    my_string.pop(); // Entfernt das letzte Zeichen\
    println!("{}", my_string);

    let new_string = my_string.replace("Hello", "Howdie");
    println!("{}", new_string);

    let parts: Vec<&str> = my_string.split(", ").collect();
    println!("{:?}", parts);

    let length = my_string.len();
    println!("Länge: {}", length);

    let upper_string = my_string.to_uppercase();
    println!("{}", upper_string);

    let lower_string = my_string.to_lowercase();
    println!("{}", lower_string);

    let contains_hello = my_string.contains("Hello");
    println!("{}", contains_hello);

    let my_string = String::from("   Hello, Rust!   ");
    let trimmed_string = my_string.trim();
    println!("'{}'", trimmed_string);


}
