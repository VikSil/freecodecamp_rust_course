// RULES OF BORROWING:

// 1. There can only be ONE mutable reference OR any number of immutable references simultaneously
// 2. References must always be valid

fn main() {
    let mut s = String::from("Hello ");

    let v1 = &s;
    let v2 = &s;

    println!("{} and {} are v1 and v2", v1, v2); // this is ok, because both of the references are immutable

    let v3 = &mut s;
    println!("{} is v3", v3); // this is ok because it is a single mutable reference. No idea why it's ok to have valid immutable references in the same scope, but it works

    //println!("{}, {} and {} are v1, v2 and v3", v1, v2, v3); // this would panic because immutable and mutable references are made at the same time

    // let reference_to_nothing = dangle(); // would violate second rule and cause a panic
    let legit_reference = no_dangle();
    println!("Success!");
}

//fn dangle() -> &String { // compiler won't allow this function to even exist in the code, even if it is not used by anyone
//    let s = String::from("Hello");
//    &s // would return a pointer to something that no longer exists once we're outside of this scope - i.e. invalid reference
//}

fn no_dangle() -> String {
    let s = String::from("Hello");
    s // passes on ownership of the value held by s
}
