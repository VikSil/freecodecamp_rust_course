trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("The duck is flying");
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("The swan is flying");
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck, duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn main() {
    let birds: [Box<dyn Bird>; 2] = [Box::new(Swan), Box::new(Duck)]; // an array of Bird trait bojects
    let birds: [&dyn Bird; 2] = [&Swan, &Duck]; // for this example works the same as Box
    // in general terms Box also owns the variable, reference does not

    for bird in birds {
        bird.quack();

        // bird.fly(); // and
        // bird.swim(); // would both panic for all array elements
        // because all of them are Bird trait objects and neither are Duck or Swan
    }
}
