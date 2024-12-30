use aoc_core::spatial::Point;
use aoc_core::{end_measure, read, start_measure};
use regex::Regex;

#[derive(Debug)]
struct ClawMachine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

enum AmountResult<T> {
    Some(T),
    None,
    Overflow,
}

impl ClawMachine {
    fn from_string(s: &str) -> Self {
        let points: Vec<_> = s
            .lines()
            .map(|l| {
                let matches: Vec<_> = Regex::new(r"(=|\+)([\d]+)")
                    .unwrap()
                    .captures_iter(l)
                    .collect();

                Point {
                    x: matches[0][2].parse::<i32>().unwrap(),
                    y: matches[1][2].parse::<i32>().unwrap(),
                }
            })
            .collect();

        ClawMachine {
            button_a: points[0],
            button_b: points[1],
            prize: points[2],
        }
    }

    fn get_cheapest_tokens(&self) -> i32 {
        let mut amount_a = 0;

        loop {
            let amount_b = self.get_b_amount(amount_a);

            match amount_b {
                AmountResult::Some(b) => {
                    if self.play(amount_a, b) {
                        return amount_a * 3 + b;
                    }
                }
                AmountResult::None => {}
                AmountResult::Overflow => {
                    return 0;
                }
            }

            amount_a += 1;
        }
    }

    fn play(&self, amount_a: i32, amount_b: i32) -> bool {
        self.button_a.x * amount_a + self.button_b.x * amount_b == self.prize.x
            && self.button_a.y * amount_a + self.button_b.y * amount_b == self.prize.y
    }

    fn get_b_amount(&self, amount_a: i32) -> AmountResult<i32> {
        let remaining_x = self.prize.x - self.button_a.x * amount_a;
        let remaining_y = self.prize.y - self.button_a.y * amount_a;

        if remaining_x <= 0 || remaining_y <= 0 {
            return AmountResult::Overflow;
        }

        let amount_b = remaining_x / self.button_b.x;

        // this is a little hack i think
        if (amount_b == 0 || remaining_x % amount_b != 0)
            && amount_b != remaining_y / self.button_b.y
        {
            return AmountResult::None;
        }

        AmountResult::Some(amount_b)
    }
}

fn main() {
    let mes = start_measure();

    let input = read("in/input");

    let machines: Vec<_> = input
        .split("\n\n")
        .map(ClawMachine::from_string)
        .collect();

    let cheapest: i32 = machines
        .iter()
        .map(|m| m.get_cheapest_tokens())
        .sum();

    println!("{:?}", cheapest);

    end_measure(mes);
}
