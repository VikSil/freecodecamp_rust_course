// String is a collection type - it is a collection of chars
// std::string::Strng is a UTF-8 encoded, growable string. It is the most common string type and it has ownership over the contents

fn main() {
    let mut s: String = String::from("Hello, "); // String type from string literal type
    s.push_str("World"); // push_str requires a string literal type &s - use double quotes
    s.push('!'); // push requires char - use single quotes

    move_ownership(s.clone()); // give a clone of the variable to the function to own
    assert_eq!(s, "Hello, World!"); // because we will need the variable to still own it's value for equality assertion
    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" has been moved to this function!", s)
}
