#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
pub struct PointData<'a> {
    pub value: &'a char,
    pub point: Point,
}

#[derive(Debug, Clone)]
pub struct DirectionalPoint {
    pub point: Point,
    pub direction: Direction,
}

#[derive(Clone, Debug, PartialEq)]
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
    Deg90 = 2
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

    pub fn read_directions() -> [Direction; 4] {
        [
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down
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
            Rotation::Left => (current + 8 - (rotation as u8 * angle as u8)) % 8,
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