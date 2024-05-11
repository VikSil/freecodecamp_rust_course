fn main() {
    #[derive(Debug)]
    struct Person { // struct is a custom composite type
        name: String, // this  is on stack
        age: Box<u8>, // this is heap
    }

    let person: Person = Person { // instantiate the struct
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person { name, ref age } = person; // this is destructuring values of the Person struct into two variables that must be called the same, but can now be used outside of the struct
    // name will be moved out of the person, but age will be referenced

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // println!("The person struct is {:?}", person); // this would panic because person is partially moved
    // println!("The person's name is {}", personn.name); // this would panic because the name was moved out of the person, i.e. person no longer owns it
    println!("The person's age is {}", person.age); // this works because age was destructured by reference and was not moved out of the person
}
