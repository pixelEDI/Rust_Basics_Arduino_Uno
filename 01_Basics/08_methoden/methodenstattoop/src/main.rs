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
// class Circle {
//     public:
//         float radius;
        
//         Circle(float r) : radius(r) {}
    
//         float area() {
//             return 3.14 * radius * radius;
//         }
//     };
    
//     void setup() {
//         Serial.begin(9600);
        
//         Circle c(5);
//         Serial.println(c.area());
//     }
    
//     void loop() {}
// ----------------------- ARDUINO CODE ----------------------- //

pub struct Circle {
    radius: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        // std::f32::consts::PI
        3.14 * self.radius * self.radius
    }

    fn scale(&self, val: f32) -> f32 {
        let new_radius = self.radius * val;
        Circle { radius: new_radius }.area()
    }

    
}

fn main() {
    let c = Circle { radius: 2.0};
    println!("{}", c.area());

    println!("{}", c.scale(2.0));

    let c2 = Circle { radius: 5.5};
    println!("{}", c2.area());


}
