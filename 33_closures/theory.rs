// a closure is an annonymous function that is able to capture values from the scope in which it is defined
// can be defined inline, e.g. as a function parameter
// don't require type annotation
// both the input and output type of closure can be inferred by the compiler
// can take ownership of value via 'move' keyword
// akin to lamda or arrow f-tions in JS
// convenient for 'on-the-fly' use

// all functions, including closures implment Fn traits
// Fn traits describe types, number of arguments and return type

// FnOnce
// f-tion can be called only once
// f-tion takes ownership of the captured values

// FnMut
// f-tion can mutate the captured values
// f-tion can be called repeatedly

// Fn
// f-tion does not take ownership of the captured values
// f-tion does not mutate anything
// f-tion might even not capture anything from its environment

// when using closures, the compiler will capture variables in the least restrictive manner possible

fn main() {
    let x = 1;
    // this closure captures value x and modifies it
    // mutable reference to x is taken rather than ownership, because it is less restrictive
    let closure = |val| val + x; // this essentially works like a map
    assert_eq!(closure(2), 3);
}
