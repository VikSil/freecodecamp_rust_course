//pub mod back_of_house { // this is not needed, since back_of_house module is already declared in lib.rs file
fn fix_incorrect_order() {
    cook_order();

    // super::front_of_house::serving::serve_order(); // if this code was in lib.rs, then super could be used to refer to parent one level higher, i.e. lib.rs itself

    crate::front_of_house::serving::serve_order(); // absolute path
}

pub fn cook_order() {}
//}
