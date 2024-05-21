// matches! macro allows to test a variable structure against a pattern
// matches!(variable, pattern)

enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let alphabet = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for letter in alphabet {
        assert!(matches!(letter, 'A'..='Z' | 'a'..='z' | '0'..='9')); // matches a character A to Z OR a to z OR 1 to 9 (also as chars not ints)
    }

    println!("Success!");

    //------------------------------------------------

    let mut count = 0;
    let v: Vec<MyEnum> = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo]; // a vector is an array that has a dynamic size
    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }

    assert_eq!(count, 2);
    println!("Success!");
}
