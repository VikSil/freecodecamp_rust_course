#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            Self::Green => "green", // 'Self' can be used instead of the type name
        }
    }
}

fn main() {
    let c = TrafficLightColor::Yellow; // instantiating TrafficLightColor type with variant Yellow
    assert_eq!(c.color(), "yellow");
    println!("{:?}", c);
}
