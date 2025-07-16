use std::collections::HashMap;

use aoc_core::{end_measure, read, start_measure};

#[derive(Debug)]
struct Lan {
    connections: HashMap<String, Vec<String>>,
}

impl Lan {
    fn from_string(input: String) -> Self {
        let con_strings: Vec<_> = input
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .collect();

        let mut connections: HashMap<String, Vec<String>> =
            HashMap::with_capacity(con_strings.len() * 2);

        for con in con_strings {
            connections
                .entry(con.0.to_string())
                .or_default()
                .push(con.1.to_string());

            connections
                .entry(con.1.to_string())
                .or_default()
                .push(con.0.to_string());
        }

        Lan { connections }
    }

    fn get_inter_cons(&self, level: u32) -> Vec<Vec<String>> {
        let mut inter_cons = vec![];

        for con in self.connections.keys() {
            let levels = self.get_levels(con.clone(), vec![], level);

            inter_cons.extend(levels);
        }

        inter_cons
    }

    fn get_levels(
        &self,
        node: String,
        mut list: Vec<String>,
        levels: u32,
    ) -> Vec<Vec<String>> {
        list.push(node.clone());

        if levels == 1 {
            return vec![list];
        }

        let next = self.connections.get(&node).unwrap();

        next.iter()
            .map(|n| self.get_levels(n.clone(), list.clone(), levels - 1))
            .flatten()
            .collect::<Vec<_>>()
    }

    fn filter_relevant(inter_cons: Vec<Vec<String>>) -> Vec<String> {
        let mut relevant: Vec<_> = inter_cons
            .into_iter()
            .filter(|l| l.first().unwrap() == l.last().unwrap())
            .filter(|l| l.iter().any(|n| n.starts_with('t')))
            .map(|l| {
                let mut nl = l.clone();
                nl.pop();
                nl.sort();

                nl.join(",")
            })
            .collect();

        relevant.sort();
        relevant.dedup();

        relevant
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let lan = Lan::from_string(input);

    let inter_cons = lan.get_inter_cons(4);
    let relevant_cons = Lan::filter_relevant(inter_cons);

    println!("{:#?}", relevant_cons);
    println!("{:#?}", relevant_cons.len());

    end_measure(mes);
}
