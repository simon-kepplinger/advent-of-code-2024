use std::{fmt, fs};

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn neighbour(&self, direction: &Direction) -> Point {
        self.add(direction.as_point())
    }

    pub fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Direction {
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
    pub fn as_point(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::UpRight => Point { x: 1, y: -1 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::DownRight => Point { x: 1, y: 1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::DownLeft => Point { x: -1, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::UpLeft => Point { x: -1, y: -1 },
        }
    }

    pub fn all() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ]
    }

    pub fn read_directions() -> [Direction; 4] {
        [
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down
        ]
    }
}

#[derive(Debug)]
pub struct Grid {
    pub data: Vec<Vec<char>>,
}

pub struct GridIterator<'a> {
    pub grid: &'a Grid,
    pub point: Point,
}

#[derive(Debug)]
pub struct PointData<'a> {
    pub value: &'a char,
    pub point: Point,
}

impl Grid {
    pub fn from_string(input: &str) -> Self {
        let grid = input.lines().map(|line| line.chars().collect()).collect();

        Grid { data: grid }
    }

    pub fn get(&self, point: &Point) -> Option<&char> {
        self.data.get(point.y as usize)?.get(point.x as usize)
    }

    pub fn length(&self) -> i32 {
        self.data[0].len() as i32
    }

    pub fn height(&self) -> i32 {
        self.data.len() as i32
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator {
            grid: self,
            point: Point { x: -1, y: 0 },
        }
    }

    pub fn move_to(&self, point: &Point,
               direction: &Direction) -> Option<PointData> {
        let neighbour = point.neighbour(direction);

        self.get(&neighbour)
            .map(|v| PointData {
                point: neighbour,
                value: v
            })
    }
}

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

pub fn read(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("could not open file {}", path))
}