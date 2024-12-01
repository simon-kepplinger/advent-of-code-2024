use std::collections::HashMap;
use std::fs;

fn main() {
    let values: Vec<u32> = read("in/input")
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    let (left, right): (Vec<_>, Vec<_>) = values
        .into_iter()
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    let mut count_map = HashMap::new();
    
    for (_, v) in right {
        let count = count_map.entry(v).or_insert(0);
        *count += 1;
    }

    let mut sum: u32 = 0;

    for (_, v) in left  {
        let count = count_map.get(&v).unwrap_or(&0);
        sum += count * v;
    }

    println!("{}", sum);
}

fn read(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("could not open file {}", path))
}