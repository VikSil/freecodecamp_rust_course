// Method is a funciton that is associated with a particular type or struct
// Method takes parameters and returns a value same as a function would,
// but method is a member of a struct or enum
// Methods are called using dot notation, same as accessing other members of a struct
// Methods ar eimplmented via an 'impl' block

// Associated function is function that is associated with a struct or enum, but doe snot take an istnace of that struct or enum as its first parameter,
// i.e. associated function does not use 'self', i.e. no instance is needed to call it, associated functions are called from the type instead
// Associated functions are called using TypeName::Function notation
//  Associated functions are often used as constructors for a struct or enum

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // impl block for type Rectangle
    fn area(&self) -> u32 {
        // method definition
        self.width * self.height // return value
    }

    fn new(width: u32, height: u32) -> Rectangle {
        // Associated function which is also a constructor
        Rectangle {
            width,
            height,
        }
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Associated function
    fn origin() -> Point {
        // without any arguments
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        // Associated function taking two arguments
        Point { x: x, y: y }
    }
}

fn main() {
    let rec1 = Rectangle { // instantiation wihtout associated function constructor
        width: 30,
        height: 50,
    };

    let rec2: Rectangle = Rectangle::new(5, 10);

    println!("The area fo the rectangle is {} square pixels", rec1.area());
    println!("Rectangle {:?}", rec2);
}
