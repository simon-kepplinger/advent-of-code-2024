use std::fs;

fn main() {
    let content = fs::read_to_string("in/input")
        .expect("could not open file in/input");

    println!("{}", content);
}
