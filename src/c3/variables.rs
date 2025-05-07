// Constant in global scope
const MINUTES_IN_WEEK: u32 = 60 * 24 * 7;

pub fn variables() {
    // Variables in function scope
    let mut x = 5;
    println!("x -> {x}");
    x = 6;
    println!("x -> {x}");

    // Constant reference
    println!("Number of minutes in a week -> {MINUTES_IN_WEEK}")
}

pub fn shadowing() {
    let y = 2;
    let y = y * 2;

    {
        let y = y * 4;
        println!("inner y -> {y}")
    }

    println!("outer y -> {y}")
}