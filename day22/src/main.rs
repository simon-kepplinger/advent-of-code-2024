use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
};

use aoc_core::{end_measure, read, start_measure};

struct SecretGen {
    secret: i64,
}

impl SecretGen {
    fn new(initial: i64) -> Self {
        SecretGen { secret: initial }
    }

    fn next(&mut self) -> i64 {
        self.mix(self.secret * 64);
        self.prune();

        self.mix(self.secret / 32);
        self.prune();

        self.mix(self.secret * 2048);
        self.prune();

        self.secret
    }

    fn mix(&mut self, value: i64) {
        self.secret = value ^ self.secret;
    }

    fn prune(&mut self) {
        self.secret = self.secret % 16777216;
    }
}

#[derive(Debug, Clone)]
struct PriceChange {
    price: i64,
    change: Option<i64>,
    hash: Option<u64>,
}

impl PriceChange {
    fn new(price: i64, change: Option<i64>, hash: Option<u64>) -> Self {
        PriceChange {
            price,
            change,
            hash,
        }
    }

    fn from(price: i64, predecessors: Vec<PriceChange>) -> Self {
        let mut hash = None;

        let prior_changes: Vec<_> = predecessors
            .iter()
            .filter_map(|p| p.change)
            .collect();

        let last = predecessors.last();
        let change = match last {
            Some(l) => Some(price - l.price),
            None => None,
        };

        if let Some(c) = change {
            if prior_changes.len() == 3 {
                let mut hasher = DefaultHasher::new();

                let mut sequence = prior_changes
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                sequence += ",";
                sequence += &c.to_string();

                sequence.hash(&mut hasher);

                hash = Some(hasher.finish());
            }
        }

        PriceChange {
            price,
            change,
            hash,
        }
    }
}

fn to_changes(mut gen: SecretGen, cycles: u32) -> Vec<PriceChange> {
    let initial = PriceChange::new(gen.secret % 10, None, None);

    let mut changes: Vec<_> = vec![initial.clone()];
    let mut predecessors: VecDeque<PriceChange> = VecDeque::with_capacity(4);
    predecessors.push_back(initial);

    for _ in 0..cycles {
        let secret = gen.next();
        let change = PriceChange::from(
            secret % 10,
            predecessors.iter().map(|p| p.clone()).collect(),
        );

        changes.push(change.clone());
        predecessors.push_back(change);

        if predecessors.len() > 3 {
            predecessors.pop_front();
        }
    }

    changes
}

fn filter_duplicates(changes: Vec<PriceChange>) -> Vec<PriceChange> {
    let mut seen = HashSet::new();
    let mut filtered_changes = vec![];

    for change in changes {
        if let Some(hash) = change.hash {
            if !seen.contains(&hash) {
                filtered_changes.push(change);
                seen.insert(hash);
            }
        }
    }

    filtered_changes
}

fn group_changes(changes: Vec<PriceChange>) -> HashMap<u64, i64> {
    let mut map = HashMap::new();

    for change in changes {
        if let Some(hash) = change.hash {
            *map.entry(hash).or_insert(0) += change.price;
        }
    }

    map
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let gens = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .map(|n| SecretGen::new(n));

    let all_changes: Vec<_> = gens
        .map(|g| to_changes(g, 2000))
        .map(|g| filter_duplicates(g))
        .flatten()
        .collect();

    let grouped_changes = group_changes(all_changes);
    let mut values: Vec<_> = grouped_changes.values().collect();
    values.sort();

    let largest = grouped_changes
        .values()
        .max_by_key(|c| *c)
        .unwrap_or(&0_i64);

    println!("{:#?}", largest);
    println!("");

    end_measure(mes);
}
