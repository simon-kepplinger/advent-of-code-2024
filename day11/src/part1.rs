use aoc_core::{end_measure, get_digit_count_fast, read, start_measure};

fn main() {
    let mes = start_measure();

    let mut input: Vec<u64> = read("in/input")
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for progress in 0..25 {
        let mut i = 0;

        while i < input.len() {
            if input[i] == 0 {
                input[i] = 1;
            } else {
                let digit_count = get_digit_count_fast(input[i]);

                if digit_count % 2 == 0 {
                    let (left, right) = split_number(input[i], digit_count);
                    input[i] = right;
                    input.insert(i, left);

                    i += 1;
                } else {
                    input[i] *= 2024;
                }
            }

            i += 1;
        }
    }

    println!("{:?}", input.len());

    end_measure(mes);
}

fn split_number(number: u64, digit_count: u32) -> (u64, u64) {
    let factor = 10u64.pow(digit_count / 2);

    (number / factor, number % factor)
}
