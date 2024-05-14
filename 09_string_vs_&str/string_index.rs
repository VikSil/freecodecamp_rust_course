fn main() {
    let s1: String = String::from("hi, 张伟"); // ascii chars are 1 byte, unicode chars are 3 or 4 bytes
    //let h = s1[0]; // rust does not have string indexing, this would panic
    let h: &str = &s1[0..1]; // rust requires the use of string slices instead, and upper bound in a slice is excluded
    assert_eq!(h, "h"); // the double quotes because h is a slice of a string, not a char

    let h1 = &s1[4..7]; // slice of three because the unicode char is 3 bytes
    assert_eq!(h1, "张");
    println!("{}, {}", h, h1);

    println!("Success!");
}
