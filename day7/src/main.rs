use aoc_core::{end_measure, read, start_measure};
use itertools::Itertools;

#[derive(Debug)]
struct Equation {
    result: u128,
    numbers: Vec<u128>,
}

#[derive(Debug, Clone)]
enum Operator {
    Multiply,
    Addition,
    Concat
}

const OPERATORS: [Operator; 3] = [
    Operator::Addition,
    Operator::Multiply,
    Operator::Concat
];

fn main() {
    let mes = start_measure();

    let equations: Vec<Equation> = read("in/input")
        .lines()
        .map(|l| l.split(':').collect())
        .map(|s: Vec<&str>| Equation {
            result: s[0].parse().unwrap(),
            numbers: s[1]
                .trim()
                .split(' ')
                .map(|n| n.trim().parse().unwrap())
                .collect(),
        })
        .collect();

    let sum: u128 = equations
        .iter()
        .map(|eq| {
            let is_valid = (0..eq.numbers.len() - 1)
                .map(|_| OPERATORS)
                .multi_cartesian_product()
                .any(|op| {
                    let mut try_op = op.iter();

                    let res = eq.numbers
                        .clone()
                        .into_iter()
                        .reduce(|a, b| match try_op.next().unwrap() {
                            Operator::Multiply => a * b,
                            Operator::Addition => a + b,
                            Operator::Concat => a * 10u128.pow(get_digit_count(b)) + b,
                        })
                        .unwrap();

                    eq.result == res
                });

            if is_valid { eq.result } else { 0 }
        })
        .sum();

    println!("{:?}", sum);
    end_measure(mes);
}

fn get_digit_count(mut x: u128) -> u32 {
    let mut count = 1;
    while x >= 10 {
        x /= 10;
        count += 1;
    }
    count
}
