use aoc_core::{end_measure, get_digit_count_fast, read, start_measure};
use std::collections::HashMap;

fn main() {
    let mes = start_measure();

    let input: Vec<u64> = read("in/input")
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    const CYCLES: u8 = 75;

    let mut lookup = HashMap::new();

    let res = input.into_iter().fold(0, |sum, num| {
        sum + blink(num, CYCLES, &mut lookup)
    });

    println!("{:?}", res);

    end_measure(mes);
}

fn blink(num: u64, cycles: u8, lookup: &mut HashMap<(u64, u8), usize>) -> usize {
    if cycles == 0 {
        return 1;
    }

    let key = (num, cycles);

    if lookup.contains_key(&key) {
        return lookup[&key];
    }

    let mut res = 0;

    if num == 0 {
        res = blink(1, cycles - 1, lookup)
    } else {
        let digit_count = get_digit_count_fast(num);

        if digit_count % 2 == 0 {
            let (left, right) = split_number(num, digit_count);
            res = blink(left, cycles - 1, lookup) + blink(right, cycles - 1, lookup)
        } else {
            let product = num * 2024;
            res = blink(product, cycles - 1, lookup)
        }
    }

    lookup.insert(key, res);

    res
}

fn split_number(number: u64, digit_count: u32) -> (u64, u64) {
    let factor = 10u64.pow(digit_count / 2);

    (number / factor, number % factor)
}
