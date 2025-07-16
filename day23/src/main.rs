use std::collections::{HashMap, HashSet};

use aoc_core::{end_measure, read, start_measure};

#[derive(Debug)]
struct Lan<'a> {
    connections: Vec<(&'a str, &'a str)>,
    con_map: HashMap<String, HashSet<String>>,
}

impl<'a> Lan<'a> {
    fn from_string(input: &'a str) -> Self {
        let connections: Vec<_> = input
            .lines()
            .map(|l| l.split_once('-').unwrap())
            .collect();

        let mut con_map: HashMap<String, HashSet<String>> =
            HashMap::with_capacity(connections.len() * 2);

        for con in connections.clone() {
            con_map
                .entry(con.0.to_string())
                .or_default()
                .insert(con.1.to_string());

            con_map
                .entry(con.1.to_string())
                .or_default()
                .insert(con.0.to_string());
        }

        Lan {
            connections,
            con_map,
        }
    }

    fn build_lan(&self) -> Vec<HashSet<String>> {
        let mut lan: Vec<HashSet<String>> = vec![];

        for (a, b) in &self.connections {
            let mut was_found = 0;

            for network in &mut lan {
                let a_cons = self.con_map.get(*a).unwrap();
                let b_cons = self.con_map.get(*b).unwrap();

                if network.iter().all(|n| a_cons.contains(n)) {
                    network.insert(a.to_string());
                    was_found += 1;
                }

                if network.iter().all(|n| b_cons.contains(n)) {
                    network.insert(b.to_string());
                    was_found += 1;
                }
            }

            if was_found < 2 {
                let mut network = HashSet::new();
                network.insert(a.to_string());
                network.insert(b.to_string());

                lan.push(network);
            }
        }

        lan
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let lan = Lan::from_string(&input);
    let lan_nodes = lan.build_lan();

    let mut largest: Vec<_> = lan_nodes
        .iter()
        .max_by_key(|n| n.len())
        .unwrap()
        .iter()
        .map(|n| n.to_string())
        .collect();

    largest.sort();

    let password = largest.join(",");

    println!("{password}");

    end_measure(mes);
}
