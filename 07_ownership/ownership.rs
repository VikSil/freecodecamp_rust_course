fn main() {
    let x: String = String::from("Hello world!");
    // let y: String = x; // would pass ownership of the pointer from x to y and x would be dropped

    let y: String = x.clone(); // creates a new string in heap and gives y the ownership of the pointer
    println!("{},{}", x, y); // both must exist and have independent ownerships

    //------------------------------------------------

    let s1: String = String::from("hello, world");
    let s2 = take_ownership(s1); // s2 takes ownership of the s1 value via a function

    println!("{}", s2);

    //------------------------------------------------

    let s: String = give_ownership();
    println!("{}", s);

    //------------------------------------------------

    let a: String = String::from("Hello World!");
    print_str(a.clone()); // this needs to recieve a clone not the actual ownership from the variable
    println!("{}", a);

    //------------------------------------------------

    let m: (i32, i32, (), String) = (1, 2, (), "hello".to_string()); // the string here is of unknown length at the time of compiling, i tlives in heap
    let k: (i32, i32, (), &str) = (1, 2, (), "hello"); // this string is an imutable string literal that is hardcoded, i.e. known at the time of compiling, it lives on stack
    let n: (i32, i32, (), String) = m.clone(); // ownership of m gets reassigned to n because one of the values are in the heap
    let l: (i32, i32, (), &str) = k; // rather than reassigning owner, all of these will be copied because they all live on stack

    // println!("{:?}, {:?}", m, n); // this would panic because m was droped when its ownership was reassgined
    println!("{:?}, {:?}", k, l);

    //------------------------------------------------
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn give_ownership() -> String {
    let s: String = String::from("Hello World!");

    // let _s = s.into_bytes(); // converts String to byte vector AND consumes the string
    let _s = s.as_bytes(); // creates a copy of a String to byte vector
    s
}

fn print_str(s: String) {
    println!("{}", s);
}
