fn main() {
    let mut s: String = String::from(""); // initilise an empty String as String, not as a string slice &str, because slices are immutable
    s.push_str("Hello, World"); // push character sequence
    s.push('!'); // push a single character

    assert_eq!(s, "Hello, World!");

    println!("{}", s);
    println!("Success!");

    //------------------------------------------------

    // let s1: String = String::from("Hello"); // would panic because s1 needs to be mutable
    let mut s1: String = String::from("Hello");

    s1.push(','); // pushing a single char, hence push and single quotes
    // s1.push(" world"); // would panic because pushing a sequence, need to use push_str instead
    s1.push_str(" world");
    // s1 += "!".to_string(); // would panic because "!" is already a string literal (which is what s1 expects)
    s1 += "!";

    println!("{}", s1);

    //------------------------------------------------

    let mut s2: String = String::from("I like dogs");

    let s3 = s2.replace("dogs", "cats"); // s2 has to be mutable

    assert_eq!(s3, "I like cats");

    println!("Success!");

    //------------------------------------------------

    let s4: String = String::from("Hello,");
    let s5: String = String::from(" World");
    // String can only be concatenated with &str variables, and String's ownership can be moved to another variable
    let s6 = s4 + s5.as_str(); // .as_str() makes the compiler consider a String as a &str slice
    // s4 no longer owns the "Hello, " string, it's s6 who now owns it

    assert_eq!(s6, "Hello, World");
    // println!("{}", s4); // this would panic since s4 no longer owns anything, i.e. it is not valid

    println!("Success!");

    //------------------------------------------------

    let s7: String = String::from("Hello,");
    let s8: String = String::from(" World");

    let s9 = s7 + &s8; // works the same way as the previous example

    assert_eq!(s9, "Hello, World");

    println!("Success!");
}
