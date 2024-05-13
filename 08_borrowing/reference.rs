fn main() {
    let x: i32 = 5; // of type i32, it lives in stack
    let p: &i32 = &x; // of type reference to i32, it lives in stack and points to stack where x and 5 are stored. x and 5 are the same thing, it's a 5 in stack

    println!("the memory address of x is {:p}", p); // {:p} is syntax for outputing memory addresses/pointers, it is {:p} regardless of the variable name

    //------------------------------------------------

    let a: i32 = 5;
    let b: &i32 = &a;

    assert_eq!(5, a); // this is correct
    // assert_eq!(5,b); // this would panic because b is a memory address to where a is and there is no way of knowing it in advance
    assert_eq!(5, *b); // this is correct - dereferencing b, i.e. returning the value that it points to
    println!("Success!");

    //------------------------------------------------

    let mut s: String = String::from("Hello, ");

    borrow_object(&s); // this needs to have non-mutable reference because that is what the function expects

    println!("Success!");

    //------------------------------------------------

    let mut k: String = String::from("Hello, ");

    push_str(&mut k); // this needs to be a mutable reference because that is what the function expects
    println!("Success!");

    //------------------------------------------------

    let mut m: String = String::from("Hello, ");

    let p = &mut m;
    p.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}

fn push_str(s: &mut String) -> () {
    // this function performs an operation, but the return value is unit type
    s.push_str(" World!")
}
