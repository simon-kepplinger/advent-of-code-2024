use std::fs;
use aoc_core::read;

fn main() {
    let sum: u32 = read("in/input")
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .map(|v| is_secure_with_dampener(v))
        .sum();

    println!("{:?}", sum);
}

fn is_secure_with_dampener(levels: Vec<i32>) -> u32 {
    let mut variants: Vec<Vec<i32>> = vec![levels.clone()];

    for i in 0..levels.len() {
        let mut copy = levels.clone();
        copy.remove(i);
        variants.push(copy);
    }

    variants.iter()
        .map(|v| is_secure(v.clone()))
        .reduce(|sum, n| if n > 0 { 1 } else { sum })
        .unwrap()
}

fn is_secure(levels: Vec<i32>) -> u32 {
    let mut diff= Vec::with_capacity(levels.len() - 1);

    for i in 0..levels.len() - 1 {
        diff.push(levels[i] - levels[i + 1]);
    }

    let is_full_pos = diff.iter().all(|n| *n < 0);
    let is_full_neg = diff.iter().all(|n| *n > 0);
    let is_in_range = diff.iter().all(|n| 1 <= n.abs() && n.abs() <= 3);

    if is_in_range && (is_full_pos || is_full_neg) {
        1
    } else {
        0
    }
}