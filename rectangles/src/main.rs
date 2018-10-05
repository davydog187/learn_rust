#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self: &Rectangle) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    println!("Hello, world!");

    let rect = Rectangle{ width: 100, height: 50};

    println!("my rect {:#?} has area {}", rect, rect.area());
}


