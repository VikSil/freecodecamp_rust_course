fn main() {
    let mut s: String = String::from("Hello ");

    let r1: &mut String = &mut s;
    r1.push_str("World");
    // r1 is not used past this line, hence it is ok to have another mutable reference later on

    let r2: &mut String = &mut s;
    r2.push_str("!");

    // println!("{}", r1); // commenting this out allows the two mutable references coexist because r1 is not used past line 6
    println!("{}", r2); //  this is ok, because it prints out the later variable and does not expect r1 to coexist with r2

    //------------------------------------------------

    let s1: &mut String = &mut s;
    let s2: &mut String = &mut s;

    // println!("{}, {}", r1, r2); // would panic because cannot have two mutable references at the same time

    println!("Success!");
}