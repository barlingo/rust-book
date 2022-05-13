fn main() {}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // largest is a reference to the first item of the list
    let mut largest = &list[0];
    // travel the list BY reference
    for num in list {
        // need to dereference both num and largest to get the actual value
        if *num > *largest {
            // num now points to largest item
            largest = &num;
        }
    }
    largest
}

#[test]
fn num_list() {
    let num_list = vec![23, 100, 43, 52, 10, 65];
    assert_eq!(100, *largest(&num_list));
}
#[test]
fn str_list() {
    let str_list = vec!["a", "b", "c", "d", "e", "f"];
    assert_eq!("f", *largest(&str_list));
}
#[test]
fn str_list_one_empty() {
    let str_list = vec!["", "b", "c", "d", "e", "f"];
    assert_eq!("f", *largest(&str_list));
}
#[test]
fn char_list() {
    let char_list = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    assert_eq!('f', *largest(&char_list));
}
