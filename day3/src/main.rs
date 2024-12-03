use std::fs;
use regex::Regex;

fn main() {
    let mut input = read("in/input");
    input.insert_str(0, "do()"); // add provisional do as the beginning will act as a "do()"
    input = input.replace("\n", ""); // remove newlines because the somehow break the regex findings

    let match_dos = Regex::new(r"do\(\).*").unwrap();

    let splitted: Vec<&str> = input.split("don't()")
        .map(|s| {
            match match_dos.find(s) {
                Some(s) => s.as_str(),
                None => ""
            }
        })
        .collect();

    let sum: u32 = splitted.into_iter()
        .map(|s| multiply(s))
        .sum();

    println!("{:?}", sum);
}

fn multiply(memory: &str) -> u32 {
    let match_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    match_mul.captures_iter(memory)
        .map(|cap| {
            let a: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let b: u32 = cap.get(2).unwrap().as_str().parse().unwrap();

            a * b
        })
        .sum()
}

fn read(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("could not open file {}", path))
}