use std::collections::VecDeque;

use aoc_core::{end_measure, read, start_measure};

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Key,
    Lock,
}

#[derive(Debug)]
struct Schematic {
    typ: Type,
    sequence: Vec<u8>,
}

impl Schematic {
    fn from_string(input: &str) -> Self {
        let mut lines: VecDeque<_> = input.lines().take(6).collect();
        let first = lines.pop_front();

        let mut typ = Type::Key;
        let mut sequence = vec![0; 5];

        if first == Some("#####") {
            typ = Type::Lock;
        }

        for i in 0..5 {
            for j in 0..5 {
                if lines[j].chars().nth(i) == Some('#') {
                    sequence[i] += 1;
                }
            }
        }

        Schematic { typ, sequence }
    }

    fn match_with(&self, other: &Schematic) -> bool {
        self.sequence
            .iter()
            .zip(other.sequence.clone())
            .map(|(a, b)| a + b)
            .all(|n| n <= 5)
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let (keys, locks): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .map(|i| Schematic::from_string(i))
        .partition(|s| s.typ == Type::Lock);

    let mut count = 0;

    for lock in &locks {
        for key in &keys {
            if key.match_with(lock) {
                count += 1;
            }
        }
    }

    println!("{count}");

    end_measure(mes);
}
