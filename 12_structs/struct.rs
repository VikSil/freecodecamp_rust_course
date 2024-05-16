struct Person {
    name: String,
    age: u8,
    hobby: String,
} // no semicolon after the strc definition!!

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let age: u8 = 30;
    let p = Person {
        name: String::from("Sunface"),
        age, // short-hand syntax
        hobby: String::from("None"), // all fields in a struct must be initialised
    };

    println!("Success!");

    //------------------------------------------------

    let v: Point = Point(1, 2, 3);
    let c: Color = Color(0, 127, 255);
    // check_color(v); // this would panic because function expects a Color struct as an input
    check_color(c);

    println!("Success!");
}

fn check_color(p: Color) {
    let Color(x, _, z) = p; // destructuring, struct name is needed, if a field from a tuple struct is not needed, it can be skiped by underscore
    assert_eq!(x, 0);
    assert_eq!(p.1, 127); // accessing a field of a Tuple struct by index
    assert_eq!(z, 255);
}
