use crate::spatial::{Direction, Point, PointData};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Grid {
    pub data: Vec<Vec<char>>,
}

pub struct GridIterator<'a> {
    pub grid: &'a Grid,
    pub point: Point,
}

impl Grid {
    pub fn from_string(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        Grid { data: grid }
    }

    pub fn get(&self, point: &Point) -> Option<&char> {
        self.data
            .get(point.y as usize)?
            .get(point.x as usize)
    }

    pub fn set(&mut self, point: &Point, value: char) {
        self.data[point.y as usize][point.x as usize] = value;
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

    pub fn is_within(&self, point: &Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.length() && point.y < self.height()
    }

    pub fn move_to(&self, point: &Point, direction: &Direction) -> Option<PointData> {
        let neighbour = point.neighbour(direction);

        self.get(&neighbour).map(|v| PointData {
            point: neighbour,
            value: v,
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
