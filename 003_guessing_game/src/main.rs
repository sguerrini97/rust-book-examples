use std::io::{self, Write};

fn main() {
    println!("Guess the number!");

    // If we use print! instead of println!, the cursor stays on the same line ...
    print!("Please input your guess: ");
    // ... but we have to flush the buffer manually to ensure the output is displayed immediately.
    io::stdout().flush().expect("Failed to flush");

    let mut guess = String::with_capacity(6);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
