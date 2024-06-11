// static lifetime lasts the entire duration of program execution
// any reference or borrowed value with static lifetime can be safely used throught the program
// static lifetime can be coerced to a shorter lifetime if needed
// static only indicates that the data lives forever, not any references to it. References are still constrained by their scope
// static is a reserved name in Rust

const S: &'static str = "Hello, world!"; // explicit lifetime
// const constants can change memory address - they will be inlined among other function code

fn generic<T>(x: T) where T: 'static {} // static trait bound

static NUM: i32 = 18; // constant with static lifetime
// static constants will never change memory address - they will be referenced by other function code

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM //returns a reference to NUM, thus coercing its static lifetime to lifetime 'a, since that is the output lifetime
}

fn main() {
    let v = "hello"; // this will have a static lifetime just because it is a string literal
    let annotated_v: &str = "hello"; // same as above but annotated
    let fully_annotated_v: &'static str = "hello"; // same as above but annotated with explicit lifetime
    need_static(v);
    need_static(annotated_v);
    need_static(fully_annotated_v);

    println!("Success!");

    //------------------------------------------------
    {
        let static_string: &'static str = "I am in read-only memory";
        println!("static string: {}", static_string);
    } // static_string variable goes out of scope

    // static_string (variable) can no longer be called here
    //but the "I am in read-only memory" (value) is still in the memory somewhere for as long as the program runs
    // we just cannot get to it anymore

    //------------------------------------------------
    {
        let lifetime_num: i32 = 9; // make an integer to use for coerce_static fn

        //coerce variable to lifetime of the function
        let coerced_static: &i32 = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM {} stays accessible!", NUM);
}

fn need_static(r: &'static str) {
    // function expects a string literal (which is a type that is always static)
    assert_eq!(r, "hello");
}
