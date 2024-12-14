use aoc_core::grid::Grid;
use aoc_core::spatial::{Angle, Direction, DirectionalPoint, Point, PointData, Rotation};
use aoc_core::{end_measure, read, start_measure};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum PatrolResult {
    Loop,
    Exit,
}

#[derive(Clone)]
struct PatrolGrid {
    grid: Grid<char>,
    visited: Vec<DirectionalPoint>,
    size: i32,
}

impl PatrolGrid {
    fn from_string(input: &str) -> PatrolGrid {
        let grid = Grid::<char>::from_string(input);
        let size = grid.height() * grid.length();

        PatrolGrid {
            grid: Grid::<char>::from_string(input),
            visited: Vec::new(),
            size,
        }
    }

    fn get_distinct_visited(&self) -> HashSet<Point> {
        self.visited
            .iter()
            .map(|dp| dp.point.clone())
            .collect()
    }

    fn is_already_visited(&self, point: &Point, direction: &Direction) -> bool {
        self.visited
            .iter()
            .any(|p| p.direction == *direction && p.point == *point)
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

    fn get_start(&self) -> PointData<char> {
        self.grid
            .iter()
            .find(|p| p.value == &'^')
            .unwrap()
    }

    fn patrol_fast(&self, from: &Point, direction: &Direction, count: i32) -> PatrolResult {
        let moved = self.grid.move_to(from, direction);

        match moved {
            Some(pos) => {
                if pos.value != &'#' {
                    if count > 2 * self.size {
                        return PatrolResult::Loop;
                    }

                    self.patrol_fast(&pos.point, direction, count + 1)
                } else {
                    let new_direction = direction.rotate(Rotation::Right, Angle::Deg90);

                    self.patrol_fast(from, &new_direction, count + 1)
                }
            }
            None => PatrolResult::Exit,
        }
    }

    fn patrol(&mut self, from: Point, direction: Direction) -> PatrolResult {
        let moved = self.grid.move_to(&from, &direction);

        match moved {
            Some(pos) => {
                if pos.value != &'#' {
                    if self.is_already_visited(&pos.point, &direction) {
                        return PatrolResult::Loop;
                    }

                    self.visited.push(DirectionalPoint {
                        point: from.clone(),
                        direction: direction.clone(),
                    });

                    self.patrol(pos.point.clone(), direction.clone())
                } else {
                    let new_direction = direction.rotate(Rotation::Right, Angle::Deg90);

                    self.patrol(from, new_direction.clone())
                }
            }
            None => {
                self.visited.push(DirectionalPoint {
                    point: from.clone(),
                    direction: direction.clone(),
                });

                PatrolResult::Exit
            }
        }
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let mut patrol = PatrolGrid::from_string(&input);
    let start = patrol.get_start();

    patrol.patrol(start.point, Direction::Up);
    let initial_patrol = patrol.get_distinct_visited().clone();
    let mut results: Vec<PatrolResult> = vec![];

    let mut simple_patrol = PatrolGrid::from_string(&input);

    for to_block in initial_patrol.into_iter() {
        simple_patrol.block_position(&to_block);
        let start = patrol.get_start();

        let result = simple_patrol.patrol_fast(&start.point, &Direction::Up, 0);
        results.push(result);
        simple_patrol.unblock_position(&to_block);
    }

    let loop_count = results
        .into_iter()
        .filter(|r| r == &PatrolResult::Loop)
        .count();

    println!("{:?}", loop_count);

    end_measure(mes);
}
