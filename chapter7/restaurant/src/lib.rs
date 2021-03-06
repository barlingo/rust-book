mod back_of_house;
mod front_of_house;
pub use crate::front_of_house::hosting;

fn serve_order() {}

pub fn eat_at_restaurent() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
