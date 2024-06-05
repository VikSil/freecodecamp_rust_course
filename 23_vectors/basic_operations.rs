// vector is like an array, but it is dynamically sized - it can grow and shrink
// vector size does not need to be known at compile time
// vectors are heap allocated
// all elements in a vector must have the same type
// vectors use vec! macro

// String is a vector of type u8, i.e. String = Vec<u8>

fn main() {
    let arr: [u8; 3] = [1, 2, 3];

    let v: Vec<u8> = Vec::from(arr);
    is_vec(v);

    let v: Vec<u8> = vec![1, 2, 3]; // this is vector of type V<i32>
    is_vec(v);

    let v: Vec<u8> = vec![1, 2, 3]; // vec!(..) and vec![..] is the same macro
    is_vec(v.clone()); // give a clone to this function because it will assume ownership, but v will be needed for assert_eq!

    let v1: Vec<[u8; 3]> = vec![arr]; // this is vector of type Vec<[u8;3]>, i.e. vector of arrays with one element in it
    // is_vec(v1); // this would panic because is_vec expects a different type of vector

    let mut v2: Vec<u8> = Vec::new();
    is_vec(v2.clone()); // give a clone to this function because it will assume ownership, but v2 is used on the next line
    for i in &v {
        v2.push(*i); // have to dereference because v is iterrated over by reference
    }

    assert_eq!(v, v2);

    println!("Success!");

    //------------------------------------------------

    let mut vec1: Vec<i32> = Vec::from([1, 2, 4]);
    vec1.pop(); // acts like a FILO stack
    vec1.push(3);

    let mut vec2: Vec<i32> = Vec::new();
    vec2.extend(&vec1); // add elements at the tail of the vector

    assert_eq!(vec1, vec2);

    println!("Success!");

    //------------------------------------------------
}

fn is_vec(v: Vec<u8>) {
    // does not do anything other than takes ownership of the input
}
