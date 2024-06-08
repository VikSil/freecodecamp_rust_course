// panic! macro is the simplest form of handling errors
// panic! will print out an error message, unwind the stack and exit the program
// a multi-threaded program will exit only the thread in which the panic occured, unless it is the main thread

// by default panic only shows the top line of the trace and which line of code caused it
// run with RUST_BACKTRACE=1 env variable to display the backtrace

// unwinding stack takes a lot of work
// it is possible to just quit without cleaning up the mess
// adding the below to Cargo.toml will switch from unwiding to quiting
// [profile.release]
// panic = 'abort'
// this will result in a smaller binary file

use std::env; // library to manipulate environemnt from the source code

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        panic!("Going to exit via panic");
    }

    println!("If this line is being printed then the panic did not occur");
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // set env variable for full stack trace

    drink("lemonade");

    println!("If this line is being printed then the panic did not occur");
}
