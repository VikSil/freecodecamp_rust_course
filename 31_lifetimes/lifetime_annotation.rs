// borrow checker uses explicit lifetime annotations to determine for how long a reference should be valid
// lifetimes in funciton signatures have these constraints:
// any reference must have an annotated lifetime
// any reference that is returned/output must either be static or have the same lifetime as the input

// lifetimes are annotated with crocodile braces, an appostrophe and lower case letter, by convention starting at a, e.g. <'a>

// x is input reference with lifetime 'a
// the reference must live at least as long as the function
// so that the function always has a valid value when it needs it
fn print_one<'a>(x: &'a i32) {
    // x is a reference of lifetime a to an i32
    println!("print_one: x is {}", x);
}

// mutable references with lifetimes are also possible
fn add_one<'a>(x: &'a mut i32) {
    // x is a reference of lifetime a to a mutable i32
    *x += 1;
}

// multiple elements with different lifetimes
// all must live at least as long as the function
// here both can have the same lifetime 'a
// in more complex cases separate lifetimes may be required
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x is {}, y is {}", x, y);
}

// it is ok to return references that have been previously passed in
// but they must be returned with the correct lifetime
fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x // x is returned, since the function is decalred to return a reference of lifetime 'a
}

// since either value can be returned, both have to be of the same lifetime
// lifetime parameter is used only once, i.e. <'a> instead of <'a, 'a>
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 'a must live longer than the function
// here &String::from("foo") would create a String and then reference it
// then upon exiting the function the String would be dropped
// leaving the reference dangling

// fn invalid_output<'a>() ->&'a String {
// &String::from("foo") // this would panic at compile time
// }

// there are several ways to fix this

// return the string itself
fn valid_output1() -> String {
    String::from("foo") // ownership of the string is passed outside the function
}

// return a string literal, i.e. the pointer
fn valid_output2() -> &'static str {
    //&str always has static lifetime - it is valid throughout the program
    "foo" // this will be hardcoded into the binary and exist for as long as the program runs, hence static lifetime
}

//&String is automatically inferred to &str
fn valid_output3<'a>(s: &'a String) -> &'a str {
    s
}

// both lifetimes 'a and 'b must be at least as long as the function
// but they can differ between themselves
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// this function takes no arguments, but has a lifetime parameter 'a
// i.e. whatever has lifetime 'a must outlive the function
// i.e. it must be something that is not declared inside of the function
// otherwise it ceases to exist when the function exits
fn failed_borrow<'a>() {
    let _x: i32 = 12;

    // attempting to use the lifetime 'a as an explicit type annotation
    // inside the function will fail
    // because the lifetime of '&_x' is shorter than 'a
    // a short lifetime cannot be coerced into a longer one

    // let y: &'a i32 = &_x; // this would panic at compile time
    let y: &i32 = &_x; // this is ok because it does not refer to an outside lifetime
}

fn main() {
    // both of these outlive all the functions
    let x: i32 = 7;
    let y: i32 = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z: &i32 = pass_x(&x, &y);
    print_one(z);

    let mut t: i32 = 3;
    add_one(&mut t);

    //------------------------------------------------

    let x: &str = "long";
    let y: &str = "longer";

    println!("{}", longest(x, y));

    //------------------------------------------------

    let x: String = valid_output1();
    println!("{}", x);

    let y: &str = valid_output2();
    println!("{}", y);

    let input: String = String::from("foo");
    let z: &str = valid_output3(&input);
    println!("{}", z);

    //------------------------------------------------

    let (four, nine) = (4, 9);
    print_refs(&four, &nine); // both values are borrowed and passed into the function
    // any value that is borrowed mus toutlive the borrower,
    // i.e. the lifetime of four and nine must be longer than that of print_refs()

    failed_borrow();
    // nothing is passed into this function, but it references to lifetime 'a in its definition
    // because 'a is never constrained by an actual value,
    // lifetime of the function defaults to static
}
