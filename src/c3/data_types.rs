use std::io;
pub fn scalar() {
    // Default i32 type
    let x = 99;
    println!("size of x -> {} bits", size_of::<i32>() * 8);
    let y: usize = 30;
    println!("architecture size -> {} bits", size_of::<usize>() * 8);

    // Mathematical operations
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f = false;

    // Chars
    let monkey = '🐒';
    let banana = '\u{1F34C}';
    println!("{monkey} 💖 {banana}");
}
pub fn compound() {
    // Tuples
    let mut tuple = (7, 37, 64);
    let (x, y, z) = tuple;
    println!("tuple.1 -> {x}");
    println!("tuple.2 -> {y}");
    println!("tuple.3 -> {z}");

    let tuple2 = (40, 50, 60);
    println!("1st -> {}", tuple2.0);
    println!("2nd -> {}", tuple2.1);
    println!("3rd -> {}", tuple2.2);

    tuple.1 += 5;
    tuple.2 *= 4;
    println!("{:?}", tuple);

    // Arrays
    let days: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday",
        "Friday", "Saturday", "Sunday"];

    let thirty_zeros = [0; 30];

    println!("What is your favourite day?");
    let mut favourite_day = String::new();

    io::stdin()
        .read_line(&mut favourite_day)
        .expect("Failed to read line.");

    let favourite_day: usize = favourite_day
        .trim()
        .parse()
        .expect("Index not a number.");

    let day = days[favourite_day];
    println!("Your favourite day is -> {day}");
}