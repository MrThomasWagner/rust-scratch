mod front_of_house;
mod back_of_house;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    let nachos: back_of_house::Appetizer = back_of_house::Appetizer::Nachos;

    match nachos {
        back_of_house::Appetizer::Nachos => println!("extra sour cream with those nachos please"),
        _ => println!("Why didnt you get nachos?"),
    };
}
