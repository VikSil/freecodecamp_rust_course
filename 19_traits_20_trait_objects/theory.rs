// traits are a set of methods that can be implemented for multiple types in order to provide common functionality/ behaviour
// traits consist only of method signatures, which then have to be implemented by the target type
// traits are similar to classes in other languages

trait Animal {
    // this is trait
    fn sound(&self) -> String;
}

struct Sheep; // these are custom types
struct Cow;
struct Butterfly;

impl Animal for Sheep {
    // the trait can be implemented for any class
    fn sound(&self) -> String {
        // the signature must mach that in the trait definition
        String::from("Baah")
    }
}

impl Animal for Cow {
    fn sound(&self) -> String {
        String::from("Mooh")
    }
}

// Butterfly does not have a sound, hence the trait is not implemented for this class

//------------------------------------------------------------------------------

// DERIVABLE TRAITS

// Derivable trait is a trait that can be automatically implemented for a struc or enum by the Rust compiler
// Most common derivable traits are:
// Debug - allows to output content via "{:?}"
// Clone - Enables type to be duplicated with "clone()" method
// Copy - Enables type to be copied implicitly, without requiring explicit "clone()" method
// PartialEq - Enables comparison

#[derive(PertialEq, PartialOrd)] // a tuple struct that can be compared
struct Centimeters(f64);

#[derive(Debug)] // a tuple struct that can be printed
struct Inches(i32);

//------------------------------------------------------------------------------

// TRAITS AS PARAMETERS

// Traits can be passed as parameters to functions
trait Summary {}
// the function notify takes as an argument any type that has implemented the Summary trait
pub fn notify(item: &impl Summary) {
    // i.e. this type annotation ensures only variables that do have summarize mehtod will be accepted as arguments
    println!("Breaking news! {}", item.summarize()); // Summary trait defines summarize method signature that is implemented in the type and called here
}

//------------------------------------------------------------------------------

// TRAIT BOUNDS

// Trait bouds are declared like generics and used if there are a lot of paramters to be passed in
// syntax is <T:Traitname>

// the function had generic type T that implements Summary trait
// meaning that variable item that is passed into it must be of the same type T and implement Summary trait
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Using Trait bounds are similar to "Impl Summary", but less verbose.

pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// becomes
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// where clauses can be used to make trait bound syntax more readable

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// becomes
fn some_function<T, U>(t: &T, u: &U) -> i32 where T: Display + Clone, U: Clone + Debug {}

//------------------------------------------------------------------------------

// RETURN TYPES WITH TRAITS

// functions can return types that implement a specific trait

trait HousePet {} // trait

struct Dog;
struct Cat;

impl HousePet for Dog {} // trait implementation
impl HousePet for Cat {}

fn return_dog() -> impl HousePet {
    // fn returns an instance of a struc that implements HousePet trait
    Dog {}
}

fn return_cat() -> impl HousePet {
    // return type that implements HousePet trait
    Cat {}
}

// TRAIT OBJECTS

// using impl Trait does not work as return trait type when one of multiple types could be returned
// Different implementations of the same trait probably use different amounts of memory,
// but return type size must be known at compile time
// Trait object is a pointer to a type that implements a trait,
// i.e. the size of a pointer is known, it is always usize (8 bytes), the size of the type implementation does not need to be known

fn return_pet(s: &str) -> &dyn HousePet {
    match s {
        "dog" => &(Dog {}),
        "cat" => &(Cat {}),
        _ => panic!(),
    }
}

fn main() {
    let animal1: Cat = return_cat();
    let animal2: Dog = return_dog();
    let animal3: Cat = return_pet("cat");
    let animal4: Cat = return_pet("dog");
}

//------------------------------------------------------------------------------

// STATIC DISPATCH

// resolves method calls at compile time
// compiler generates function code for each concrete type that implements a trait
// function calls appropriate functions based on the concrete types
// this approach is faster/ more efficient thatn dynmic dispatch, but does not allow for flexibility

trait Friend {
    fn say_hi(&self);
}

impl Friend for Dog {
    fn say_hi(&self) {
        println!("Woof");
    }
}

impl Friend for Cat {
    fn say_hi(&self) {
        println!("Meow");
    }
}
// at compilation becomes (approximatelly, compiler also applies optimisation)

impl Dog {
    fn say_hi(&self) {
        println!("Woof");
    }
}

impl Cat {
    fn say_hi(&self) {
        println!("Meow");
    }
}

//------------------------------------------------------------------------------

// DYNAMIC DISPATCH

// resolves method calls at runtime
// implemented via reference or smart pointer to a trait object
// syntax: &dyn TypeName or Box<dyn TypeName>
// when trait object is created, compiler will build a vtable for that trait
// vtable is a table that contains a pointers to the implementations of each method in a trait for each type that has that trait implemented
// at runtime the vtable is used to determine which method should be used when called on a concrete object
// the lookup causes overhead (execution will be slower), but allows for more flexible code

trait Speak {
    fn noise(&self);
}

impl Speak for Cat {
    fn noise(&self) {
        println!("Meow");
    }
}

impl Speak for Dog {
    fn noise(&self) {
        println!("Woof");
    }
}

fn random_animal(random_number: u8) -> Box<dyn Speak> {// Box puts variable in heap and returns a pointer
    if random_number < 10 {
        Box::new(Cat {}) // this will be a line in the vtable and a variable in heap
    } else {
        Box::new(Dog {}) // this will be another line in the vtable  and a variable in heap
    }
}

fn main() {
    let random_number = 5;
    let animal = random_animal(random_number);
    animal.noise();
}


//------------------------------------------------------------------------------

// & vs Box

// Memory:
// & only points to a value that already is in heap, does not own anything
// Box allocates the value to the heap and also owns it
// i.e. Box is responsible for deallocating when the value goes out of scope

// Lifetime:
// & has limited lifetime
// Box can be passed accross scopes

// Box can be cloned, & cannot be cloned

// Box can be used in pattern matching, & cannot
