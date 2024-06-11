fn main() {
    // funciton with same functionality as the closures below
    fn function(i: i32) -> i32 {
        i + 1
    }

    // clousures are annonymous, hence they need to be assigned to variables to call them

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i= 1;
    // calling the function and the closures
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // a closure taking no arguments, returning i32
    // the return type is inferred
    let one = || 1;

    println!("closure returning one: {}",one());
}
