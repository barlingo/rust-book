#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The quit variant hsa no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {}, and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, sat {s}, and value {v}")
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}")
        }
    }
    println!("Hello, world!");

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }
    println!("setting is {:?}", setting_value);
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {x}"),
    }
    let numbers = (1, 2, 3, 4, 5, 6, 7);
    match numbers {
        (first, .., last) => println!("Some numbers {first} {last}"),
    }
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even."),
        Some(x) => println!("The number {x} is odd."),
        None => (),
    }
}
