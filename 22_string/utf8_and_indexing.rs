// String is always UTF-8
// non-UTF-8 strings are OsString type
// Strings cannot be indexed into because they are Memory Objects
// it is not clear what should an index even return, a char or byte
// Strings cannot be indexed into, but string slices can with syntax &s1[start..end]

fn main() {
    let s: String = String::from("Hello, 伟牛");
    let slice1: &str = &s[..1]; //'H' only takes 1 byte in UTF8
    assert_eq!(slice1, "H"); // slice is string literal - double quotes

    let slice2: &str = &s[7..10]; // '伟' takes 3 bytes in UTF8, hence 3 byte slice
    assert_eq!(slice2, "伟");

    for (i, c) in s.chars().enumerate() {
        // s.chars() will return each char
        // .enumerate() will return a tuple of index and the result of previous operation
        if i == 7 {
            assert_eq!(c, '伟'); // chars is char - single quotes
        }
    }

    println!("Success!");
}
