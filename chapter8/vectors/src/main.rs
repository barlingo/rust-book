fn main() {
    println!("Hello, world!");
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(5);
    // v.push(5);
    // v.push(5);
    // v.push(5);
    // dbg!(v);
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    {
        let mut x: Vec<i32> = Vec::new();
    }
    // x.push[0];
    let third: &i32 = &v[2];
    println!("The thirds element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("Theere is no third element."),
    }
    let mut v = vec![1, 2, 3, 4, 5];
    // Panic!
    // let does_not_exist = &v[100];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
    let does_not_exist = v.get(100);
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.1),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
