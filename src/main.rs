use crate::c4::slice::first_word;

mod c1;
mod c2;
mod c3;
mod c4;

fn main() {
    // String slices
    let s = String::from("Hello World!");

    println!("{}", c4::slice::first_word(&s));
    println!("{}", c4::slice::first_word("Goodbye World!"));

    // Array slices
    let sentences = ["Hello World", "Goodbye World", "Good Morning", "Afternoon"];

    for &s in sentences[1..].iter() {
        let word = first_word(s);
        println!("{word}");
    }
}