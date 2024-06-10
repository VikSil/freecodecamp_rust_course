// each write! macro generates an fmt::Result which makes it difficult to implement fmt::Display for structures whose elements must be handled separately

// ? operator replaces matching code and allows to deal with fmt::Result

use std::fmt;

struct List(Vec<i32>); // list of integer vectors

impl fmt::Display for List {
    // function signature from documentation
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec: &Vec<i32> = &self.0; // extract pointer to first vector by tuple indexing

        write!(f, "[")?; // open the brackets

        // iterate over v in vec while enumerating the iteration count
        for (count, v) in vec.iter().enumerate() {
            // for every element except the first add a comma
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", count, v)?; // ? is used everywhere to return on errors
        }
        // close the brackets
        write!(f, "]") // no questionmark because the return type is the whole Result, not either of its variants
    }
}

fn main() {
    let v: List = List(vec![1, 2, 3]); // instantiating a list with one vector of three ints
    assert_eq!(format!("{}", v), "[0:1, 1:2, 2:3]");
    println!("{}", v); // {} since the Display trait is implemented
    println!("Success!");
}
