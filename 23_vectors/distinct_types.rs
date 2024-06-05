#[derive(Debug, PartialEq)] // PartialEq necessary for assert_eq macro
enum ipAddr {
    V4(String),
    V6(String),
}

trait IP { // traits and enums are both objects and hence variable name cannot be use simultaneously for both
    fn display(&self);
}

struct V4(String); // this is a struct while previously V4 was an enum field - those are not in conflict
impl IP for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}

struct V6(String); // this is a struct while previously V4 was an enum field - name can be used simultaneously
impl IP for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn main() {
    // let v = vec![1,2.0,3]; // this would panic because all elements in a vector must be of the same type

    let v: Vec<ipAddr> = vec![
        // using Enum as vectory type allows storing different type instances in the vector
        ipAddr::V4("127.0.0.1".to_string()),
        ipAddr::V6("::1".to_string())
    ];

    assert_eq!(v[0], ipAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], ipAddr::V6("::1".to_string()));

    println!("Success!");

    //------------------------------------------------

    // vector of boxes of trait objects of any type that implements IP trait
    let v: Vec<Box<dyn IP>> =  vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
