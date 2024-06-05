// The capacity of a vector is the amount of space allocated for the vector
// The length of a vector is its current number of elements in it
// if adding new elements exceeds the capacity, the capacity will be doubled and vector will be moved to another place in memory that can accomodate the new capacity of the vector
// reallocating the vector is slow
// it is best to use Vec::with_capacity() constructor if there is some expectation of how long the vector will become at runtime

fn main() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0); // the vector contians no items, even though it has capacity for 10
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        // since vec already has 10 spaces allocated, this loop does not cause reallocation
        vec.push(i);
    }

    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    vec.push(11); // this will cause the vector to double the capacity and reallocate
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!");
}
