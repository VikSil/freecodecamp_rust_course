// there are other methods besides .next() in the std Iterator
// consuming adaptors are methods that call the .next() method to use up the iterator

use std::collections::HashMap;

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter(); // iter does not consume the vector

    let total: i32 = v1_iter.sum(); // sum takes ownership and consumes the iterator
    let short_total: i32 = v1.iter().sum();

    assert_eq!(total, 6);
    assert_eq!(short_total, 6);

    // println!("{:?}, {:?}",v1, v1_iter); // this would panic because .sum() consumed the v1_iter

    println!("{:?}", v1);
    println!("{}, {}", total, short_total);
    println!("Success!");

    //------------------------------------------------

    let names:[(&str, i32);2]= [("Sunface", 18), ("sunfei",18)];
    // convert into an iterator, then collect back into a collection
    // need map to dereference the values for some reason (should work without but doesn't)
    let folks: HashMap<&str,i32> = names.into_iter().map(|e| *e).collect();

    println!("{:?}",folks);
    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<i32> = v1.into_iter().collect();
    // collect will consume the iterator
    // assert_eq!(v1,vec![1,2,3]); // this would panic
    assert_eq!(v2,vec![1,2,3]);


}
