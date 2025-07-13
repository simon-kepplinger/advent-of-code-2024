use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use aoc_core::{end_measure, read, spatial::Point, start_measure};

#[derive(Debug)]
struct KeyPad {
    keys: HashMap<char, Point>,
    pointer: char,
    cache: HashMap<char, HashMap<char, HashSet<String>>>,
}

impl KeyPad {
    pub fn arrow_pad() -> Self {
        KeyPad::new(HashMap::from([
            ('^', Point { x: 1, y: 0 }),
            ('A', Point { x: 2, y: 0 }),
            ('<', Point { x: 0, y: 1 }),
            ('V', Point { x: 1, y: 1 }),
            ('>', Point { x: 2, y: 1 }),
        ]))
    }

    pub fn num_pad() -> Self {
        KeyPad::new(HashMap::from([
            ('7', Point { x: 0, y: 0 }),
            ('8', Point { x: 1, y: 0 }),
            ('9', Point { x: 2, y: 0 }),
            ('4', Point { x: 0, y: 1 }),
            ('5', Point { x: 1, y: 1 }),
            ('6', Point { x: 2, y: 1 }),
            ('1', Point { x: 0, y: 2 }),
            ('2', Point { x: 1, y: 2 }),
            ('3', Point { x: 2, y: 2 }),
            ('0', Point { x: 1, y: 3 }),
            ('A', Point { x: 2, y: 3 }),
        ]))
    }

    fn new(keys: HashMap<char, Point>) -> Self {
        KeyPad {
            keys,
            cache: HashMap::new(),
            pointer: 'A',
        }
    }

    pub fn press(&mut self, sequence: &str) -> Vec<String> {
        let mut branches = vec![];

        for key in sequence.chars() {
            let parts = self
                .cache
                .get(&self.pointer)
                .unwrap()
                .get(&key)
                .unwrap();

            if branches.len() == 0 {
                branches = parts.to_owned().into_iter().collect();
            } else {
                let mut new_branches = vec![];

                for branch in branches {
                    for part in parts {
                        new_branches.push(branch.to_owned() + part);
                    }
                }

                branches = new_branches;
            }

            self.pointer = key;
        }

        branches
    }

    pub fn fill_cache(&mut self) {
        for (from_key, from_point) in self.keys.clone() {
            let other_keys = self.keys.iter().filter(|k| *k.0 != from_key);
            let mut inner_cache = HashMap::new();
            inner_cache.insert(from_key, HashSet::from(["A".to_string()]));

            for (to_key, to_point) in other_keys {
                let dx = to_point.x - from_point.x;
                let dy = to_point.y - from_point.y;

                let x_char = if dx > 0 { ">" } else { "<" };
                let y_char = if dy > 0 { "V" } else { "^" };

                let input: Vec<char> = x_char
                    .repeat(dx.abs() as usize)
                    .chars()
                    .chain(y_char.repeat(dy.abs() as usize).chars())
                    .collect();

                let unique_perms: HashSet<String> = input
                    .iter()
                    .permutations(input.len())
                    .map(|p| p.into_iter().collect())
                    .map(|mut s: String| {
                        s.push('A');
                        s
                    })
                    .filter(|s| KeyPad::filter_blocked_paths(s, from_key))
                    .collect();

                inner_cache.insert(*to_key, unique_perms);
            }

            self.cache.insert(from_key, inner_cache);
        }
    }

    pub fn filter_blocked_paths(sequence: &str, from_key: char) -> bool {
        match from_key {
            '7' => !sequence.starts_with("VVV"),
            '4' => !sequence.starts_with("VV"),
            '1' => !sequence.starts_with("V"),
            '0' => !sequence.starts_with("<"),
            'A' => !sequence.starts_with("<<"),
            '^' => !sequence.starts_with("<"),
            '<' => !sequence.starts_with("^"),
            _ => true,
        }
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    println!("{}", input);

    let mut num_pad = KeyPad::num_pad();
    let mut arrow_pad_0 = KeyPad::arrow_pad();
    let mut arrow_pad_1 = KeyPad::arrow_pad();

    num_pad.fill_cache();
    arrow_pad_0.fill_cache();
    arrow_pad_1.fill_cache();

    let mut sum = 0;

    for sequence in input.lines() {
        let res: Vec<_> = num_pad
            .press(sequence.trim())
            .into_iter()
            .map(|seq| arrow_pad_0.press(seq.as_str()))
            .flatten()
            .map(|seq| arrow_pad_1.press(seq.as_str()))
            .flatten()
            .collect();

        let shortest_char_count = res
            .iter()
            .map(|s| s.chars().count())
            .min()
            .unwrap_or(0);

        let numeric_part: usize = sequence[..3].parse().unwrap();
        println!("------");
        println!("seq: {}", sequence);
        println!("{:?}", shortest_char_count);
        println!("{} * {}", shortest_char_count, numeric_part);
        sum += shortest_char_count * numeric_part;
    }

    println!();
    println!("sum: {}", sum);

    end_measure(mes);
}
