// iterator adaptors are methods that can change one iterator into another
// it is possible to chain iterator adaptors to perform complex operations in a readable way

//since iterators are lazy, one of the chained adaptors has to be called in order to get any results

use std::collections::HashMap;

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1
        .iter()
        .map(|x| x + 1)
        .collect();

    assert_eq!(v2, vec![2, 3, 4]);

    println!("v1 is {:?}", v1);
    println!("v2 is {:?}", v2);
    println!("Success!");

    //------------------------------------------------

    let arr1: [String; 2] = ["Sunface".to_string(), "sunfei".to_string()];
    let arr2: [i32; 2] = [4, 5];

    let hm: HashMap<&String, &i32> = arr1.into_iter().zip(arr2.into_iter()).collect();

    println!("Hashmap: {:?}", hm);
}
