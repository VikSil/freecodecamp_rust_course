// diverging functions never return to the caller - they are used when value of any type is expected

fn main() {
    get_option(2);
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        // match in rust is like switch/ case block
        1 => {}

        _ => {} // underscore means the default case
    }

    // Rather than returning None, a diverging function is used instead

    never_return_fn_2()
}

// all of the below are diverging functions

fn never_return_fn_1() -> ! {
    panic!() // will exit the program, will exit with "explicit panic" message
}

fn never_return_fn_2() -> ! {
    unimplemented!() // placeholder, will panic and exit with "not implemented" error
}

fn never_return_fn_3() -> ! {
    todo!() // placeholder, will panic and exit with "not yet implemented" error
}
