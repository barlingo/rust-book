use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    dbg!(&scores);
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<String, i32> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    dbg!(&scores);
    // Overwrite
    scores.insert(String::from("Blue"), 10);
    // Insert if it does not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    dbg!(&scores);
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", &map);
}
