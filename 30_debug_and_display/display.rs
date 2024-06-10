// Display is used to customise the output of a type
// Display cannot be derived, it has to be implmented explicitly
// placeholder for Display is {} not {:?}

use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y) // write the stuff to buffer f that will be output by println
    }
}

// manual implementation allows for different outputs of Display vs Debug

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: Complex {{real: {}, imag: {}}}", self.x, self.y) // curly braces are escaped by double curly braces
    }
}

fn main() {
    let point: Point2D = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}", point), "Debug: Complex {real: 3.3, imag: 7.2}");

    println!("{}", point); // Display notation
    println!("{:?}", point); // Debug notation
    println!("Success!");
}
