#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self {
            value, // shorthand notation for value: value
        }
    }
}

fn main() {
    let num: Number = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = (30).into();
    assert_eq!(num.value, 30);

    println!("Success!");
}
