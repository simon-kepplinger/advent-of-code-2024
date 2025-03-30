use std::i32;

use aoc_core::grid::Grid;
use aoc_core::spatial::{Direction, Point, PointData};
use aoc_core::{end_measure, read, start_measure};

fn move_to_direction(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::Up),
        '>' => Some(Direction::Right),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        _ => None,
    }
}

struct RobotGrid {
    map: Grid<char>,
    moves: String,
    robot: Point<i32>,
}

impl RobotGrid {
    fn walk_all(&mut self) {
        while self.moves.len() > 0 {
            self.walk();
        }
    }

    fn walk(&mut self) {
        let move_str = self.moves.remove(0);

        if move_to_direction(move_str).is_none() {
            return;
        }

        let direction = move_to_direction(move_str).unwrap();

        let mut move_buffer: Vec<Point<i32>> = vec![];
        let mut pointer_point = self.robot;

        loop {
            let pointer = self
                .map
                .move_to(&pointer_point, &direction)
                .unwrap();

            pointer_point = pointer.point;

            if *pointer.value == '#' {
                return;
            }

            if *pointer.value == 'O' {
                move_buffer.push(pointer.point);
            }

            if *pointer.value == '.' {
                let old_robot = self.robot.clone();

                // set robot to new pos
                if move_buffer.len() > 0 {
                    self.push(&pointer.point, &move_buffer);
                    self.map.set(&move_buffer[0], '@');
                    self.robot = move_buffer[0];
                } else {
                    self.robot = pointer.point;
                    self.map.set(&pointer.point, '@');
                }

                self.map.set(&old_robot, '.');

                return;
            }
        }
    }

    fn push(&mut self, to: &Point, points: &Vec<Point<i32>>) {
        let mut last: &Point = &to;

        for point in points.iter().rev() {
            self.map.set(last, 'O');

            last = point;
        }
    }

    fn get_gps_coords(&self) -> i32 {
        self.map
            .iter()
            .filter(|p| *p.value == 'O')
            .map(|b| b.point.x + b.point.y * 100)
            .sum()
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let split: Vec<_> = input.split("\n\n").collect();
    let map = split.get(0).unwrap();
    let moves = split.get(1).unwrap().trim();

    let inner_grid = Grid::<char>::from_string(map);
    let robot = inner_grid
        .iter()
        .find(|p| *p.value == '@')
        .unwrap();

    let mut grid = RobotGrid {
        map: inner_grid.clone(),
        moves: moves.to_string(),
        robot: robot.point,
    };

    println!("{}", grid.map);

    grid.walk_all();

    println!("{}", grid.map);

    println!("{}", grid.get_gps_coords());

    end_measure(mes);
}
