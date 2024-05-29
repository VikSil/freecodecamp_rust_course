trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    // u8 is a type like any other and traits can be implemented for it
    fn draw(&self) -> String {
        format!("u8: {}", self) // dereferencing is happening automatically , but *self dereferencing would also work
    }
}

impl Draw for f64 {
    // f64 is a type like any other and traits can be implemented for it
    fn draw(&self) -> String {
        format!("f64: {}", *self) // explicit dereferencing
    }
}

fn main() {
    let x: f64 = 1.1f64;
    let y: u8 = 8u8;

    draw_with_box(Box::new(x)); // variables can also be Boxed, not just types

    draw_with_ref(&y);

    println!("Success!");
}

fn draw_with_box(x: Box<dyn Draw>) {
    // function expects a boxed trait object of Draw trait type
    x.draw(); // calls draw method on the trait object that is referenced in the vtable
}

fn draw_with_ref(x: &dyn Draw) {
    // function expects a reference to a trait oobject that implements Draw trait
    x.draw();
}
