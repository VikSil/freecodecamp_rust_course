// static dispatch - compiler knows which methods will be called at runtime
// i.e. when trait bounds or generics are used, the compiler generates nongeneric implementations of functions and methods
// for each concrete type in place of a generic parameter
// this process is called monomorphization

// dynamic dispatch - compiler does not know which methods will be called during runtime, but it knows pointers to all possible methods
// dynamic dispatch occurs with trait objects
// each trait object contains pointers to all of its own methods that could be called during runtime
// pointer lookup incurs runtime cost
// with trait objects the compiler cannot optimize the code at runtime
// ut dynamic dispatch allows for more flexible code

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    // types from the standard library are types like any other and can hve traits implemented for them
    fn method(&self) -> String {
        format!("u8 {}", *self)
    }
}

impl Foo for String {
    // types from the standard library are types like any other and can hve traits implemented for them
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

fn static_dispatch<T: Foo>(a: T) {
    // fn works for any type that implements Foo trait
    a.method(); // and calls the method implmented in that trait type
}

fn dynamic_dispatch(a: &dyn Foo) {
    // works for any type that implements Foo trait, expects a pointer to that type
    a.method();
}

fn main() {
    let x: u8 = 5u8;
    let y: String = "Hello".to_string();

    static_dispatch(x); // at compile time this will become:

    // fn static_dispatch_u8<u8>(a:u8) {
    // format!("u8 {}", *self)
    // }

    dynamic_dispatch(&y);

    println!("Success!");
}
