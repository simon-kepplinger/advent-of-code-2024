use std::collections::HashMap;
use aoc_core::{end_measure, read, start_measure};

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let lines: Vec<&str> = input
        .split("\n\n")
        .collect();

    let rules_entries: Vec<&str> = lines[0].lines().collect();
    let updates: Vec<&str> = lines[1].lines().collect();

    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();

    for rules_entry in rules_entries {
        let mut split = rules_entry.split('|');
        let key: u8 = split.next().unwrap().parse().unwrap();
        let value: u8 = split.next().unwrap().parse().unwrap();

        rules.entry(key).or_insert_with(Vec::new).push(value);
    }

    let mut middles = vec![];

    for update in updates {
        let mut pages: Vec<u8> = update
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        if swap_invalid(&mut pages, &rules) {
            middles.push(
                pages[pages.len() / 2]
            )
        }
    }

    let sum: u32 = middles
        .iter()
        .map(|n| *n as u32)
        .sum();

    println!("{}", sum);

    end_measure(mes);
}

fn swap_invalid(pages: &mut Vec<u8>,
                rules: &HashMap<u8, Vec<u8>>) -> bool {
    let mut i = 0;
    let mut is_invalid = false;
    let len = pages.len();

    while i < len {
        let mut swapped = false;
        let page = pages.get(i).unwrap();

        if rules.contains_key(page) {
            let rule = &rules[page];

            swapped = swap_if_invalid(i, pages, rule);
            is_invalid = is_invalid || swapped;
        }

        if !swapped {
            i += 1;
        }
    }

    is_invalid
}

fn swap_if_invalid(i: usize,
                   pages: &mut [u8],
                   rule: &[u8]) -> bool {
    let leading = &pages[..i];

    for l_index in 0..leading.len() {
        let leader = &leading.get(l_index).unwrap();

        if rule.contains(leader) {
            pages.swap(i, l_index);
            return true;
        }
    }

    false
}
