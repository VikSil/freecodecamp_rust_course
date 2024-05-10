use std::mem::size_of_val;

fn main() {
    let v: () = (); // unit type is a type that does not hold any value and has a size of zero bytes

    let _v: (i32, i32) = (2, 3); // tuple of two i32 type values
    assert_eq!(v, implicitly_ret_unit()); // comparison of two unit types

    println!("Success!");

    //------------------------------------------------

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

fn implicitly_ret_unit() {
    // if a function does not return any value, it implicitly returns a unit type
    println!("I will return a ()");
}

fn explicitly_ret_unit() -> () {
    // same as above, but annotated
    println!("I will return a ()");
}
