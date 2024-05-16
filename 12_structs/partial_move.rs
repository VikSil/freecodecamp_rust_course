// both by-move and by-reference destructuring is allowed simulatneously
// this will result in a partial move of the variable
// going forward the variable cannot be used as a whole, only the remaining parts can be used

fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let alice = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person { name, ref age } = alice; // destructuring into variables, name assumes ownership, age is a reference

    println!("Her age is {}", age);
    println!("Her name is {}", name);

    // println!("Alice looks like this: {:?} ", alice); // this would panic because alice variable no longer owns its name field value - it was destructured into name variable
    println!("The age from the struct is {}", alice.age); // this works, because alice variable still owns age field value

    //------------------------------------------------

    #[derive(Debug)]
    struct File {
        name: String, // the same field name can be used accross structs
        data: String,
    }

    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practise".to_string(),
    };

    let filename: String = f.name.clone(); // filename get a replica of the f.name and f still owns its name

    println!("{}, {}, {:?}", filename, f.data, f);
}
