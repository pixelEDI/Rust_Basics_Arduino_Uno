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

// #include <Vector.h>

// void setup() {
//   Serial.begin(9600);

//   const int ELEMENT_COUNT_MAX = 5;
//   int storage_array[ELEMENT_COUNT_MAX];
//   Vector<int> vec(storage_array);

//   vec.push_back(1);
//   vec.push_back(2);
//   vec.push_back(3);

//   Serial.print("Element an Index 0: ");
//   Serial.println(vec[0]);

//   Serial.print("Element an Index 2: ");
//   Serial.println(vec[2]);

//   Serial.print("Größe des Vektors: ");
//   Serial.println(vec.size());
// }

// void loop() {}

// ----------------------- ARDUINO CODE ----------------------- //


fn main() {

    // In Rust sind Vektoren (`Vec<T>`) dynamische, veränderbare Arrays.
    // Beispiel: `let mut vec: Vec<i32> = Vec::new();` (kann wachsen und schrumpfen).

    // Eigenschaften von Vektoren:
    // 1. **Dynamische Größe**: Vektoren können während der Laufzeit Elemente hinzufügen oder entfernen.
    // 2. **Heap-Speicher**: Sie verwenden Heap-Speicher, was bedeutet, dass sie mehr Flexibilität, aber auch mehr Overhead haben als Arrays.
    // 3. **Typensicherheit**: Alle Elemente im Vektor müssen denselben Typ haben, was die Typensicherheit gewährleistet.

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec);
    println!("{}", vec.len());
    println!("{}", vec[1]);
    println!("{}", vec.contains(&3));

    println!("-----new vec2------");

    // mit macro
    let vec2 = vec!["Arduino uno", "Arduino Nano", "ESP32"];
    println!("{}", vec2.len());
    println!("{}", vec2[1]);
    println!("-----------");
    
    let search_term = "uno";
    let found = vec2.iter().any(|s| s.contains(search_term));
    println!("{}", found);
    println!("-----closures------");  
    
}
