use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut v = vec![2, 3, 1, 3, 4, 0, 9, 8, 12, 12, 1, 1, 1];
    let median = median(&mut v);
    let mode = mode(&v);
    println!("median: {}, mode: {}", median, mode);
}

fn median(v: &mut Vec<u32>) -> u32 {
    v.sort();
    let size: f32 = v.len() as f32;
    let middle: usize = (size / 2.0).round() as usize;
    v[middle]
}

fn mode(v: &Vec<u32>) -> u32 {
    let mut map = HashMap::new();
    for num in v {
        // if key doesn't exist it enters default 0 + 1
        // if it does exist then take a ref to the key->value
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max_val = 0;
    let mut max = 0;
    for (key, val) in map.iter() {
        if val > &max_val {
            max = **key as u32;
            max_val = *val;
        }
    }
    max
}

#[test]
fn test_median_odd() {
    let mut v = vec![2, 3, 1, 3, 4, 0, 9, 8, 12, 12, 1, 1, 1];
    assert_eq!(median(&mut v), 3);
}
#[test]
fn test_mode() {
    let mut v = vec![2, 3, 1, 3, 4, 0, 9, 8, 12, 12, 1, 1, 1];
    assert_eq!(mode(&mut v), 1);
}
#[test]
fn test_median_even() {
    let mut v = vec![2, 3, 1, 3, 4, 0, 9, 8, 12, 12, 1, 1];
    assert_eq!(median(&mut v), 3);
}
