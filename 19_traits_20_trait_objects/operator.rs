// Rust allows overloading of operators with traits
// Some operators can be used to accomplish different tasks based on the input argument types
// This is possible because operators are syntactic sugar for method calls
// e.g. + operator in a + b is a method call a.add(b), where add() method is part of Add trait
// hence any type tha timplements Add trait can use + operator to it's liking

use std::ops; // import ops module from standard library
// sice that is where std::ops::Mul trait lives that is associated with * operator

// function with a trait bound std::ops::Mul<Output = T>
fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    // i.e. inputs of generic type T must implement standard Mul trait
    a * b
}

struct Foo;
struct Bar;

#[derive(PartialEq, Debug)] // PartialEQ is necessary for assert_eq! macro
struct FooBar;
#[derive(PartialEq, Debug)]
struct BarFoo;

// the inbuilt descriptor is ops::Add<Rhs = Self> , where Rhs = righ hand side, if nothing is provided, then it is self
impl ops::Add<Bar> for Foo {
    // Rhs in this overload is Bar, trait implementation for Foo
    type Output = FooBar; // Output is an associated function

    fn add(self, _rhs: Bar) -> FooBar {
        // since Foo + Bar is converted by compiler to Foo.add(Bar), it is the add method that has to be overriden
        FooBar
    }
}

impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

fn main() {
    // function of generic type T can take different variable types as inputs
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");

    //------------------------------------------------

    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success!");
}
