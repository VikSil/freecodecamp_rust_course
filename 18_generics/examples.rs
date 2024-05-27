// Generics are placeholders for different types
// they allow to call the same function passing differnet variable types on different calls
// Using generics enables more reusable and flexible code
// Generics help avoid having duplicate code for different types
// Rust compiler will fill out generics with concrete types at compile time
// Const generics represent compile-time constant values
// Constant generics are used for array sizes, bit widths and other constants that are known in advance

struct A; // Concrete struct type A
struct S(A); // Concrete struct type S
struct SGen<T>(T); // Generic struc type SGen of type T can hold any type

// non-generic functions
fn reg_fn(_s: S) {} // takes concrete type as argument
fn gen_spec_t(_s: SGen<A>) {} // takes implicitly specified type A
fn gen_spec_i32(_s: SGen<i32>) {} // takes implicitly specified type i32

fn generic_func<T>(_s: SGen<T>) {} // generic function takes an instance of any type as its argument

// : std::ops::Add<Output = T> - needs to be declared for the function to be able to use plus (+) sign in the body
fn sum<T: std::ops::Add<Output = T>>(var1: T, var2: T) -> T {
    var1 + var2
}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(7));

    generic_func::<char>(SGen('g')); // calls generic function, specifying the type and value of the argument
    // generic_func::<char>(SGen(7.7)); // this would panic because the generic_func is annotated to expect type char

    generic_func(SGen('m')); // the generic_func implicitly assumes type of char
    generic_func(SGen(7.7)); // the generic_func implicitly assumes type of f64

    println!("Success!");

    //------------------------------------------------

    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30)); // default type i32
    assert_eq!(2.46, sum(1.23, 1.23)); // default type f64

    println!("Success!");
}
