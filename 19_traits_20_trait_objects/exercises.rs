trait Hello {
    fn say_hi(&self) -> String{
        String::from("hi") // default implementation, any type implementing Hello trait automatically has say_hi method
    }

    fn say_something(&self) ->String; // only method signature is provided, there is no default
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) ->String{
        String::from("I'm a good student")
    }
}
struct Teacher {}
impl Hello for Teacher{
    fn say_hi(&self) -> String{ // overrides the default implementation that is inherent in the trait
        String::from("Hi, I'm your new teacher")
    }
    fn say_something(&self) ->String{
        String::from("I'm not a bad teacher")
    }
}

fn main() {
    let s: Student = Student {};
    assert_eq!(s.say_hi(),"hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t: Teacher = Teacher {};
    assert_eq!(t.say_hi(),"Hi, I'm your new teacher");
    assert_eq!(t.say_something(),"I'm not a bad teacher");

    println!("Success!");

    //------------------------------------------------

    
}