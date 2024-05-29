// A trait is object safe if all the methods in the trait have the following properties:
// * the return type is not Self
// * there are no generic type parameters

// object-safe traits always use dynamic dispatch, and are always trait objects

// STATIC DISPATCH implementation
trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self {
        42
    } // Self here is u32 because Self is the same as the type for which a trait is implemented
}

impl MyTrait for String {
    fn f(&self) -> Self {
        self.clone()
    } // Self here is String because Self is the same as the type for which a trait is implemented
}

fn my_function<T: MyTrait>(x: T) -> T {
    // fn has a trait bound of MyTrait - it only accepts arguments that implement MyTrait
    x.f()
}

// DYNAMIC DISPTCH implementation
trait MyTrait1 {
    fn f(&self) -> Box<dyn MyTrait1>; // cannot use Self for object-safe traits
}

impl MyTrait1 for u32 {
    fn f(&self) -> Box<dyn MyTrait1> {
        // cannot use Self for object-safe traits, and cannot use u32 because signature must match the trait signature
        Box::new(42)
    } // Self here is u32 because Self is the same as the type for which a trait is implemented
}

impl MyTrait1 for String {
    fn f(&self) -> Box<dyn MyTrait1> {
        // cannot use Self for object-safe traits, and cannot use String because signature must match the trait signature
        Box::new(self.clone())
    } // Self here is String because Self is the same as the type for which a trait is implemented
}

fn my_function1(x: Box<dyn MyTrait1>) -> Box<dyn MyTrait1> {
    // object-safe traits cannot use generic parameters
    // fn has a trait bound of MyTrait - it only accepts arguments that implement MyTrait
    x.f()
}

fn main() {
    // Static dispatch implementation
    my_function(13_u32);
    my_function(String::from("abc"));

    // Dynamic dispatch implementation
    my_function1(Box::new(13_u32));
    my_function1(Box::new(String::from("abc")));

    println!("Success!");

    //------------------------------------------------
}
