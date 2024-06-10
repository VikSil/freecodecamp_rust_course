// all types can derive std::fmt::Debug implementation
// {:?} syntax is used for printing out a type that implements the Debug trait

// this struct cannot be printed either via fmt::Display or via fmt::Debug

struct UnPrintable(i32);

// to print a custom type, Debug trait must be derived
#[derive(Debug)]
struct DebugPrintable(i32);

//------------------------------------------------
use std::fmt; // use namespace so that don't have to repeat std in the code all the time

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure); // nested structure

struct OneNumber(i32);
struct SimpleOutput(OneNumber);

impl fmt::Debug for SimpleOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write will write to the buffer, into the variable f, which is a Formatter that can be passed into println
        write!(f, "{}", self.0.0) // .0.0 refers to addressing into a struct
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("There are {} months in a year", 12); // standard types can be printed via either Debug {:?} or Display {} notation
    println!("Derive the Debug trait in order to print the struct: {:?}", Structure(3));

    //------------------------------------------------

    let person: Person = Person { name: "Sunface".to_string(), age: 18 };

    println!("{:?}", person); // output in one line
    println!("{:#?}", person); // prettified output on multiple lines

    //------------------------------------------------

    // the problem with 'derive' is that there is no control over what
    // the result looks like.

    println!("It will print the entire {:?}, not just 7", Deep(Structure(7)));

    println!(
        "But this has custom impementation of Debug and will print {:?}",
        SimpleOutput(OneNumber(7))
    );
}
