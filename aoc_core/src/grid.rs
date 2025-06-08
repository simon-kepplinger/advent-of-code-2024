use crate::spatial::{Direction, Point, PointData};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

pub struct GridIterator<'a, T> {
    pub grid: &'a Grid<T>,
    pub point: Point,
}

impl<T> Grid<T> {
    pub fn get(&self, point: &Point) -> Option<&T> {
        self.data
            .get(point.y as usize)?
            .get(point.x as usize)
    }

    pub fn set(&mut self, point: &Point, value: T) {
        self.data[point.y as usize][point.x as usize] = value;
    }

    pub fn length(&self) -> i32 {
        self.data[0].len() as i32
    }

    pub fn height(&self) -> i32 {
        self.data.len() as i32
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            point: Point { x: -1, y: 0 },
        }
    }

    pub fn is_within(&self, point: &Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.x < self.length()
            && point.y < self.height()
    }

    pub fn move_to(
        &self,
        point: &Point,
        direction: &Direction,
    ) -> Option<PointData<'_, T>> {
        let neighbour = point.neighbour(direction);

        self.get(&neighbour).map(|v| PointData {
            point: neighbour,
            value: v,
        })
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, value: T) -> Option<Point<i32>> {
        for pos in self.iter() {
            if *pos.value == value {
                return Some(pos.point);
            }
        }

        None
    }
}

impl Grid<char> {
    pub fn from_size(x_size: usize, y_size: usize) -> Self {
        let data: Vec<Vec<char>> = vec![vec!['.'; x_size]; y_size];

        Grid { data }
    }

    pub fn from_string(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        Grid { data: grid }
    }
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = PointData<'a, T>;

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

impl Grid<u8> {
    pub fn from_string(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        Grid { data: grid }
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
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
