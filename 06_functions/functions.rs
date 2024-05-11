fn main() {
    let (x, y) = (1, 2); // Destructuring of a tuple
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");

    //------------------------------------------------

    print();

    //------------------------------------------------

    never_return();
    println!("Failed"); // this will get an unreachable statement warning, since the line above it panics

    //------------------------------------------------
}

fn sum(x: i32, y: i32) -> i32 {
    // all arguments for a function always have to be annotated
    x + y // no semicolon so that the value gets returned
}

fn print() {
    // since function does not return anything, it will implicitly return unit type, which does not need to be annotated
    println!("Success!");
}

fn never_return() -> ! {
    // exclamation mark means it is a diverging function, i.e. a function that never returns to the caller
    // examples of a diverging function: panic, infinite loop, quiting the program
    panic!() // panic will cause the program to exit with an error, BUT compiling a panic macro statement does not cause a compilation error
}
