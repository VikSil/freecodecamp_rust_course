// Rust compiler can provide basic implementations for some traits via the #[derive] attribute
// Most common derivable traits are:
// Debug - allows to output content via "{:?}"
// Clone - Enables type to be duplicated with "clone()" method
// Copy - Enables type to be copied implicitly, without requiring explicit "clone()" method
// PartialEq - Enables comparison

#[derive(PartialEq, PartialOrd)] // a tuple struct that can be compared and ordered
struct Centimeters(f64);

#[derive(Debug)] // a tuple struct that can be printed litteraly
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self; // dereference self and descturcture the i32 value of inches that the Inches struct holds, i.e. self = &Inches(12)
        Centimeters((inches as f64) * 2.54) // perform calculation and return it as instance of Cenimeters type
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Seconds(i32);

fn main() {
    let _one_second: Seconds = Seconds(1); // instantiating the Seconds struct

    println!("One second looks like {:?}", _one_second); // printing via {:?} requires Debug trait
    let _this_is_true = _one_second == _one_second; // comparison for equality requires PartialEq trait
    let _this_is_true = _one_second > _one_second; // comparison for order requires PartialOrd trait

    let foot: Inches = Inches(12);

    println!("One foot equals {:?}", foot); // will output "One foot equals Inches(12)"
    let meter: Centimeters = Centimeters(100.0);

    let cmp: &str = // cmp is a variable of string literal type
        if foot.to_centimeters() < meter {
            //Inches.to_centimeters returns Centimeters type and meter is also Centimeters type
            "smaller"
        } else {
            "larger"
        };

    println!("One foot is {} than one meter", cmp); // since cmp is string literal, it can be printed by simple {}
}
