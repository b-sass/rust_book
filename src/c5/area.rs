#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn area_struct() {
    let rect = Rectangle {
        width: 8,
        height: 16,
    };

    println!("The area of the rectangle is {} square pixels.",
             area(&rect)
    );

    println!("Rectangle -> {rect:#?}");
    dbg!(&rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}