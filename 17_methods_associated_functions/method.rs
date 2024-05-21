struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(self) -> u32 {
        // the first paramter of a method is always self either by ownership or by borrowing
        self.width * self.height
    }
}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self) {
        // reference to self by borrowing (i.e. pointer), it is the same as self: &Self
        println!("The current state is {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }
}

fn main() {
    let rec1 = Rectangle { width: 30, height: 50 };
    assert_eq!(rec1.area(), 1500);
    println!("Success!");

    //------------------------------------------------

    let light = TrafficLight { // instantiaiton of the struct
        color: "red".to_owned(), // the String is owned by the struct
    };

    light.show_state(); // execution of the method only borrows the field values
    println!("Success!");
}
