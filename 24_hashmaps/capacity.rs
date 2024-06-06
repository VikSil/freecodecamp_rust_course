// HashMaps can grow and shrink same as Vectors
// HasMaps can be initated with a certain capacity using constructor HashMap::with_capacity(unit)

use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100); // reserve a minimum capacity of 100 at initilisation
    // up to 100 inserts can happen without having to move the HashMap to a new place in memory (which would be slow)
    map.insert(1, 2); // key and value can be of the same type
    map.insert(3, 4);

    assert!(map.capacity() >= 100); // however, it is possible that the actual allocated capacity is more than 100

    map.shrink_to(50); // shrinks the capacity with the min limit of 50

    map.shrink_to_fit(); // shrinks the capacity to the lowest possible
    assert!(map.capacity() >= 2);

    println!("Success!");
}
