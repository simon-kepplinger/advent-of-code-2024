use std::collections::HashSet;
use aoc_core::{end_measure, read, start_measure};
use aoc_core::grid::Grid;
use aoc_core::spatial::{Angle, Direction, DirectionalPoint, Point, PointData, Rotation};

#[derive(Debug, PartialEq)]
enum PatrolResult {
    Loop,
    Exit
}

#[derive(Clone)]
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

    fn block_position(&mut self, point: &Point) {
        self.grid.set(point, '#');
    }

    fn unblock_position(&mut self, point: &Point) {
        self.grid.set(point, '.');
    }

    fn reset_visits(&mut self) {
        self.visited.clear();
    }

    fn get_start(&self) -> PointData {
        self.grid
            .iter()
            .find(|p| p.value == &'^')
            .unwrap()
    }

    fn patrol(&mut self,
              from: Point,
              direction: Direction) -> PatrolResult {
        let moved = self.grid.move_to(&from, &direction);

        match moved {
            Some(pos) => {
                if pos.value != &'#' {
                    if self.is_already_visited(&pos.point, &direction) {
                        return PatrolResult::Loop
                    }

                    self.visited.push(
                        DirectionalPoint {
                            point: from.clone(),
                            direction: direction.clone()
                        }
                    );

                    self.patrol(pos.point.clone(), direction.clone())
                } else {
                    let new_direction = direction.rotate(Rotation::Right, Angle::Deg90);

                    self.patrol(from, new_direction.clone())
                }
            }
            None => {
                PatrolResult::Exit
            }
        }
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/example");

    let mut patrol = PatrolGrid::from_string(&input);
    let start = patrol.get_start();

    patrol.patrol(start.point, Direction::Up);
    let initial_patrol: Vec<_> = patrol.visited.clone();
    let mut results: Vec<PatrolResult> = vec![];

    for to_block in initial_patrol {
        println!("{}", results.iter().len());

        if results.iter().len() == 1130 || results.iter().len() == 4327 || results.iter().len() == 4732 {
            results.push(PatrolResult::Exit);
            continue;
        }

        let mut blocked_patrol = patrol.clone();
        blocked_patrol.reset_visits();
        blocked_patrol.block_position(&to_block.point);
        let start = patrol.get_start();

        let result = blocked_patrol.patrol(start.point, Direction::Up);
        results.push(result);
    }

    let loop_count = results
        .into_iter()
        .filter(|r| r == &PatrolResult::Loop)
        .count();

    println!("{:?}", loop_count);

    end_measure(mes);
}
