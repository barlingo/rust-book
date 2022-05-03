extern crate rand;

use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = thread_rng().gen_range(1, 101);
    println!("Secret Number: {}", secret_number);

    loop {
        println!("Please enter your guess [1-100]:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read number.");

        println!("{}", input);
        let input: u32 = match &input.trim().parse() {
            Ok(num) => *num,
            Err(_) => continue,
        };
        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
