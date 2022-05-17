fn main() {
    let x: i32 = 21;
    let get_answer = |y: i32| x + y;
    println!("{}", get_answer(5));
}
