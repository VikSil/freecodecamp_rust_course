fn main() {
    let s: String = String::from("Hello, ");

    let mut s1 = s; // mutability belongs to the owner not the value, i.e. imutable values can become mutable when owenrship is trasferred

    s1.push_str("World!");
    println!("{}", s1);

    println!("Success!");

    //------------------------------------------------

    let x: Box<i32> = Box::new(5); // boxed integer, this allows to allocate the value into the heap and give a pointer to the owner
    let mut y: Box<i32> = Box::new(1);

    *y = 4; // the * is called dereferrencing. It is necessary because y owns a pointer, not i32 in the stack. I.e. *this-place-in-memory = value

    assert_eq!(*x, 5); // x itself is not five, it is smthn like 0x00772873 , defereferrencing returns the value in that address

    println!("Success!");
}
