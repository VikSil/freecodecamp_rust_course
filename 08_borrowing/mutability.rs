fn main() {
    let mut s: String = String::from("Hello"); // this needs to be mutable to begin with to be passed on as a mutable reference

    borrow_mut_object(&mut s);

    borrow_imut_object(&s); // but it is perfectly ok to borrow a mutable object as an imutable reference

    s.push_str(" World!");

    println!("Success!");
}

fn borrow_mut_object(s: &mut String) {}

fn borrow_imut_object(s: &String) {
    // the mutability in the function statement and the call has to match

}
