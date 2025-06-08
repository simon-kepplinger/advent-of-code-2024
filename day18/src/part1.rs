use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc_core::{
    end_measure,
    grid::Grid,
    read,
    spatial::{Direction, Point},
    start_measure,
};

#[derive(Eq, PartialEq, Debug)]
pub struct Byte {
    pos: Point,
    g: i32,
    h: i32,
    f: i32,
}

impl Byte {
    fn new(pos: Point, g: i32, h: i32) -> Self {
        Byte {
            pos,
            g,
            h,
            f: g + h,
        }
    }
}

impl Ord for Byte {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f.cmp(&other.f)
    }
}

impl PartialOrd for Byte {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct MemoryPath {
    pub path: Vec<Point>,
    pub score: i32,
}

pub struct Memory {
    pub grid: Grid<char>,
    pub start: Point,
    pub end: Point,
    pub came_from: HashMap<Point, Point>,
    pub corruptions: Vec<Point>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        let grid = Grid::<char>::from_size(size + 1, size + 1);

        Memory {
            start: Point { x: 0, y: 0 },
            end: Point {
                x: size as i32,
                y: size as i32,
            },
            grid,
            came_from: HashMap::new(),
            corruptions: vec![],
        }
    }

    pub fn corrupt(&mut self, bytes: Vec<Point>) {
        self.corruptions = bytes;

        for byte in self.corruptions.iter() {
            self.grid.set(byte, '#');
        }
    }

    pub fn a_star(&mut self) -> Option<MemoryPath> {
        let mut open_set = BinaryHeap::<Reverse<Byte>>::new();
        let mut closed_set = HashSet::<Point>::new();
        let mut g_score = HashMap::<Point, i32>::new();

        let start_node = Byte::new(
            self.start,
            0,
            Memory::heuristic(&self.start, &self.end),
        );

        open_set.push(Reverse(start_node));

        while let Some(Reverse(current)) = open_set.pop() {
            if current.pos == self.end {
                return Some(MemoryPath {
                    path: self.reconstruct_path(current.pos),
                    score: current.g,
                });
            }

            closed_set.insert(current.pos);

            let neighbours = Direction::cardinal()
                .map(|d| current.pos.neighbour(&d))
                .into_iter()
                .filter(|p| !closed_set.contains(p))
                .filter(|p| self.grid.get(&p).is_some_and(|v| *v != '#'));

            for neighbour in neighbours {
                let tentative_g = current.g + 1;

                // abort if not the best
                if let Some(&best) = g_score.get(&neighbour) {
                    if tentative_g >= best {
                        continue;
                    }
                }

                let node = Byte::new(
                    neighbour,
                    tentative_g,
                    Memory::heuristic(&neighbour, &self.end),
                );

                g_score.insert(node.pos, node.g);
                self.came_from.insert(node.pos, current.pos);
                open_set.push(Reverse(node));
            }
        }

        None
    }

    fn heuristic(from: &Point, to: &Point) -> i32 {
        (from.x - to.x).abs() + (from.y - to.y).abs()
    }

    fn reconstruct_path(&self, from: Point) -> Vec<Point> {
        let mut path = Vec::new();
        let mut curr = from;

        path.push(curr);

        while let Some(prev) = self.came_from.get(&curr) {
            path.push(*prev);
            curr = *prev
        }

        path
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let bytes: Vec<_> = input
        .lines()
        .take(1024)
        .map(|l| l.split(',').collect())
        .map(|s: Vec<_>| Point {
            x: s[0].parse().unwrap(),
            y: s[1].parse().unwrap(),
        })
        .collect();

    let mut memory = Memory::new(70);
    memory.corrupt(bytes);

    println!("{}", memory.grid);

    let res = memory.a_star().unwrap();

    for (i, node) in res.path.iter().enumerate() {
        let i_str = i.to_string();
        memory
            .grid
            .set(node, i_str.chars().last().unwrap());
    }

    println!("{}", memory.grid);
    println!("Length {}", res.path.len() - 1);

    end_measure(mes);
}
