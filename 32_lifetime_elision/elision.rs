// some lifetime patterns are so common that the borrow checker will allow omission of them for readability - this is known as "elision"

fn noelision_input<'a>(x: &'a i32) {
    println!("annotated input: {}",x);
}
// the above only has one input argument
// and it does not return anything
// hence lifetimes of inputs and the function itself does not matter at all

fn elision_input(x: &i32) {
    println!("annotated input: {}",x);
}

//------------------------------------------------

fn noelision_pass<'a>(x: &'a i32) ->&'a i32 {x}
// only one input reference
// hence compiler will infer that the output reference has the same lifetime
// no annotation needed

fn elision_pass(x: &i32) ->&i32 {x}

//------------------------------------------------

fn noelision_longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str{
    x
}

// two input references and neither is self
// hence full lifetime annotation is needed
// function cannot be shortened

//------------------------------------------------

struct NoelisionOwner(i32);

impl NoelisionOwner {
    fn noelision_add_one<'a>(&'a mut self) {self.0 +=1;}
    fn noelision_print<'a>(&'a self){
        println!("print: {}",self.0);
    }
}

// this struct implements methods (i.e. functions to &self)
// when there is a reference to self, lifetimes are not necessary
// all references passed to a method will get the same lifetimes as self

struct ElisionOwner(i32);

impl ElisionOwner {
    fn noelision_add_one(&mut self) {self.0 +=1;}
    fn noelision_print(&self){
        println!("print: {}",self.0);
    }
}

//------------------------------------------------

struct NoelisionPerson<'a> {
    age: u8,
    name: &'a str,
}

// lifetime annotation here is needed to ensure that the name lives longer than the NoelisionPerson
// otherwise the compiler cannot know
// but the &str can be made static in which case the struct does not need a lifetime
// since &str variables are hardcoded into the binary, they are in fact always static

struct ElisionPerson {
    age: u8,
    name: &'static str,
}

//------------------------------------------------

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

// lifetime annotation here is needed to ensure that the i32 lives longer than the Either
// otherwise the compiler cannot know

//------------------------------------------------

fn main() {}