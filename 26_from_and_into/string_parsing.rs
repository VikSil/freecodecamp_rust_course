// parse method can be used to convert a String into i32 number
// this is possible because FromStr is implemented for i32 type in the standard library

use std::str::FromStr;

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap(); // annotating the type directly on the method is called turbofish syntax
    let from_str: i32 = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}
