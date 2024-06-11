// a static trait bound means that the type with that trait does not contain any non-static references
// this means that the receiver can hold onto the type for as long as they want and the type will never become invalid until the receiver drops the type
// any owned data always passes 'static lifetime check, but a reference to that data may not

use std::fmt::Debug;

// input argument must be of generic type T implementing Debug trait and all parts of it having static lifetimes
fn print_it<T: Debug + 'static>(input: T) {
    println!("static value passed in is: {:?}", input);
}

// input must implement Debug trait and be static
fn print_it1(input: impl Debug + 'static) {
    println!("static value passed in is {:?}", input);
}

// input must be a reference to a generic type T that implements Debug trait and only has static parts
fn print_it2<T: Debug + 'static>(input: &T) {
    println!("static value passed in is {:?}", input);
}

fn main() {
    let i: i32 = 5; // all primitive types implement Debug
    const j: i32 = 5;
    static k: i32 = 5;

    // this works because i owns its value, thus it is static
    print_it(i);

    // this will panic because reference to i only has the lifetime of scope of main
    // thus it is not static, but the trait bound expects static input
    // print_it(&i);
    print_it(&j); // this works because j is const and thus lives forever
    print_it(&k); // this works because k is explicitly static

    // this will panic because reference to i only has the lifetime of scope of main
    // thus it is not static, but the trait bound expects static input
    // print_it1(&i);
    print_it1(&j); // this works because j is const and thus lives forever
    print_it1(&k); // this works because k is explicitly static

    // this does not panic because fn expects reference to a static variable, not the variable itself
    print_it2(&i);
}
