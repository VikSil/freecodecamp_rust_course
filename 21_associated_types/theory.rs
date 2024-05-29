// types that are associated with traits
// type placeholder that the trait methods can use in their signature
// similar to generic types, bu tmore flexible because they allow a trait to have different associated types for different implementing types
// when implementing the trait for a specific type, a concrete type must be specified

trait MyTrait {
    type MyType; // this is associated type

    fn get_my_type(&self) -> Self::MyType; // associated type can then be used in methods defined in the trait
}

struct MyStruct {}

impl MyTrait for MyStruct {
    type MyType = i32; // in type implementation the associated type becomes a concrete type

    fn get_my_type(&self) -> Self::MyType {
        // and methods with that type have to use the concrete type
        return 42;
    }
}

fn main() {}
