pub mod grid;
pub mod spatial;

use std::fs;
use std::time::Instant;

pub fn start_measure() -> Instant {
    Instant::now()
}

pub fn end_measure(measure: Instant) {
    println!("\nFinished in {:?}", measure.elapsed());
}

pub fn read(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("could not open file {}", path))
}

pub fn get_digit_count_fast(x: u64) -> u32 {
    if x < 10 {
        1
    } else if x < 100 {
        2
    } else if x < 1_000 {
        3
    } else if x < 10_000 {
        4
    } else if x < 100_000 {
        5
    } else if x < 1_000_000 {
        6
    } else if x < 10_000_000 {
        7
    } else if x < 100_000_000 {
        8
    } else if x < 1_000_000_000 {
        9
    } else if x < 10_000_000_000 {
        10
    } else if x < 100_000_000_000 {
        11
    } else if x < 1_000_000_000_000 {
        12
    } else if x < 10_000_000_000_000 {
        13
    } else if x < 100_000_000_000_000 {
        14
    } else if x < 1_000_000_000_000_000 {
        15
    } else if x < 10_000_000_000_000_000 {
        16
    } else if x < 100_000_000_000_000_000 {
        17
    } else if x < 1_000_000_000_000_000_000 {
        18
    } else if x < 10_000_000_000_000_000_000 {
        19
    } else {
        20
    }
}
