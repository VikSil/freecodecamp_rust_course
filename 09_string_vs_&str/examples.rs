// String - pronounced as "string", heap allocated, mutable, owns its content, made up of characters and is thus a compound type
// &str - pronounced as "string slice", stack allocated, immutable, does not own its content, points to a String
// &'static str - pronounced as "string literal", hardcoded sequence of chars that is known at compile time and valid and immutable throughout the lifetime of the program


fn main() {
    let s: String = String::from("Hello World!");

    let hello = &s[0..5]; // s remains the owner of the entire "Hello World!", since hello is a reference (indicated by &) to s
    let world = &s[6..11]; // same here, world is not taking ownership of the String, that still belongs to s

    println!("Success!");

    
}