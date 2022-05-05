fn main() {
    println!("Hello, world!");
    another_function(32);
    print_labeled_measurement(5, 'h');
    let y = {
        let x = 3; // statement
        x + 1 //expression
    };
    let x = plus_one(5);
}

fn another_function(x: i32) {
    println!("The value of x is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
