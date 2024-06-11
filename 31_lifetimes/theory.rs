// lifetimes are only necessary when dealing with references not owned values
// most of the time compiler can implicitly infer the lifetimes and they don't  need to be handled
// lifetimes are generics that ensure that references are valid for as long as they are needed
// every reference has a lifetime, i.e. the scope in which that reference is valid
// sometimes, when the compiler cannot infer the lifetime, annotation is needed
// lifetime annotation is a concept unique to Rust
// the main aim of lifetimes is to prevent dangling references/ dangling pointers (i.e. pointers that point to nowhere/nothing)

// borrow checker is a key part of Rust's ownership system
// borrow checker compares scopes to determine whther all borrows are valid
// borrow checker tracks lifetimes of references and ensures that they don't violate the ownership rules
// ownership rules ensure that value is not accessed after the value has been moved or after the memory has been freed
// A reference must never outlive the value

fn main() {
    // outer scope
    let r; // declares r with no initial value
    // compiler assumes lifetime until the end of scope

    {
        //inner scope
        let x: i32 = 5; // declares x with initial value on stack
        // compiler assumes lifetime until the end of scope
        r = &x; //reference to x assigned to r
    } // x goes out of scope and is dropped

    // r is refering to smthn that no longer exists
    // println!("r: {}", r); // compiler would panic: "borrowed value does not live long enough"

    {
        let a: i32 = 5; // value
        let b = &a; // borrow of the value
        println!("b: {}", b); // this will work since both a and b live till the end of the same scope
    }
}
