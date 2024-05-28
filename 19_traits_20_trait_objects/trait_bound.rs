// generic types use traits as bounds to stipulate what functionality a type must implement

// here T: std::ops::Add<ouput = T>> is the trait bound, i.e. type T must implement trait std::ops::Add
fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // this is an associated function not a method because it does not have &self as a parameter
    fn new(x: T, y: T) -> Self {
        // upercase Self refers to the type of the implementation block
        Self { x, y }
    }
}

impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            // fields will need PartialOrd and PartialEq derive
            println!("The larger member is x = {:?}", self.x); // fields will need Debug derive
        } else {
            println!("The larger member is y = {:?}", self.y);
        }
    }
}

#[derive(Debug)] // needed since this struc will be used as type for Pair implementation
#[derive(PartialOrd)]
#[derive(PartialEq)]
struct Unit(i32);

fn main() {
    assert_eq!(sum(1, 2), 3);
    println!("{}", sum(5, 5));
    println!("{}", sum(5.2, 5.7));

    //------------------------------------------------

    let pair: Pair<Unit> = Pair {
        x: Unit(1),
        y: Unit(3),
    };
    // works the same as using the new function on the Pair type
    let pair: Pair<Unit> = Pair::new(Unit(1), Unit(3));

    pair.cmp_display(); // requires implementatin of derive traits for the struct fields
}
