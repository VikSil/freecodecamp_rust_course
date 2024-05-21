// It should be called Overshadowing
// both match and if let can shadow a variable

fn main() {
    let age: Option<i32> = Some(30);

    if let Some(age) = age {
        // this creates a new variable of type Option with the same name 'age'
        // assert_eq!(age,Some(30)); // this would panic because Some(30) is refering to the overshadowed variable age
        assert_eq!(age, 30);
        println!("Success!");
    } // the new variable 'age' goes out of scope here

    match age {
        // matching on 'age' defined on line 2
        // creates a new shadowed variable with the same name 'age'
        Some(age) => println!("age is a new variable, its value is {}", age),
        _ => (),
    }
}
