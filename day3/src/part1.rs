use std::fs;
use regex::Regex;
use aoc_core::read;

fn main() {
    let input = read("in/input");
    let matcher = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: u32 = matcher.captures_iter(&input)
        .map(|cap| {
            let a: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let b: u32 = cap.get(2).unwrap().as_str().parse().unwrap();

            a * b
        })
        .sum();

    println!("{:?}", sum);
}