#[derive(Debug)]
#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool { // Functions can have the same name as struct properties
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn set_to_max(&mut self, other: Rectangle) {
        // Max takes ownership of self and other
        *self = self.max(other);    // Does not compile unless struct allows copying
    }
    fn max(self, other: Rectangle) -> Self {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

pub fn methods() {
    let rect1 = Rectangle {
        width: 8,
        height: 16,
    };
    let rect2 = Rectangle {
        width: 15,
        height: 32,
    };
    let mut rect3 = Rectangle::square(20);

    println!("Area of {rect1:?} is {}", rect1.area());
    println!("Area of {rect3:?} is {}", Rectangle::area(&rect3)); // Equivalent to method call above

    if rect1.width() { println!("Rectangle has non zero width") } else { println!("Rectangle has no width") }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    // rect2.set_width(35); // Does not compile, rect2 is not mutable (set_width() needs &mut rect)
    rect3.set_width(35); // Compiles as rect3 is mutable

    rect3.set_to_max(rect2);
    println!("Rect3 is now {rect3:?}");
}