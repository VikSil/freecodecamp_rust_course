// all iterators implement Iterator trait from std
// Iterator trait comes with next() method

pub trait Iterator {
    type Item;
    // next method takes a mutable refernce to self 
    // and returns an Option of the trait associated type
    fn next(&mut self) -> Option<Self::Item>;
}


fn main() {
    // all iterator variables have to be mutable
    let mut v1 = vec![1,2].into_iter();

    // next method can be explicitly called to pop the next value out of an iterator
    assert_eq!(v1.next(),Some(1)); // since Iterator returns an Option enum, the non-None values all must be wrapped in Some
    assert_eq!(v1.next(),Some(2));
    assert_eq!(v1.next(),None); // since Itertor returns an Option enum type, one of the valid variants is None

    println!("Success!");
}