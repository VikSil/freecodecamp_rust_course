// for types that implement Copy trait, like i32, their values are copied into the HashMap
// for owned types like String their values are moved into the HashMap, and HashMap will become the new owner of the values

use std::collections::HashMap;

fn main() {
    let v1: i32 = 10;
    let mut m1: HashMap<i32, i32> = HashMap::new();
    m1.insert(v1, v1); // i32 value is copied
    println!("v1 is still usable even after adding it to the hashmap : {}", v1);

    let v2: String = "hello".to_string();
    let mut m2: HashMap<String, i32> = HashMap::new();

    m2.insert(v2, v1); // ownership of the String value is moved

    // assert_eq!(v2,"hello"); // this would panic because v2 no longer owns its value

    let v3: String = "world".to_string();
    let mut m3: HashMap<&str, i32> = HashMap::new();

    m3.insert(&v3, v1); // use a pointer to/ string slice of the String does not take away ownership

    assert_eq!(v3, "world"); // this does not panic because String has ownership over its value

    println!("Success!");
}
