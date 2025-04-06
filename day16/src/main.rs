use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc_core::{
    end_measure,
    grid::Grid,
    read,
    spatial::{Direction, DirectionalPoint, Point},
    start_measure,
};

#[derive(Eq, PartialEq, Debug)]
struct MazeNode {
    pos: DirectionalPoint,
    g: i32,
    h: i32,
    f: i32,
}

impl MazeNode {
    fn new(pos: DirectionalPoint, g: i32, h: i32) -> Self {
        MazeNode {
            pos,
            g,
            h,
            f: g + h,
        }
    }
}

impl Ord for MazeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f.cmp(&other.f)
    }
}

impl PartialOrd for MazeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct MazeResult {
    path: Vec<DirectionalPoint>,
    score: i32,
}

struct Maze {
    grid: Grid<char>,
    start: Point,
    end: Point,
    came_from: HashMap<DirectionalPoint, DirectionalPoint>,
}

impl Maze {
    fn from_string(input: &str) -> Self {
        let grid = Grid::<char>::from_string(input);
        Maze {
            start: grid.find('S').unwrap(),
            end: grid.find('E').unwrap(),
            grid,
            came_from: HashMap::new(),
        }
    }

    fn a_star(&mut self) -> Option<MazeResult> {
        let mut open_set = BinaryHeap::<Reverse<MazeNode>>::new();
        let mut closed_set = HashSet::<DirectionalPoint>::new();

        let start_point = DirectionalPoint::new(self.start, Direction::Right);

        let start_node = MazeNode::new(
            start_point,
            0,
            Maze::heuristic(&start_point, &self.end),
        );

        open_set.push(Reverse(start_node));

        while let Some(Reverse(current)) = open_set.pop() {
            if current.pos.point == self.end {
                return Some(MazeResult {
                    path: self.reconstruct_path(current.pos),
                    score: current.g,
                });
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

                let node = MazeNode::new(
                    neighbour,
                    current.g + 1 + rotation_score,
                    Maze::heuristic(&neighbour, &self.end),
                );

                self.came_from.insert(node.pos, current.pos);
                open_set.push(Reverse(node));
            }
        }

        None
    }

    fn heuristic(from: &DirectionalPoint, to: &Point) -> i32 {
        let manhattan =
            (from.point.x - to.x).abs() + (from.point.y - to.y).abs();

        let mut x_unaligned = false;
        let mut y_unaligned = false;

        let dx = from.point.x - to.x;
        let dy = from.point.y - to.y;

        if dx < 0 {
            x_unaligned = from.direction != Direction::Right;
        } else if dx > 0 {
            x_unaligned = from.direction != Direction::Left;
        }

        if dy < 0 {
            y_unaligned = from.direction != Direction::Up
        } else if dy > 0 {
            y_unaligned = from.direction != Direction::Down
        }

        manhattan + (x_unaligned as i32) * 1000 + (y_unaligned as i32) * 1000
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

fn main() {
    let mes = start_measure();
    let input = read("in/input");
    let mut maze = Maze::from_string(input.as_str());

    let res = maze.a_star().unwrap();
    let path_length = res.path.len();

    for point in res.path {
        maze.grid
            .set(&point.point, to_char(point.direction));
    }

    println!("{}", maze.grid);
    println!("Score {}", res.score);
    println!("Length {}", path_length);

    end_measure(mes);
}

fn to_char(direction: Direction) -> char {
    match direction {
        Direction::Up => '^',
        Direction::Right => '>',
        Direction::Down => 'V',
        Direction::Left => '<',
        _ => ' ',
    }
}
