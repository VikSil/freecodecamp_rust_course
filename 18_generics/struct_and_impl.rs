struct Point<T> {
    x: T, // both fields are of type T, which means both have to be of the same type
    y: T,
}

struct Point1<T, U> { // T and U are both generic types, but might be different ones (could be the same as well, but don't have to)
    x: T,
    y: U, // generic types following T are U, V, W, X, Y, Z by convention
}

struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn main() {
    // if Point is provided in type annotation, then the i32 has to be provided, i.e. just Point as annotation will not work
    let integer: Point<i32> = Point { x: 5, y: 10 };
    let float: Point<f64> = Point { x: 1.0, y: 4.0 };

    println!("Success!");

    //------------------------------------------------

    let p: Point1<i32, String> = Point1 { x: 5, y: "hello".to_string() };

    println!("Success!");

    //------------------------------------------------

    let x: Val<f64> = Val { val: 3.0 };
    let y: Val<String> = Val { val: "hello".to_string() };
    println!("{} {}", x.value(), y.value());

}
