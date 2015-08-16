extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

/* The guessing game as shown in the Rust Book */

fn main() {
    println!("Guess the number.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        /* A few remarks about the following line:
         * read_line() returns an io::Result
         *
         * The .ok() method of io::Result consumes the result, and converts it to an Option<T>,
         * discarding the error (if any).
         *
         * The .expect() method of Option<T> unwraps the value stored in the option, or panics with
         * the specified message if there is no value to unwrap.
         */

        io::stdin().read_line(&mut guess).ok().expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),

            Ordering::Greater => println!("Too big."),

            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
