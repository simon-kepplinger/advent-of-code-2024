use std::fmt;
use std::fs;
use crate::Direction::{Down, DownLeft, DownRight, Left, Right, Up, UpLeft, UpRight};

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn neighbour(&self, direction: &Direction) -> Point {
        self.add(direction.as_point())
    }

    fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn as_point(&self) -> Point {
        match self {
            Up => Point { x: 0, y: -1 },
            UpRight => Point { x: 1, y: -1 },
            Right => Point { x: 1, y: 0 },
            DownRight => Point { x: 1, y: 1 },
            Down => Point { x: 0, y: 1 },
            DownLeft => Point { x: -1, y: 1 },
            Left => Point { x: -1, y: 0 },
            UpLeft => Point { x: -1, y: -1 },
        }
    }

    fn all() -> [Direction; 8] {
        [
            Up,
            UpRight,
            Right,
            DownRight,
            Down,
            DownLeft,
            Left,
            UpLeft,
        ]
    }

    fn read_directions() -> [Direction; 4] {
        [
            UpRight,
            Right,
            DownRight,
            Down
        ]
    }
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
}

struct GridIterator<'a> {
    grid: &'a Grid,
    point: Point,
}

#[derive(Debug)]
struct PointData<'a> {
    value: &'a char,
    point: Point,
}

// iterator for Grid
impl<'a> Iterator for GridIterator<'a> {
    type Item = PointData<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.point.y >= self.grid.height() {
            return None;
        }

        self.point.x += 1;

        if self.point.x >= self.grid.length() {
            self.point.x = 0;
            self.point.y += 1;
        }

        let value = self.grid.get(&self.point);

        match value {
            None => None,
            Some(v) => Some(PointData {
                value: v,
                point: self.point.clone(),
            }),
        }
    }
}

impl Grid {
    fn from_string(input: &str) -> Self {
        let grid = input.lines().map(|line| line.chars().collect()).collect();

        Grid { data: grid }
    }

    fn get(&self, point: &Point) -> Option<&char> {
        self.data.get(point.y as usize)?.get(point.x as usize)
    }

    fn length(&self) -> i32 {
        self.data[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.data.len() as i32
    }

    fn iter(&self) -> GridIterator {
        GridIterator {
            grid: self,
            point: Point { x: -1, y: 0 },
        }
    }

    fn move_to(&self, point: &Point,
               direction: &Direction) -> Option<PointData> {
        let neighbour = point.neighbour(direction);

        self.get(&neighbour)
            .map(|v| PointData {
                point: neighbour,
                value: v
            })
    }

    fn has_word(&self,
                pos: &PointData,
                remaining: String) -> Vec<Direction> {
        let mut actual: Vec<Direction> = vec![];

        for dir in Direction::read_directions() {
            let direction = self.has_word_in_direction(
                pos,
                dir.clone(),
                remaining.clone()
            );

            if direction.is_some() {
                actual.push(dir);
            }
        }

        actual
    }

    fn has_word_in_direction<'a>(&'a self,
                                 pos: &PointData,
                                 direction: Direction,
                                 remaining: String) -> Option<Direction> {
        if remaining.is_empty() {
            return Some(direction);
        }

        let new_pos = self.move_to(&pos.point, &direction);
        let expected_char = remaining.chars().next().unwrap();

        match new_pos {
            None => None,
            Some(pos) => {
                if pos.value == &expected_char {
                    self.has_word_in_direction(
                      &pos,
                      direction,
                      remaining.chars().skip(1).collect()
                    )
                } else {
                    None
                }
            }
        }
    }

    fn is_mas_x(&self, pos: &PointData) -> bool {
        match (
            self.move_to(&pos.point, &UpLeft),
            self.move_to(&pos.point, &UpRight),
            self.move_to(&pos.point, &DownLeft),
            self.move_to(&pos.point, &DownRight),
        ) {
            (Some(ul), Some(ur), Some(ll), Some(lr)) => {
                (
                    ul.value == &'M' && lr.value == &'S'
                    || ul.value == &'S' && lr.value == &'M'
                ) && (
                    ur.value == &'M' && ll.value == &'S'
                        || ur.value == &'S' && ll.value == &'M'
                )
            },
            _ => false
        }
    }
}

// formatter for Grid struct
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for item in row {
                write!(f, "{} ", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = read("in/input");
    let grid = Grid::from_string(&input);

    let mut count = 0;

    for pos in grid.iter() {
        if pos.value == &'A' && grid.is_mas_x(&pos) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn read(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("could not open file {}", path))
}
