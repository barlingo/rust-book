use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please enter your guess:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read number.");

    println!("You entered: {}", input);
}
