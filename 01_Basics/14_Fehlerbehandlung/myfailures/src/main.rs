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

fn divide(zaehler: i32, nenner: i32) -> Result<i32, String>{
    if nenner == 0{
        return Err("Die Welt geht jetzt unter weil div/0".to_string());
    }
    Ok(zaehler/nenner)
}


fn main(){
    match divide(10, 0){
        Ok(result) => println!("Ergebnis {}", result),
        Err(e) => println!("Fehler {}", e),
    }
}