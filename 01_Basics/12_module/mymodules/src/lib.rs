pub fn add(a:i32,b:i32,c:i32) -> i32{
    a+b+c
}


pub mod my_functions {
    pub fn blink_led(pin: u8) {
        // Simuliere das Blinken der LED (hier nur Platzhalter)
        println!("Blinking LED on pin {}", pin);
    }
    pub fn public_function(){
        println!("Zugriff auf streng_geheim {}", streng_geheim());

    }
    fn streng_geheim() -> i32{
         73
    }
}