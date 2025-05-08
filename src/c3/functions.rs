pub fn calc(a: i32, b: i32) {
    println!("{a} + {b} = {}", add(a, b));
    println!("{a} - {b} = {}", sub(a, b));
    println!("{a} * {b} = {}", mult(a, b));
    println!("{a} / {b} = {}", div(a, b));
}

fn add(a: i32, b: i32) -> i32 {
    // Implicit return
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mult(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> i32 {
    a / b
}
