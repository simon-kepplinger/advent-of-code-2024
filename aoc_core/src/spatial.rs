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
pub struct PointData<'a> {
    pub value: &'a char,
    pub point: Point,
}