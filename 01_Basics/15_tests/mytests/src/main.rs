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


fn add(a: i32, b: i32) -> i32 {
    a + b
}



fn main() {
    println!("Verwende 'cargo test' und nicht 'cargo run'");
    // println!("{}",add(10,5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers(){
        assert_eq!(add(2,3),5);
    }

    #[test]
    fn test_add_negative_numbers(){
        assert_eq!(add(-2,-3),-5);
    }

    #[test]
    fn test_add_mixed_numbers(){
        assert_eq!(add(-2,3),1);
    }
}

