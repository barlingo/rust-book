use std::fmt::Display;

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is {}", self.x);
        } else {
            println!("The largest number is {}", self.y);
        }
    }
}
fn main() {
    println!("Hello, world!");
}