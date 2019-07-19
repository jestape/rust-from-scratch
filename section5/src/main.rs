#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width >= rectangle.width && self.height >= rectangle.height
    }
}

fn main() {

    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the {:?} is {} square pixels.",
        rect1, rect1.area()
    );

    let rect2 = Rectangle { width: 40, ..rect1 };
    
    println!(
        "The area of the {:?} is {} square pixels.",
        rect2, rect2.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sq1 = Rectangle::square(20);

    println!("Can rect1 hold sq1? {}", rect1.can_hold(&sq1));
}
