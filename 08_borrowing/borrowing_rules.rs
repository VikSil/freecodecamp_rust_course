fn main() {
    let mut s: String = String::from("Hello");
    let r1 = &s; // if either or both of these were mutable the println would fail
    let r2 = &s;

    println!("{}, {}", r1, r2);
    println!("s is still valid and contains {}", s);

    println!("Success!");
}
