// these are references to modules that are not held directly in this lib.rs file (although they could be), but are to be found at the same level as the lib.rs file
pub mod front_of_house;
pub mod back_of_house;

pub fn eat_at_restaurant() -> String {
    crate::front_of_house::hosting::add_to_waitlist(); // absolute path
    back_of_house::cook_order(); // relative path

    String::from("yum!")
}
