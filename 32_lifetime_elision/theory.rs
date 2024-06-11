// the compiler uses these three rules to figure out lifetimes of references that are not explicitly annotated:
// 1. Compiler assigns a lifetime parameter to each reference
// 2. If there is only one input lifetime, that lifetime is assigned to all output parameters
// 3. If there are multiple input lifetime parameters, but one of them is &self or & mut self, then the lifetime of self is assigned to all output parameters

//e.g.

fn version1(s: &str) -> &str {"return this"} // initial unannotated function
// 1. compiler applies a lifetime to each reference
// above function becomes
fn version2<'a>(s: &'a str) -> &str {"return this"}
// 2. since there is only one input lifetime, compiler applies it to all output references
// above function becomes
fn version3<'a>(s: &'a str) -> &'a str {"return this"}

// another e.g.

// fn longest1(x: &str, y: &str) -> &str{"return this"} // compiler panics because it cannot infer output lifetime

// 1. compiler applies a lifetime to each reference
// above function becomes

// fn longest2<'a, 'b>(x: &'a str, y: &'b str)-> &str{"return this"}  // compiler panics because it cannot infer output lifetime

// the second rule cannot be applied here because there is more than one input lifetime
// the third rule cannot be applied here because this function is not a method, i.e. it does not take &self as an input parameter
// hence, lifetimes have to be annotated manually, depending on which lifetime will be returned

// another e.g.
struct ImportantExcerpt<'a> { // full annotation with lifetime
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please {}",announcement);
        self.part
    }
}
// here the third rule applies because one of the inputs is &self
// all references will have the same lifetime as &self
// this is implicit and thus lifetimes don't need to be annotated

fn main() {}