//WTF?
fn main() {
    let s1: &str = "Hello";
    let s: String = format!("{}, World!", s1);
    assert_eq!(s, "Hello, World!");

    println!("{}", s);
    println!("Success!");
}
