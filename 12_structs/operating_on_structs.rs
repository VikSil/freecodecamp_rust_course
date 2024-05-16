struct Person {
    name: String,
    age: u8,
}

fn main() {
    let age: u8 = 18;
    let mut p: Person = Person { // the instance has to be mutable to change the field values
        name: String::from("Sunface"),
        age, // separate fields cannot be mutable
    };

    p.age = 30;
    p.name = String::from("Sunfeis");

    println!("Success!");

    //------------------------------------------------

    let chris: Person = create_person(String::from("Chris"), 15);

    println!("Success!");

    //------------------------------------------------

    let clone: Person = clone_person(chris);

    println!("Success!");
}

fn create_person(name: String, age: u8) -> Person {
    Person {
        age,
        name,
    }
}

fn clone_person(p: Person) -> Person {
    Person {
        name: String::from("Clone"), // field value here is assigned via colon not equal sign
        ..p
    }
}
