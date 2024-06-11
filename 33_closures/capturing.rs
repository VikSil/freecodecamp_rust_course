// closures can capture variables by borrowing or by moving the values
// the order of capture preference:
// by reference - &T
// by mutable reference - &mut T
// by value - T

fn main() {
    let color: String = String::from("green");

    // closure does not take any arguments, i.e. ||
    let print = || println!("color: {}", color); // color is a not-owned immutable reference

    print();
    print();

    // color value can be borrowed immutably again, because the closure only holds an immutable reference to it

    let _reborrow: &String = &color;
    println!("{}", color);

    //------------------------------------------------

    let mut count: i32 = 0;

    // without move closure would capture the variable as a mutable reference (least restrictive approach)
    let mut inc = move || {
        // with move closure takes ownership of the value (a copy for primitives)
        count += 1; // closure captures the variable
        println!("count: {}", count);
    };

    inc();

    // this is only possible because closure uses move and takes ownership
    // otherwise it would take mutable reference
    // and mixing mutable and immutable references in the same scope is forbidden
    let _reborrowed: &i32 = &count;

    inc(); //scope of closure ends here

    // since neither inc nor _reborrowed will not be borrowing count beyond this point, it is possible for someone else to borrow it again without panic
    let _count_reborrowed = &mut count;

    assert_eq!(count, 0);

    //------------------------------------------------

    let movable: Box<i32> = Box::new(3); // heap allocated (boxed) i32

    let consume = || {
        println!("movable: {}", movable); // closure captures variable by immutable reference
        take_ownership(movable);
    };

    consume();
    // consume(); this would panic because take took ownership of movable and went out of scope with it, so there is nothing inside movable for consume to reference on the second call

    let referrable: Box<i32> = Box::new(5); // heap allocated (boxed) i32

    let eat = || {
        println!("movable: {}", referrable); // captures variable by immutable reference
        take_reference(&referrable); // technically passes a reference to a reference
    };

    eat();
    eat(); // can be called multiple times because referrable does not cease to exist, since take_referance only took reference
}

// f-tion takes argument of any type and takes ownership of it
// for some reason the passed in value can be a referrence, the caller does not need to own it
fn take_ownership<T>(_v: T) {}

fn take_reference<T>(_v: &T) {}
