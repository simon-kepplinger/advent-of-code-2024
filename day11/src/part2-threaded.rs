use aoc_core::{end_measure, get_digit_count_fast, read, start_measure};
use std::thread;

fn main() {
    let mes = start_measure();

    let input: Vec<u64> = read("in/example")
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut handles = Vec::new();

    let mut array = vec![0; 100_000_000usize];
    let mut size = input.len();

    array[..input.iter().len()].copy_from_slice(&input[..input.iter().len()]);

    for _ in 0..30 {
        let mut i = 0;
        let cycle_size = size;

        while i < cycle_size {
            if array[i] == 0 {
                array[i] = 1;
            } else {
                let digit_count = get_digit_count_fast(array[i]);

                if digit_count % 2 == 0 {
                    let (left, right) = split_number(array[i], digit_count);
                    array[i] = right;
                    array[size] = left;

                    size += 1;
                } else {
                    array[i] *= 2024;
                }
            }

            i += 1;
        }
    }

    println!("Handle {} batches async", size);

    let mut sum: usize = 0;
    let mut async_i = 0;

    let mut handled_batches = 0;

    while async_i < size {
        while async_i % 16 != 15 {
            handles.push(calc_async(array[async_i], 45));
            async_i += 1;
        }

        // println!("Resolve {} batches in parallel", handles.len());
        handled_batches += handles.len();

        for handle in handles {
            sum += handle.join().unwrap();

            // println!("Finished batch!!");
        }

        handles = vec![];
        println!(
            "Handeled {}/{} batches ({}%)",
            handled_batches,
            size,
            handled_batches as f32 / size as f32 * 100f32
        );

        async_i += 1;
    }

    println!("{:?}", sum);

    end_measure(mes);
}

fn calc_async(number: u64, cycles: u8) -> thread::JoinHandle<usize> {
    thread::spawn(move || {
        let mut array = vec![0; 500_000_000usize];
        let mut size = 1;
        array[0] = number;

        for _ in 0..cycles {
            let mut i = 0;
            let cycle_size = size;

            while i < cycle_size {
                if array[i] == 0 {
                    array[i] = 1;
                } else {
                    let digit_count = get_digit_count_fast(array[i]);

                    if digit_count % 2 == 0 {
                        let (left, right) = split_number(array[i], digit_count);
                        array[i] = right;
                        array[size] = left;

                        size += 1;
                    } else {
                        array[i] *= 2024;
                    }
                }

                i += 1;
            }
        }

        size
    })
}

fn split_number(number: u64, digit_count: u32) -> (u64, u64) {
    let half_digits = (digit_count / 2).min((POWERS_OF_10.len() - 1) as u32) as usize;
    let factor = POWERS_OF_10[half_digits];

    (number / factor, number % factor)
}

const POWERS_OF_10: [u64; 20] = [
    1,                          // 10^0
    10,                         // 10^1
    100,                        // 10^2
    1_000,                      // 10^3
    10_000,                     // 10^4
    100_000,                    // 10^5
    1_000_000,                  // 10^6
    10_000_000,                 // 10^7
    100_000_000,                // 10^8
    1_000_000_000,              // 10^9
    10_000_000_000,             // 10^10
    100_000_000_000,            // 10^11
    1_000_000_000_000,          // 10^12
    10_000_000_000_000,         // 10^13
    100_000_000_000_000,        // 10^14
    1_000_000_000_000_000,      // 10^15
    10_000_000_000_000_000,     // 10^16
    100_000_000_000_000_000,    // 10^17
    1_000_000_000_000_000_000,  // 10^18
    10_000_000_000_000_000_000, // 10^19
];
