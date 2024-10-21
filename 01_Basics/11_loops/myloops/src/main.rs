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


#[allow(dead_code)]
fn simple_for() {
    let numbers = vec![1, 2, 3, 4, 5];

    for number in &numbers {
        if number % 2 == 0 {
            println!("{} ist eine gerade Zahl", number);
        } else {
            println!("{} ungerade Zahl", number);
        }
    }
}

#[allow(dead_code)]
fn simple_while() {
    let mut count = 0;

    while count < 10 {
        println!("Counter up {}",count);
        count +=1;
    }
}

#[allow(dead_code)]
fn simple_loop() {
    let mut counter = 0;
    loop{
        println!("Zähler ist: {}", counter);
        counter +=1;

        if counter >= 70{
            break;
        }
    }

    println!("Loop wurde nach 70 Durchgängen beendet.");
}

fn main() {
    // simple_for();

    // simple_while();

    simple_loop();
}
