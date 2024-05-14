fn main() {
    for c in "你好, 张伟".chars() { // .chars() converts a string literal into an itterator of separate chars
        println!("{}", c)
    }

}