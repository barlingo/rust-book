// enum only need to have declared pub before the enum keyword
pub enum Appetizer {
    Soup,
    Salad,
}
// struct require to declare each field pub
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
// accessing private fields is possible through an impl
impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
}
fn cook_order() {}
