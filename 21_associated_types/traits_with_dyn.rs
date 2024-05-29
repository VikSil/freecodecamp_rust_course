trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("The duck is swimming"); // this will not exist in the Vtable of the boxed trait object
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        // this is a method not a function because it takes a reference to self
        println!("The swan is flying"); // this will not exist in the Vtable of the boxed trait object
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "quack quack".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swans don't quack".to_string()
    }
}

fn main() {
    let duck: Duck = Duck;
    duck.swim(); // this is type object, hence it can access type methods

    let bird: Box<dyn Bird> = hatch_a_bird(2); // this is trait object, not the type object, hence

    // bird.fly(); // fly is not available because it is defined on the Swan type not the trait type

    assert_eq!(bird.quack(), "swans don't quack");

    let bird: Box<dyn Bird> = hatch_a_bird(1); // this is trait object, not the type object, hence

    // bird.swim(); // fly is not available because it is defined on the Duck type not the trait type

    assert_eq!(bird.quack(), "quack quack");

    println!("Success!");
}

// since Rust is compiled the order of functions does not matter
fn hatch_a_bird(species: i32) -> Box<dyn Bird> {
    match species {
        1 => Box::new(Duck),
        2 => Box::new(Swan),
        _ => panic!(),
    }
}
