// use utf8_slice; // compiler cannot find this for some reason

fn main() {
    let example: &str = "The 🚀 is going to the 🌕!";

    // let rocket = utf8_slice::slice(example,4,5); // should equal "🚀" as a string literal

    //------------------------------------------------

    let mut s: String = String::new(); // String is Vec<u8>, i.e. a vector of u8 integers encoding chars
    s.push_str("hello");

    // some bytes in a vector
    let v: Vec<u8> = vec![104, 101, 108, 108, 111]; // thanslates to "hello" via ASCII table

    // similar to ::from_string(&str)
    let s1: String = String::from_utf8(v).unwrap(); // turns a vector into a String

    assert_eq!(s, s1); // both are "hello"

    println!("Success!");
}
