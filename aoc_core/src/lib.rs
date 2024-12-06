pub mod grid;
pub mod spatial;

use std::{fs};

pub fn read(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("could not open file {}", path))
}