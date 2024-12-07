pub mod grid;
pub mod spatial;

use std::{fs};
use std::time::Instant;

pub fn start_measure() -> Instant {
    Instant::now()
}

pub fn end_measure(measure: Instant) {
    println!("\nFinished in {:?}", measure.elapsed());
}

pub fn read(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("could not open file {}", path))
}