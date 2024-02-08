use std::u32;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect: Rectangle = Rectangle {
        height: 10,
        width: dbg! (504)
    };

    println!("A área do retângulo é de {}", rect.area());
}