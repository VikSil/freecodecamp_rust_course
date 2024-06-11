fn create_closure() -> impl Fn(i32) -> i32 {
    // static dispatch annotation, Fn is sufficient because num does not get mutated
    let num: i32 = 5; // num is declared inside a function, hence it will only live for as long as this function runs

    // move keyword takes ownership of num
    move |x| x + num // this entire closure will be returned, including num which it owns
}

fn create_closure_dyn() -> Box<dyn Fn(i32) -> i32> {
    // dynamic dispatch with boxed trait object
    let num: i32 = 5;
    Box::new(move |x| x + num)
}

// fn factory(x: i32) ->  impl Fn(i32)->i32 {
// would not work because it is not known which closure will be returned
// they look the same, but they are two separate objects in memory
// this needs to be explicitly addressed by boxing both
// and boxing requires dynamic dispatch
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num: i32 = 5;

    if x > 1 {
        Box::new(move |x| x + num) // this is the same value but a different closure object in memory (even if it was not boxed)
    } else {
        Box::new(move |x| x + num) // this is the same value but a different closure object in memory
    }
}

fn main() {
    let fn_static = create_closure(); // fn_main will hold a closure that was returned by the function
    fn_static(1); // this is calling the closure
    println!("{}", fn_static(1));

    let fn_dyn = create_closure_dyn();
    fn_dyn(2);
    println!("{}", fn_dyn(2));
}
