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
/* 
   Generische Typen in Rust ermöglichen die Definition von Strukturen und Funktionen,
   die mit beliebigen Datentypen arbeiten können. Dies fördert die Wiederverwendbarkeit
   des Codes und reduziert Redundanz, da dieselbe Logik für verschiedene Typen verwendet 
   werden kann. Generics bieten Typensicherheit zur Kompilierzeit und helfen, Fehler 
   frühzeitig zu erkennen, was die Robustheit und Wartbarkeit des Codes verbessert. 

   Wir kümmern uns um das Was und nicht um das Wie
*/

struct Point<T> {
    x: T,
    y: T,
}


impl Point<i32>{
    pub fn area(&self) -> i32{
        self.x * self.y
    }
}

impl Point<f64>{
    pub fn area(&self) -> f64{
        self.x * self.y
    }
}

/*
   `T` ist ein Typparameter in der Struktur `Point<T>`, der einen beliebigen Datentyp darstellt.
   - Ermöglicht die Verwendung von generischen Typen.
   - Flexibel: Kann für verschiedene Datentypen (z.B. `i32`, `f64`, `String`) verwendet werden.
   - Gewährleistet Typensicherheit: Die Felder `x` und `y` müssen denselben Datentyp haben.
*/

    let int_point = Point {x:5, y:10};
    let float_point = Point {x: 3.5, y: 7.2};
    let string_point = Point {x: "links", y: "rechts"};

    println!("Integer Point: ({}, {})", int_point.x, int_point.y);
    println!("Fläche: {}", int_point.area());

    println!("Float Point: ({}, {})", float_point.x, float_point.y);
    println!("Fläche: {}", float_point.area());

    println!("String Point: ({}, {})", string_point.x, string_point.y);



}
