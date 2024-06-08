fn main() {
    // as_bytes converts chars to utf ascii numbers/bytes
    // assert_eq!("abc".as_bytes(),[96,97,98]); // this would panic because the numbers are wrong
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v: Vec<i32> = vec![1, 2, 3];
    // let ele = v[3]; // this would panic because there are only 3 elements in the vector
    let ele: i32 = v[2];

    // .get returns either a Some(&value) or None
    // let ele = v.get(3).unwrap(); // unwrap would panic over geting a None
    let ele: &i32 = v.get(1).unwrap(); // Some(2) --> 2

    // Sometimes the compiler is unable to find the overflow error at compile time (see return type of the function below)
    let v = production_rate_per_hour(2);

    divide(15, 1); // division by zero when wrapped in a function would not panic at compile time, but at run time

    println!("Success!");
}

fn divide(x: u8, y: u8) {
    println!("{}", x / y); // dividion with zero would panic at runtime
}

fn production_rate_per_hour(speed: u16) -> f64 {
    let cph: u16 = 221; // has to match the input tupe
    match speed {
        1..=4 => (speed * cph) as f64, // if function used u8 arythmetics then for speed > 2 this would overflow at runtime because u8::MAX = 255
        5..=8 => ((speed * cph) as f64) * 0.9,
        9..=10 => ((speed * cph) as f64) * 0.77,
        _ => 0 as f64,
    }
}
