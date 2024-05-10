use std::mem::size_of_val; //returns the size of a reserved space in memory in bytes

fn main() {
    let c1: char = 'a';
    println!("{}", size_of_val(&c1)); // will output 4, because chars are always 4
    assert_eq!(size_of_val(&c1), 4);

    let mut s: &str = "a"; // str will take as much space as necessary
    println!("{}", s.len()); // "a" will be 1 byte

    let c2: char = '牛'; // chars are written with single quotes
    println!("{}", size_of_val(&c2)); // will also output 4 because it is also a char
    assert_eq!(size_of_val(&c2), 4);

    s = "牛"; // strings are written with double quotes
    println!("{}", s.len()); // this will be 3 bytes because it is a chinese character as a string

    // reserved space is not the same as length

    let c3 = '牛'; // needs to have single quotes because the function expects a char, not a string
    print_char(c3);

    println!("Success!");
}

fn print_char(c: char) {
    println!("{}", c);
}
