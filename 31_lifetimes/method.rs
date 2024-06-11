struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("print: {}", self.0);
    }
}

struct ImportantExcerpt<'a> { // full annotation with lifetime
    part: &'a str,
}

impl ImportantExcerpt<'_> {
    // full annotation with lifetime
    fn level<'a>(&'a self) -> i32 {
        3
    }
}

// same struct as above but with static lifetime does not require any additional lifetime annotation
// i.e. the variable with static lifetime lives everywhere in the program
struct AnotherImportantExcerpt {
    part: &'static str,
}

impl AnotherImportantExcerpt {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
