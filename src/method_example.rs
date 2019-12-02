struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn example_method() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 20 };
    let rect3 = Rectangle { length: 60, width: 20 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.calculate_area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}