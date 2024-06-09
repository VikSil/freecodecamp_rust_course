// the program is run by command
// cargo run
// from binary_package folder

mod front_of_house; // binary crate must import functionality from the library crate

fn main() {
    println!("Hello, world!");

    assert_eq!(binary_package::front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(binary_package::eat_at_restaurant(), "yum!");

    println!("{}", binary_package::front_of_house::hosting::seat_at_table());
    println!("{}", binary_package::eat_at_restaurant());
}
