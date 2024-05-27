struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        } // a new instance does not need to be assigned to a variable, just create it and it will be returned
    }
}

struct Point1<T> {
    x: T,
    y: T,
}

impl Point1<f64> {
    // implementation will only work for Point1 struct that holds f64 fields
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt() // powi (power integer) must be called on f64, takes in an i32 and returns f64
        // hence the struct Point1 field and method return types must be f64
    }
}

fn main() {
    let p1: Point<i32, i32> = Point { x: 5, y: 10 };
    let p2: Point<&str, char> = Point { x: "Hello", y: 'W' };
    let p3: Point<i32, char> = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, 'W');

    println!("Success!");

    //------------------------------------------------

    let p: Point1<f64> = Point1 { x: 5.0_f64, y: 10.0_f64 }; // fields need to be of type f64 for the method to work
    println!("{}", p.distance_from_origin());
}
