// type alias is a way of giving a new name to an existing type
// it is easy to confuse, but not the same as associated type in a trait

type U64 = u64; // wherever U64 is mentioned in the code it will be replaced by u64 at compile time

fn main() {
    let number: U64 = 42;
    println!("Success!");
}
