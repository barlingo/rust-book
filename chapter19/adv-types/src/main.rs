type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    let f: Thunk = Box::new(|| println!("Hi"));
    println!("Hello, world!");
}
