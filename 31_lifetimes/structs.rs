#[derive(Debug)]
struct Borrowed<'a>(&'a i32); // 'a lifetime annotation enforces that the i32 that is referenced must outlive the struct

// Both references must outlive the struct
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// an enum that is either i32 or a reference to i32
#[derive(Debug)]
enum Either<'a> {
    Num(i32), // this does not need lifetime because it is not a reference
    Ref(&'a i32),
}

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'c, 'd> { //lifetimes 'c and 'd could end up functionally being the same length
    c: &'c i32,
    d: &'d NoCopyType,
}

// reference to the Example and NoCopyType must both outlive fix_me function
// both can have the same lifetime annotation, since both live in the same scope
fn fix_me<'e>(foo: &'e Example) -> &'e NoCopyType {
    foo.d
}

fn main() {
    let x: i32 = 18;
    let y: i32 = 15;

    let single: Borrowed = Borrowed(&x); // x must outlive Borrowed
    let double: NamedBorrowed = NamedBorrowed { x: &x, y: &y }; // x and y must outlive NamedBorrowed and have the same lifetime
    let reference: Either = Either::Ref(&x);
    let number: Either = Either::Num(y); // i32 will be copied because it is a primitive, owenrship is not changed

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is not borrowed in {:?}", number);

    //------------------------------------------------

    // lifetime 'c starts here
    let var_c: i32 = 35;
    let example: Example; // declared but not instantiated

    {
        // lifetime 'd starts here
        let var_d: NoCopyType = NoCopyType {}; // instance of the struct

        example = Example { c: &var_c, d: &var_d };
        println!("Success! {:?}", example);
    } // var_d will cease to exist here and example field will end up pointing nowhere

    // println!("Success! {:?}",example); // this would panic at compile time because example would be partially invalid

    //------------------------------------------------

    // no_copy will outlive the fix_me function, as the lifetime in function annotation expects
    let no_copy: NoCopyType = NoCopyType {};
    // example2 will outlive the fix_me function, as the lifetime in function expects
    let example2: Example = Example { c: &1, d: &no_copy };
    fix_me(&example2);
    println!("Success!");
} // x and y live until the end of scope
