use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use aoc_core::{
    end_measure,
    grid::Grid,
    read,
    spatial::{Angle, Direction, DirectionalPoint, Point, Rotation},
    start_measure,
};

#[derive(Eq, PartialEq, Debug)]
pub struct MazeNode {
    pos: DirectionalPoint,
    g: i32,
}

impl MazeNode {
    fn new(pos: DirectionalPoint, g: i32) -> Self {
        MazeNode { pos, g }
    }
}

impl Ord for MazeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.g.cmp(&other.g)
    }
}

impl PartialOrd for MazeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct MazeResult {
    pub path: Vec<DirectionalPoint>,
    pub score: i32,
}

pub struct Maze {
    pub grid: Grid<char>,
    pub start: Point,
    pub end: Point,
    pub came_from: HashMap<DirectionalPoint, DirectionalPoint>,
    pub seen: HashMap<DirectionalPoint, i32>,
}

impl Maze {
    pub fn from_string(input: &str) -> Self {
        let grid = Grid::<char>::from_string(input);
        Maze {
            start: grid.find('S').unwrap(),
            end: grid.find('E').unwrap(),
            grid,
            came_from: HashMap::new(),
            seen: HashMap::new(),
        }
    }

    pub fn dijkstra(&mut self) -> Option<MazeResult> {
        let mut res = None;

        let mut open_set = BinaryHeap::<Reverse<MazeNode>>::new();
        let mut closed_set = HashSet::<DirectionalPoint>::new();

        let start_point = DirectionalPoint::new(self.start, Direction::Right);
        let start_node = MazeNode::new(start_point, 0);

        open_set.push(Reverse(start_node));

        while let Some(Reverse(current)) = open_set.pop() {
            let not_seen_already = !self.seen.contains_key(&current.pos)
                || self.seen.get(&current.pos).unwrap() > &current.g;

            if not_seen_already {
                self.seen.insert(current.pos, current.g);
            }

            if current.pos.point == self.end {
                res = Some(MazeResult {
                    path: self.reconstruct_path(current.pos),
                    score: current.g,
                });

                continue;
            }

            if res.as_ref().is_some_and(|r| r.score <= current.g) {
                continue;
            }

            closed_set.insert(current.pos);

            let neighbours = Direction::cardinal()
                .map(|d| current.pos.neighbour(&d))
                .into_iter()
                .filter(|p| !closed_set.contains(p))
                .filter(|p| self.grid.get(&p.point).is_some_and(|v| *v != '#'));

            for neighbour in neighbours {
                let rotation_score = Maze::get_rotation_score(
                    current.pos.direction,
                    neighbour.direction,
                );

                let node =
                    MazeNode::new(neighbour, current.g + 1 + rotation_score);

                self.came_from.insert(node.pos, current.pos);
                open_set.push(Reverse(node));
            }
        }

        res
    }

    fn get_rotation_score(from: Direction, to: Direction) -> i32 {
        let left_rotation = (from as i32 - to as i32).abs() / 2;
        let right_rotation = 4 - left_rotation;

        left_rotation.min(right_rotation) * 1000
    }

    fn reconstruct_path(
        &self,
        from: DirectionalPoint,
    ) -> Vec<DirectionalPoint> {
        let mut path = Vec::new();
        let mut curr = from;

        path.push(curr);

        while let Some(prev) = self.came_from.get(&curr) {
            path.push(*prev);
            curr = *prev
        }

        path.push(curr);

        path
    }
}

fn reverse_all_paths(maze: &mut Maze, end: Point) -> HashSet<Point> {
    let mut path = HashSet::new();

    // init with all possible ends
    let mut open: VecDeque<_> = Direction::cardinal()
        .map(|d| DirectionalPoint::new(end, d))
        .into_iter()
        .filter(|p| maze.seen.contains_key(p))
        .collect();

    while let Some(current) = open.pop_front() {
        let current_score = &maze.seen.get(&current).unwrap().clone();

        println!("");
        println!("----------------");
        println!(
            "CURRENT {} {} {} [{}]",
            current.point.x,
            current.point.y,
            to_char(current.direction),
            current_score
        );

        let neighbours: Vec<_> = Direction::cardinal()
            .map(|d| current.neighbour(&d))
            .into_iter()
            .map(|mut d| {
                d.direction = d.direction.rotate(Rotation::Left, Angle::Deg180);
                d
            })
            .filter(|p| maze.grid.get(&p.point).is_some_and(|v| *v != '#'))
            .collect();

        println!("");

        for neighbour in &neighbours {
            let rotation_score = Maze::get_rotation_score(
                current.direction,
                neighbour.direction,
            );

            println!(
                "neighbour {} {} {} [{}]",
                neighbour.point.x,
                neighbour.point.y,
                to_char(neighbour.direction),
                *current_score - 1 - rotation_score,
            );
        }

        println!("");

        for neighbour in neighbours {
            let variants: Vec<_> = Direction::cardinal()
                .map(|d| DirectionalPoint::new(neighbour.point, d))
                .into_iter()
                .collect();

            for variant in variants {
                if let Some(seen_score) = maze.seen.get(&variant) {
                    let rotation_score = Maze::get_rotation_score(
                        current.direction,
                        variant.direction,
                    );

                    print!(
                        "seen neighbour {} {} {} [{} == {}]",
                        variant.point.x,
                        variant.point.y,
                        to_char(variant.direction),
                        *current_score - 1 - rotation_score,
                        seen_score
                    );

                    if *seen_score == *current_score - 1 - rotation_score {
                        path.insert(variant.point);
                        open.push_back(variant);
                        print!(" push");
                    }

                    println!();
                }
            }
        }

        maze.seen.insert(current, i32::MAX);
    }

    path
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let mut maze = Maze::from_string(input.as_str());
    let res = maze.dijkstra().unwrap();

    let end = res.path[0];

    println!("");
    println!("Path");
    println!("X | Y | Dir");
    println!("-----------");

    for point in res.path {
        let dir = to_char(point.direction);
        println!("{} | {} | {}", point.point.x, point.point.y, dir);

        maze.grid.set(&point.point, dir);
    }

    println!("");
    println!("{}", maze.grid);

    println!("Seen");
    println!("X | Y | Dir | Score");
    println!("-----------");

    println!("");
    println!("Score {}", res.score);

    let all_paths = reverse_all_paths(&mut maze, end.point);
    maze.grid.set(&end.point, 'O');

    for point in &all_paths {
        maze.grid.set(point, 'O');
    }

    println!("");
    println!("{}", maze.grid);
    println!("");
    println!("Score {}", res.score);
    println!("all paths length {}", all_paths.len() + 1);

    end_measure(mes);
}

pub fn to_char(direction: Direction) -> char {
    match direction {
        Direction::Up => '^',
        Direction::Right => '>',
        Direction::Down => 'V',
        Direction::Left => '<',
        _ => ' ',
    }
}
