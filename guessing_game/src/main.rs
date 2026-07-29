use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Welcome to Guess The Number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("\nMake a guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
