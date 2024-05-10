use std::mem::size_of_val;

fn main() {
    let _f: bool = false; // underscore prefix prevents a warning about the variable never being used

    let t = false;
    if !t {
        println!("Success!");
    }

    //------------------------------------------------

    let a: bool = true;
    let b: bool = true || false;
    assert_eq!(a, b);

    println!("Success!");

    //------------------------------------------------

    let x = true;
    assert!(size_of_val(&x) == 1); // bools are 1 byte
    
    println!("Success!");

}
