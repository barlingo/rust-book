fn main() {
    let number = 7;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    if number != 0 {
        println!("number was something else than zero");
    }

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("Not divisible by 2, 3, 4");
    }

    // if let
    let condition = true;
    let number = if condition { 5 } else { "six" };
}
