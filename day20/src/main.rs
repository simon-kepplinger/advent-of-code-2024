use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    usize,
};

use aoc_core::{
    end_measure,
    grid::Grid,
    read,
    spatial::{Direction, Point},
    start_measure,
};

#[derive(Eq, PartialEq, Debug)]
pub struct Node {
    pos: Point,
    g: i32,
    h: i32,
    f: i32,
}

impl Node {
    fn new(pos: Point, g: i32, h: i32) -> Self {
        Node {
            pos,
            g,
            h,
            f: g + h,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f.cmp(&other.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct RacePath {
    pub path: VecDeque<Point>,
    pub score: i32,
}

pub struct RaceTrack {
    pub grid: Grid<char>,
    pub start: Point,
    pub end: Point,
    pub came_from: HashMap<Point, Point>,
}

impl RaceTrack {
    pub fn from_string(input: &str) -> Self {
        let grid = Grid::<char>::from_string(input);
        RaceTrack {
            start: grid.find('S').unwrap(),
            end: grid.find('E').unwrap(),
            grid,
            came_from: HashMap::new(),
        }
    }

    pub fn a_star(&mut self) -> Option<RacePath> {
        let mut open_set = BinaryHeap::<Reverse<Node>>::new();
        let mut closed_set = HashSet::<Point>::new();
        let mut g_score = HashMap::<Point, i32>::new();

        let start_node = Node::new(
            self.start,
            0,
            RaceTrack::heuristic(&self.start, &self.end),
        );

        open_set.push(Reverse(start_node));

        while let Some(Reverse(current)) = open_set.pop() {
            if current.pos == self.end {
                return Some(RacePath {
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

                let node = Node::new(
                    neighbour,
                    tentative_g,
                    RaceTrack::heuristic(&neighbour, &self.end),
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

    fn reconstruct_path(&mut self, from: Point) -> VecDeque<Point> {
        let mut path = VecDeque::new();
        let mut curr = from;

        path.push_back(curr);

        while let Some(prev) = self.came_from.get(&curr) {
            path.push_back(*prev);
            curr = *prev
        }

        path
    }
}

struct PathCheater {
    path: HashMap<Point, usize>,
}

impl PathCheater {
    pub fn from_vec(vec: VecDeque<Point>) -> Self {
        let mut map = HashMap::new();

        for (i, p) in vec.iter().enumerate() {
            map.insert(*p, i);
        }

        PathCheater { path: map }
    }

    fn find_cheats(&self, max: usize) -> Vec<usize> {
        self.path
            .iter()
            .map(|(point, i)| {
                self.points_within_range(point, 20)
                    .into_iter()
                    .filter(|s| s < i)
                    .map(|s| i - s)
                    .filter(|s| *s >= max)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }

    fn points_within_range(&self, origin: &Point, range: i32) -> Vec<usize> {
        let mut cheat_scores =
            Vec::with_capacity(((range + 1).pow(2) + range.pow(2)) as usize);

        for dx in -range..=range {
            let max_dy = range - dx.abs();
            for dy in -max_dy..=max_dy {
                let point = Point {
                    x: origin.x + dx,
                    y: origin.y + dy,
                };

                if self.path.contains_key(&point) {
                    let remaining = *self.path.get(&point).unwrap();
                    let cheat_time = dx.abs() + dy.abs();
                    cheat_scores.push(remaining + cheat_time as usize);
                }
            }
        }

        cheat_scores
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let mut race_track = RaceTrack::from_string(input.as_str());

    let res = race_track.a_star().unwrap();
    let cheater = PathCheater::from_vec(res.path.clone());

    for node in res.path {
        race_track.grid.set(&node, 'O');
    }

    println!("{}", race_track.grid);
    println!("{} picoseconds", res.score);

    let cheats = cheater.find_cheats(100);
    println!("{:?}", cheats);
    println!("{} amount", cheats.len());

    end_measure(mes);
}
