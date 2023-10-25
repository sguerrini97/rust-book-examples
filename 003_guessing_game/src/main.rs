use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    // println!("The secret number is: {}", secret_number);

    loop {
        // If we use print! instead of println!, the cursor stays on the same line ...
        print!("Please input your guess: ");
        // ... but we have to flush the buffer manually to ensure the output is displayed immediately.
        io::stdout().flush().expect("Failed to flush");

        let mut guess = String::with_capacity(6);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parsing error handling. Reusing the guess variable name will shadow the previous variable.
        let guess: u32 = match guess.trim().parse() {
            Ok(size) => {
                size
            },
            Err(_) => {
                println!("Invalid input, please enter a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            },
            Ordering::Greater => {
                println!("Too big!");
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
