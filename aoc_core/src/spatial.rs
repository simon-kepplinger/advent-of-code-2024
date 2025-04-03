#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point<T = i32> {
    pub x: T,
    pub y: T,
}

impl Point {
    pub fn neighbour(&self, direction: &Direction) -> Point {
        self.add(&direction.as_point())
    }

    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PointData<'a, T> {
    pub value: &'a T,
    pub point: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct OwnedPointData<T> {
    pub value: T,
    pub point: Point,
}

impl<T: Copy> OwnedPointData<T> {
    pub fn from_point_data(point_data: PointData<T>) -> Self {
        OwnedPointData {
            value: *point_data.value,
            point: point_data.point,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirectionalPoint {
    pub point: Point,
    pub direction: Direction,
}

#[derive(Debug)]
pub struct Vector {
    pub from: Point,
    pub to: Point,
}

impl Vector {
    pub fn from_points(from: Point, to: Point) -> Vector {
        Vector { from, to }
    }

    pub fn direction(&self) -> Point {
        self.to.sub(&self.from)
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Direction {
    Up = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    DownLeft = 5,
    Left = 6,
    UpLeft = 7,
}

pub enum Rotation {
    Left = -1,
    Right = 1,
}

pub enum Angle {
    Deg45 = 1,
    Deg90 = 2,
}

impl Direction {
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

    pub fn cardinal() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }

    pub fn read_directions() -> [Direction; 4] {
        [
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
        ]
    }

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

    pub fn rotate(&self, rotation: Rotation, angle: Angle) -> Direction {
        let current = self.clone() as u8;

        let new_dir = match rotation {
            Rotation::Left => {
                (current + 8 - (rotation as u8 * angle as u8)) % 8
            }
            Rotation::Right => (current + (rotation as u8 * angle as u8)) % 8,
        };

        Direction::from_num(new_dir)
    }

    fn from_num(n: u8) -> Direction {
        match n {
            0 => Direction::Up,
            1 => Direction::UpRight,
            2 => Direction::Right,
            3 => Direction::DownRight,
            4 => Direction::Down,
            5 => Direction::DownLeft,
            6 => Direction::Left,
            7 => Direction::UpLeft,
            _ => panic!("Invalid direction number: {}", n),
        }
    }
}
