use aoc_core::{end_measure, read, start_measure};
use regex::Regex;

#[derive(Debug)]
struct ClawMachine {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    rx: f64,
    ry: f64,
}

impl ClawMachine {
    fn from_string(s: &str, price_offset: f64) -> Self {
        let captures: Vec<Vec<_>> = s
            .lines()
            .map(|l| {
                Regex::new(r"(=|\+)([\d]+)")
                    .unwrap()
                    .captures_iter(l)
                    .collect()
            })
            .collect();

        ClawMachine {
            ax: captures[0][0][2].parse::<f64>().unwrap(),
            ay: captures[0][1][2].parse::<f64>().unwrap(),
            bx: captures[1][0][2].parse::<f64>().unwrap(),
            by: captures[1][1][2].parse::<f64>().unwrap(),
            rx: price_offset + captures[2][0][2].parse::<f64>().unwrap(),
            ry: price_offset + captures[2][1][2].parse::<f64>().unwrap(),
        }
    }

    fn get_cheapest_tokens(&self) -> f64 {
        let nb = (self.rx * self.ay - self.ry * self.ax) / (self.bx * self.ay - self.by * self.ax);
        let na = (self.ry - nb * self.by) / self.ay;

        if na.fract() != 0.0 || nb.fract() != 0.0 {
            return 0.0;
        }

        na * 3.0 + nb
    }
}

fn main() {
    let mes = start_measure();

    let input = read("in/input");

    let sum: f64 = input
        .split("\n\n")
        .map(|l| ClawMachine::from_string(l, 10000000000000.0).get_cheapest_tokens())
        .sum();

    println!("{:?}", sum);

    end_measure(mes);
}
