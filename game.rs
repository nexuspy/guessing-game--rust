use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess:");

        // Read the player's guess from the user input
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Convert the guess to an unsigned integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        // Provide feedback on the guess
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too low! Try again."),
            std::cmp::Ordering::Greater => println!("Too high! Try again."),
            std::cmp::Ordering::Equal => {
                println!("Congratulations! You guessed the correct number: {}", secret_number);
                break;
            }
        }
    }
}
