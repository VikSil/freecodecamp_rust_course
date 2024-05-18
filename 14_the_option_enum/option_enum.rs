// Rust does not have the concept of null
// instead absence of a value is is handled by enum Option<T> - an enum that might have or not have a value

// this definition is refered to as "prelude"
enum Option<T> { // T is a generic that means that Some can hold any type
    None,
    Some(T), // but it is also possible to define enum Options with specific types
}

fn main() {
    let five: Option<i32> = Option::Some(5); // the tutorial says that Option:: prefix can be omitted from everywhere, but rustc panics without it
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(Option::None); // none can be passed in and out of funcions expecting and returning any type

    if let Option::Some(n) = six {
        println!("{}", n);

        println!("Success!");
    } else {
        panic!("The program panicked");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // since this function expects an option, the integer that is passed in has to be wrapped in Some to become one of the option variants
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}
