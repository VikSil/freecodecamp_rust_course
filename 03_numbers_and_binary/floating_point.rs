fn main() {
    let x = 1000.0001; // by default this will be f64
    let y: f32 = 0.12;
    let z = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());

    //------------------------------------------------

    // assert!(0.1 + 0.2 == 0.3); // this would panic due to f64 precision where 0.1 + 0.2 = 0.300000000000000000001
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32); // f32 is less precise and this should work
    assert!((0.1 as f32) + (0.2 as f32) == (0.3 as f32));

    //------------------------------------------------

    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
