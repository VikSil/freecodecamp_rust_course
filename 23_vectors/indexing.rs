fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 { // iterated from zero to four
        //println!("{:?}", v[i]); // this would panic at index 3 because there are only three elements in the vector
        println!("{:?}", v.get(i)); // if element exists .get() returns an Option<i32> of type usize
    }

    // for existing elements add +1, when no more elements to increment, add new elements
    for i in 0..5 {
        match v.get(i) {            
            Some(e) => v[i] = e + 1, // if Some element exists
            None => v.push(v[i - 1] + 1), // v.push(i+2) would work the same
        }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}
