#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn new() -> Self {
        // 'Self' in an impl block means the same as type name
        Self {
            color: String::from("red"),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
    println!("Success!");
}
