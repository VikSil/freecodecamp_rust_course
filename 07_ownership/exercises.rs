fn main() {
    let t: (String, String) = (String::from("Hello"), String::from("World!"));

    let _s: String = t.0; // tuples are zero indexed like arrays
    // ownership of "Hello" was transfered to _s and it is impossible to access t or t.0

    println!("{:?}", t.1); // but is is possible to access t.1

    //------------------------------------------------

    let a: (String, String) = (String::from("Hello"), String::from("World!"));

    let (s1, s2) = a.clone(); // clone is needed, otherwise both values will change ownership and a will no longer exist.

    println!("{:?}, {:?}, {:?}", s1, s2, a); // will output "Hello", "World!", ("Hello", "World!")
}
