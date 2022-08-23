use std::io;
use rand::Rng; // random number generators
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // read_line returns Ok or Err (variant of Result enum)
        // If Ok, expect returns the obtained value otherwise prints line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadows the previous variable name "guess"
        // trim() removes whitespace, carriage return and newline
        // Similar to .read_line(), parse() returns a Result enum
        // We handle the enum using a match expression
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // A match expression is made up of arms and runs the 
        // arm that matches the result of the input expression (enum variant)
        // cmp returns a variant of the Ordering enum 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

