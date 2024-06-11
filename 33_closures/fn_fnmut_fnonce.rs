// when closure is taken as an input parameter its complete type must be annotated using one of the following traits:
// Fn - the closure uses the captured value by reference (&T)
// FnMut - the closure uses the captured value by mutable reference (&mut T)
// FnOnce - the closure uses the captured value by ownership (T)
// if annotated by a more restrictive trait than necessary, the compiler will automatically fall back to the least restrictive trait possible.
// e.g. if closure is annotated with FnOnce, but move is not necessary, a borrow will be implemented, since if move is possible, borrow is also possible (but not vice versa)
// in the same vein move only annotates how the variables are captured (i.e. by FnOnce), but the actual traits that compilier will implement are determined by what the closure does with the values, not how they were captured

use std::mem;

fn fn_once<F>(func: F) where F: FnOnce(usize) -> bool {
    // function takes closure F as an argument
    // trait bound defines type (FnOnce) and return type of the closure
    println!("{}", func(3)); // use the closure inside the function
    // println!("{}", func(4)); // can only use it once, sice FnOnce trait
}

// this function takes in closure with Fn trait bound
fn fn_many<F>(func: F) where F: Fn(usize) -> bool {
    println!("{}", func(3));
    println!("{}", func(4)); // Fn trait bound is not restricted in how many times it can be called
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    // FnMut is least restrictive capturing
    f("hello") // this must have lifetime 'a , i.e. longet than the function exec
}

fn execute_owned<F: FnOnce()>(f: F) {
    // values will be moved into the closure F because it is FnOnce
    f()
}

fn execute_borrowed<F: Fn()>(f: F) {
    // values will only be borrowed because it is Fn
    f()
}

// function takes closure as an argument and calls it
fn apply<F>(f: F) where F: FnOnce() {
    // from below we know that closure has to be FnOnce, it does not take any arguments and does not return anything, hence annotation F: FnOnce()
    f()
}

// function takes a closure and returns i32, the result of execution of that closure
fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}

fn push_hello<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
    f("hello");
}

fn main() {
    let x: Vec<i32> = vec![1, 2, 3];

    // the closure |z| {z == x.len()} is passed as a function argument
    fn_once(|z| { z == x.len() }); // passed to fn_once it will be evaluated and print 'true'

    fn_many(|z| { z == x.len() }); // will evaluate twice inside the function and print 'true', then 'false'

    //------------------------------------------------

    let mut s: String = String::new();
    let update_string = |str| s.push_str(str); // closure captures s and appends to it whatever it was given by caller

    exec(update_string);

    println!("{:?}", s);

    //------------------------------------------------

    let greeting: &str = "hello";

    // to_owned creates owned data from borrowed
    let mut farewell: String = "goodbye".to_owned();
    // in case of &str it converts it to String (reference to type)

    // this closure captures both greeting and farewell
    // look at how both are used to decide which annotation is needed
    let diary = || {
        // greeting is by reference, it requires Fn
        println!("I said {}", greeting);

        // farewell has to be mutable, it requires FnMut
        farewell.push_str("!!!");
        println!("Then I shouted {}", farewell);

        // manual drop requires farewell to be owned
        // it requires FnOnce
        mem::drop(farewell);
    };

    apply(diary); // call function which executes the closure

    // double satisfies the apply_to_3 trait bound of Fn with i32 input and output
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    //------------------------------------------------

    let st = String::new();
    // even though st is moved into the closure, it would only be borrowed for println
    let new_string = move || println!("{}", st);

    // however execute is FnOnce annotated, hence st will be actually owned
    execute_owned(new_string);

    let st2 = String::new();
    let new_string2 = move || println!("{}", st2); // move to borrow
    execute_borrowed(new_string2); // this is Fn function, the value is still only borrowed

    //------------------------------------------------

    let mut st3 = String::new();

    // this closure is returning st3, hence it have to own it in the first place, hence function that uses this closure has to be FnOnce, it is not sufficient to be FnMut
    let upd_string = |str| -> String {
        st3.push_str(str);
        st3
    };
    push_hello(upd_string);
}
