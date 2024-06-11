fn main() {
    // this closure does not say anything about what type it expects or returns
    let example_closure = |x| x;
    // but the first time the closure is called, its type gets inferred
    let s: String = example_closure(String::from("hello"));

    // this would panic because by now compiler knows that closure is supposed to be a String
    // let n = example_closure(5);

    // this would work because type is correct
    let n: String = example_closure((5).to_string());

    println!("Success!");
}
