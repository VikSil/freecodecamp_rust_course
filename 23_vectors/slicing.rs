// A vecotr can be mutable, but slices must be read-only
// a slice of a vector is declared by &, e.g. String and &str
// it is more common to pass slices as arguments than vectors if only read access is required

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    let slice1: &[i32] = &v[..]; // slice of the entire length of the vector, whatever the length may be

    // let slice2 = &[0..4]; // this would panic, since index 4 is out of bounds
    let slice2: &[i32] = &v[..v.len()]; // slice until the length of the vector (not included, hence don't have to subtract one)

    assert_eq!(slice1, slice2);

    let vec_ref: &mut Vec<i32> = &mut v; // slice of an existing object and &Vec reference type are different
    (*vec_ref).push(4);
    // let slice3: &[i32] = &mut v[0..3]; // this owuld not work because &mut is not possible since slices are immutable
    // slice3.push(4); // this would panic because it is not possible to mutate a vector via its slice

    let slice3: &[i32] = &v[0..]; // this will include the last pushed element of 4 as well

    assert_eq!(slice3, &[1, 2, 3, 4]);
    println!("Success!");
}
