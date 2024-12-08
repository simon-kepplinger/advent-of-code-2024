use std::collections::HashSet;
use aoc_core::{end_measure, read, start_measure};
use aoc_core::grid::Grid;
use aoc_core::spatial::{Angle, Direction, Point, Rotation};

#[derive(Debug)]
struct DirectionalPoint {
    point: Point,
    direction: Direction,
}

struct PatrolGrid {
    grid: Grid,
    visited: Vec<DirectionalPoint>,
}

impl PatrolGrid {
    fn from_string(input: &str) -> PatrolGrid {
        PatrolGrid {
            grid: Grid::from_string(input),
            visited: Vec::new(),
        }
    }

    fn get_distinct_visited(&self) -> HashSet<Point> {
        self.visited.iter()
            .map(|dp| dp.point.clone())
            .collect()
    }

    fn is_already_visited(&self, point: &Point, direction: &Direction) -> bool {
        self.visited.iter()
            .any(|p| { p.direction == *direction && p.point == *point })
    }

    fn patrol(&mut self,
              from: Point,
              direction: Direction) -> usize {
        let moved = self.grid.move_to(&from, &direction);

        match moved {
            Some(pos) => {
                if pos.value != &'#' {
                    if self.is_already_visited(&pos.point, &direction) {
                        return self.get_distinct_visited().len()
                    }

                    self.visited.push(
                        DirectionalPoint {
                            point: from.clone(),
                            direction: direction.clone()
                        }
                    );

                    return self.patrol(pos.point.clone(), direction.clone());
                }
            }
            None => {
                return self.get_distinct_visited().len();
            }
        }

        let new_direction = direction.rotate(Rotation::Right, Angle::Deg90);

        self.patrol(from, new_direction.clone())
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let mut patrol_grid = PatrolGrid::from_string(&input);

    let start = patrol_grid.grid
        .iter()
        .find(|p| p.value == &'^')
        .unwrap();

    let result = patrol_grid.patrol(start.point, Direction::Up);

    println!("{}", result + 1);

    end_measure(mes);
}
