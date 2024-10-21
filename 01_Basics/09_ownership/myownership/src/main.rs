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


fn print_message(msg1: &String, msg2: &String) { 
    println!("Meine Nachricht: {} und {}", msg1, msg2);
}

fn with_return_value(val: i32) ->  i32{
    val * val

    // // oldschool
    // let returnvalue = val * val;
    // return returnvalue;
}

fn main() {

    // Ownership in Rust regelt, wer für den Speicher eines Wertes verantwortlich ist.
    // 1. **Jeder Wert hat einen Owner**: Es gibt immer genau einen Besitzer für jeden Wert.
    // 2. **Nur ein Owner**: Ein Wert kann zu einem Zeitpunkt nur einen Besitzer haben!!!
    // 3. **Move**: Wenn der Wert übertragen wird (z.B. durch Zuweisung), verliert der ursprüngliche Besitzer Ownership.
    // 4. **Borrowing**: Werte können ausgeliehen werden (mit `&` oder `&mut`), ohne das Ownership zu ändern.

        let s1: String = String::from("Hello");
        let s2 = s1;

        println!("{}", s2);

        let s3 = String::from(" World");
        print_message(&s2, &s3);

        println!("{}", s3);

        // Bei `i32` braucht es kein Borrowing, weil `i32` ein einfacher Typ ist.
        // 1. **Wird kopiert**: `i32` wird einfach kopiert, wenn es in eine Funktion übergeben wird.
        // 2. **Kein Ownership-Problem**: Da es kopiert wird, bleibt das Original erhalten und es gibt keine Probleme mit dem Besitz.

        let val2 = 10;
        let new_val = with_return_value(val2);
        println!("{}", new_val); 
        println!("{}", val2); 

   

       
}
