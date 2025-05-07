use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn guessing_game() {
    println!("Guess the number!");

    // Secret number RNG
    let secret_number = rand::rng().random_range(1..=100);

    // Game Loop
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // User input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Convert user input (string) into a number (u32)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Error handling
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        // Compare guess to generated number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                // Exit game loop
                println!("You guessed the number! You win!");
                break;
            }
        }
    }
}