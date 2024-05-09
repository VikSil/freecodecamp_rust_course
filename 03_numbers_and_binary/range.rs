use std::ops::{ Range, RangeInclusive };

fn main() {
    let mut sum: i32 = 0;
    // iterate over -3 , -2, -1, 0, 1, since rust ranges are non-inclusive by default
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);

    //------------------------------------------------

    // this range will be inclusive due to the = sign before 'z'
    for c in 'a'..='z' {
        // this will output chars a to z
        println!("{}", c);
        // this wiil output ascii codes 97 to 122 for the chars, since c is requested as u8 - unsigned integer
        println!("{}", c as u8);
    }

    //------------------------------------------------

    assert_eq!(1..5, Range { start: 1, end: 5 }); // 5 will be excluded by both syntaxes
    assert_eq!(1..=5, RangeInclusive::new(1, 5)); // 5 will be included

    //------------------------------------------------

    println!("Success!");
}
