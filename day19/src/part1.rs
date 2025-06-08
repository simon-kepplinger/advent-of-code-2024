use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use aoc_core::{end_measure, read, start_measure};

#[derive(Debug)]
struct Onsen {
    towels: HashSet<String>,
    designs: Vec<String>,

    towel_limit: usize,
    cache: HashMap<String, bool>,
}

impl Onsen {
    pub fn from_string(input: String) -> Self {
        let (towles_str, designs_str) = input.split_once("\n\n").unwrap();

        let towels: HashSet<_> = towles_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let towel_limit = towels
            .iter()
            .map(|t| t.len())
            .reduce(|max, curr| if curr > max { curr } else { max })
            .unwrap();

        return Onsen {
            towels,
            designs: designs_str
                .lines()
                .map(|s| s.to_string())
                .collect(),
            towel_limit,
            cache: HashMap::new(),
        };
    }

    pub fn match_design(&mut self, design: &str) -> bool {
        let limit = cmp::min(self.towel_limit, design.len());
        let mut buffer = String::with_capacity(limit);

        let cached = self.cache.get(design);

        if cached.is_some() {
            return *cached.unwrap();
        }

        if design.len() == 0 {
            return true;
        }

        for c in design.chars().take(limit) {
            buffer.push(c);
            let towel_match = self.towels.get(&buffer);

            let does_match = match towel_match {
                Some(towel) => self.match_design(&design[towel.len()..]),
                None => false,
            };

            self.cache.insert(design.to_string(), does_match);
            if does_match {
                return true;
            }
        }

        self.cache.insert(design.to_string(), false);
        false
    }

    pub fn get_matching_count(&mut self) -> usize {
        let designs = self.designs.clone().into_iter();

        return designs
            .map(|d| self.match_design(d.as_str()))
            .filter(|m| *m)
            .count();
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let mut onsen = Onsen::from_string(input);
    println!("{}", onsen.get_matching_count());

    end_measure(mes);
}
