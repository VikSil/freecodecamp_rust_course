// types are converted to String via ToString trait for that type
// this functionality is implemented by fmt::Display trait which automatically provides ToString and also allows to print the type via println! macro

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // definition from the std docs
        // write  works like println! or format!, but it writes to the buffer
        write!(f, "The point is ({},{})", self.x, self.y)
    }
}

fn main() {
    let origin: Point = Point { x: 0, y: 0 };

    assert_eq!(origin.to_string(), "The point is (0,0)");
    assert_eq!(format!("{}", origin), "The point is (0,0)"); // format! macro autoconverts a type to a string to print it out

    println!("{}", origin);

    println!("Success!");
}
