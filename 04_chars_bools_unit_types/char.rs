use std::mem::size_of_val; //returns the size of a reserved space in memory in bytes

fn main() {
    let c1: char = 'a';
    println!("{}", size_of_val(&c1)); // will output 4, because chars are always 4
    assert_eq!(size_of_val(&c1), 4);

    let mut s: &str = "a"; // str will take as much space as necessary
    println!("{}", s.len()); // "a" will be 1 byte of length
    println!("{}", size_of_val(&s)); // the size of the variable s will be 16 bytes, because it is a string

    let c2: char = '牛'; // chars are written with single quotes
    println!("{}", size_of_val(&c2)); // will also output 4 because it is also a char
    assert_eq!(size_of_val(&c2), 4);

    s = "牛"; // strings are written with double quotes
    println!("{}", s.len()); // this will be 3 bytes because it is a chinese character as a string
    println!("{}", size_of_val(&s)); // the size of the variable s will be 16 bytes, regardless of what is the value or lenght of it


    // chars are stored directly into the stack
    // strings are stored in the heap and the stack holds a string structure:
        // 1) pointer to where the string starts in the heap
        // 2) length of the string
        // the above are both 8 bytes of data, hence 2 * 8 = 16 bytes to store a string


    let c3 = '牛'; // needs to have single quotes because the function expects a char, not a string
    print_char(c3);

    println!("Success!");
}

fn print_char(c: char) {
    println!("{}", c);
}
