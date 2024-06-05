// String consists of three parts:
// pointer to some bytes in the Heap
// current length
// maximum capacity

// the size of String will be 24 bytes on the stack, i.e. 8 bytes for each of the three parts

// String has to be a continous block in the Heap.
// If a String needs to grow, but the memory ahead is reserved by something else,
// then the entire String will be realocated to where there is enough continous space
// by default capacity will always double: 1,2,4,8,16,32,64,128,256 etc.

fn main() {
    let mut s: String = String::new();

    println!("{}", s.capacity()); // new string will have no capacity

    for _ in 0..2 {
        s.push_str("hello"); // with each push the size will grow, and hence so will the capacity
        println!("{}", s.capacity());
    }

    println!("Success!");

    let mut s1: String = String::with_capacity(25);

    println!("{}", s1.capacity()); // new string will have designated capacity

    for _ in 0..2 {
        s1.push_str("hello"); // with each push size changes, but initialised capacity is enough, it does not need to change
        println!("{}", s1.capacity());
    }

    println!("Success!");
}
