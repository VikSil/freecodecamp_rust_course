enum Foo { // Foo enum
    Bar(u8), // with Bar variant holding a type of u8
}

enum FooBar {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        None => (), // match needs every possible variant to be present
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } // if let works the same as match, but does not require every variant to be evaluated

    //------------------------------------------------

    let o: Option<i32> = Some(7);

    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            println!("Success!");
        }

        _ => {}
    }

    // does the same thing as the match statement above, but in fewer lines
    if let Some(i) = o {
        // if it is possible to destructure o into Some Option i, then print
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }

    //------------------------------------------------

    let a: Foo = Foo::Bar(1); // instance of Bar variant holding a value of 1
    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
        println!("Success!");
    }

    //------------------------------------------------

    let a: FooBar = FooBar::Qux(10);

    if let FooBar::Bar = a {
        println!("FooBar::Bar matched");
    } else if let FooBar::Baz = a {
        println!("FooBar::Baz matched");
    } else {
        println!("Something else matched");
    }

    match a {
        // doe sthe same thing as the if let above, but looks nicer
        FooBar::Bar => println!("Foo::Bar matched"),
        FooBar::Baz => println!("Foo::Baz matched"),
        _ => println!("Something else matched"),
    }
}
