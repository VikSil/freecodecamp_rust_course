// impl Trait syntax can be used in the return position to return a value of type that implements that trait
// imp Trit can only be used to return a single type
// Trait Objects should be used to return several trait types

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "mooo!".to_string()
    }
}

// function will return some struct that implemets Animal, and we don't/ don't need to know which one in advance

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 { Box::new(Sheep {}) } else { Box::new(Cow {}) }
}

fn main() {
    let random_number: f64 = 0.234;
    let animal: Box<dyn Animal> = random_animal(random_number);
    println!("The random animal says {}", animal.noise());
}
