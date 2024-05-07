fn main() {
    let mut x: i32 = 1;
    x = 7;

    // shadowing and re-binding
    let x = x;
    // x += 3; <-- this would fail because the redeclared x is immutable

    let y: i32 = 4;
    // shadowing
    let y: &str = "I can be text as well!";

    println!("Success!");
}