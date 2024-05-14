fn main() {
    let s: &str = "Hello World!";   // this has to be &str, not str
    println!("Success!");

    //------------------------------------------------

    // str insted of &str can be used by boxing it, i.e. by allocating it to the heap
    let s1: Box<str> = "Hello, World!".into(); // into() is typecasting method - it will convert the value into the type that is annotated for the variable
    greetings(&s1); // & can be used to refer to a boxed str

    let s2: &str = "Hello, World!";
    greetings(s2); // no need to prefix with & here, since s2 is already of type &str

}


fn greetings(s: &str) { 
    println!("{}",s);
}