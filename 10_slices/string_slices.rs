fn main() {
    let s: String = String::from("hello");

    let slice1: &str = &s[0..2];
    let slice2: &str = &s[..2]; // works the same way as the line above

    assert_eq!(slice1, slice2);

    println!("{}", slice1);

    println!("Success!");

    //------------------------------------------------

    let s3: &str = "你好, 张伟";

    let slice3: &str = &s3[0..3]; // it's 3 bytes because the chinese character takes 3 bytes in memory

    assert!(slice3 == "你");

    println!("Success!");

    //------------------------------------------------

    let mut s4: String = String::from("Hello World!");

    let letter: &str = first_letter(&s4); // function expects &str, and s4 of type String can be implicitly converted by referencing it

    // s4.clear(); // this clears the string, but does not drop it, i.e. s4 now owns a value of length zero, it requires a mutable reference, otherwise panic

    println!("The first letter is: {}", letter); // this would panic at runtime if s4 was already cleared, because it would try to access the first letter in a now empty string

    s4.clear(); // this is fine because letter is not used going forward

    println!("Success!");
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}
