extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number: u16 = rand::thread_rng().gen_range(1, 101);
    println!("Please enter your guess:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read number.");

    // if secret_number == input {
    //     println!("You guessed correctly!!");
    // } else {
    //     println!("You guessed incorrectly!!");
    // }
    println!("Secret Number: {}, Entered: {}", secret_number, input)
}
