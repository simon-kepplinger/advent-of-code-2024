use std::collections::HashMap;
use aoc_core::read;

fn main() {
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
        let pages: Vec<u8> = update
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        if is_valid_update(&pages, &rules) {
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
}

fn is_valid_update(pages: &[u8],
                   rules: &HashMap<u8, Vec<u8>>) -> bool {
    for (i, p) in pages.iter().enumerate() {
        if rules.contains_key(p) {
            let rule = &rules[p];
            let leading = &pages[..i];

            if leading.iter().any(|p| rule.contains(p)) {
                return false;
            }
        }
    }

    true
}
