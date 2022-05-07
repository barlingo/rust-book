use std::io;
const C_TO_F_MULT: f32 = 1.8;
const OFFSET: f32 = 32.0;

fn main() {
    let is_f_conv = select_conversion();
    let value = enter_value();
    let result = conversion(&value, &is_f_conv);
    if is_f_conv {
        println!("{}F is {}C", value, result);
    } else {
        println!("{}C is {}F", value, result);
    }
}

fn select_conversion() -> bool {
    println!("select unit conversion:\n1. F to C\n2. C to F");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read number");
        match input.trim().parse::<u8>() {
            Ok(n) => {
                if n == 1 {
                    println!("Selected F to C conversion");
                    return true;
                } else if n == 2 {
                    println!("Selected C to F conversion");
                    return false;
                } else {
                    println!("Failed to read number");
                }
            }
            Err(_e) => println!("Invalid entry"),
        }
    }
}

fn enter_value() -> f32 {
    println!("Enter value to convert:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read number");
        match input.trim().parse::<f32>() {
            Ok(n) => {
                return n;
            }
            Err(e) => println!("{}\nInvalid entry", e),
        }
    }
}

fn conversion(value: &f32, is_f_conv: &bool) -> f32 {
    if *is_f_conv {
        (value - OFFSET) / C_TO_F_MULT
    } else {
        (value * C_TO_F_MULT) + OFFSET
    }
}
