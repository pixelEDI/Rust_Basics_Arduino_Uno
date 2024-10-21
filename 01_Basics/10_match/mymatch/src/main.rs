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

fn main() {


    let number = 8; 

    // if number < 10 {
    //     println!("Die Zahl ist kleiner als 10.");
    // } else if number < 80 {
    //     println!("Die Zahl ist zwischen 10 und 79.");
    // } else {
    //     println!("Die Zahl ist 80 oder größer.");
    // }


    match number {
        0 => println!("Die Zahl ist 0"),
        1..=10 =>  println!("Zahl zwischen 1 und 10"),
        11..=80 =>  println!("Zahl zwischen 11 und 80"),
        _ =>  println!("Größer als 80"),
    }


// Vorteile von match gegenüber if:
//  https://rustc-dev-guide.rust-lang.org/pat-exhaustive-checking.html
// 1. **Klarere Struktur:**
//    Match macht den Code übersichtlicher, besonders bei vielen Bedingungen.
//
// 2. **Sichere Überprüfung:**
//    Der Compiler zwingt dich, alle möglichen Werte zu prüfen. Fehlende Fälle führen zu Fehlern.
//
// 3. **Einfaches Pattern Matching:**
//    Du kannst komplexe Datenstrukturen leicht abfragen, z.B. bei Enums.
//
// 4. **Weniger Fehleranfällig:**
//    Match ist klarer und reduziert die Wahrscheinlichkeit von logischen Fehlern.


    enum Farbe {
        Red,
        Green,
        Blue,
    }

    let my_color = Farbe::Red;

    match my_color {
        Farbe::Blue => println!("Blau"),
        Farbe::Red => println!("Rot"),
        Farbe::Green => println!("Grün"),
        _ =>  println!("Farbe nicht bekannt"),
    }


}
