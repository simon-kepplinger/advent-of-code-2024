use std::{char, i32, vec};

use aoc_core::grid::Grid;
use aoc_core::spatial::{Direction, OwnedPointData, Point, PointData};
use aoc_core::tree::{TreeNode, TreeRoot};
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
    pub fn scale_up(line: &str) -> String {
        line.chars()
            .flat_map(|c| match c {
                '#' => "##".chars(),
                'O' => "[]".chars(),
                '.' => "..".chars(),
                '@' => "@.".chars(),
                '\n' => "\n".chars(),
                _ => "".chars(),
            })
            .collect()
    }

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
        let initial_robot = self.robot.clone();

        let mut robot_node = TreeNode::new(OwnedPointData {
            point: initial_robot,
            value: '@',
        });

        let movable_boxes = self.get_moveable_boxes(&initial_robot, &direction);

        // wall is hit somewhere
        if movable_boxes.is_none() {
            return;
        }

        robot_node.children = movable_boxes.unwrap();

        let tree = TreeRoot::new(Some(robot_node));

        self.robot = self.push_tree(&tree, &direction);

        return;
    }

    fn get_moveable_boxes(
        &self,
        from: &Point,
        direction: &Direction,
    ) -> Option<Vec<TreeNode<OwnedPointData<char>>>> {
        let pointer = self.map.move_to(&from, direction).unwrap();
        let mut box_parts: Vec<_> = vec![];
        let mut nodes: Vec<_> = vec![];

        if *pointer.value == '#' {
            return None;
        }

        if *pointer.value == '[' || *pointer.value == ']' {
            if *direction == Direction::Up || *direction == Direction::Down {
                box_parts = RobotGrid::get_box_parts(pointer);
            } else {
                box_parts.push(pointer);
            }
        }

        for box_part in box_parts {
            let owned_data = OwnedPointData::from_point_data(box_part);
            let mut node = TreeNode::new(owned_data);

            let moveable_boxes =
                self.get_moveable_boxes(&box_part.point, direction);

            match moveable_boxes {
                Some(children) => {
                    node.children = children;
                    nodes.push(node);
                }
                None => return None,
            }
        }

        Some(nodes)
    }

    fn push_tree(
        &mut self,
        tree: &TreeRoot<OwnedPointData<char>>,
        direction: &Direction,
    ) -> Point<i32> {
        let mut pushed_root = tree.root_node.as_ref().unwrap().value.point;

        for node in tree.iter_breadth_rev() {
            let moved_node = self
                .map
                .move_to(&node.value.point, direction)
                .unwrap();

            pushed_root = moved_node.point;

            self.map.set(&moved_node.point, node.value.value);
            self.map.set(&node.value.point, '.');
        }

        pushed_root
    }

    fn get_box_parts(pointer: PointData<char>) -> Vec<PointData<char>> {
        if *pointer.value == '[' {
            let box_end = PointData {
                value: &']',
                point: Point {
                    x: pointer.point.x + 1,
                    y: pointer.point.y,
                },
            };

            return vec![pointer, box_end];
        }

        if *pointer.value == ']' {
            let box_start = PointData {
                value: &'[',
                point: Point {
                    x: pointer.point.x - 1,
                    y: pointer.point.y,
                },
            };

            return vec![box_start, pointer];
        }

        vec![]
    }

    fn get_gps_coords(&self) -> i32 {
        self.map
            .iter()
            .filter(|p| *p.value == '[')
            .map(|b| b.point.x + b.point.y * 100)
            .sum()
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let split: Vec<_> = input.split("\n\n").collect();
    let map = RobotGrid::scale_up(split.get(0).unwrap());
    let moves = split.get(1).unwrap().trim();

    let inner_grid = Grid::<char>::from_string(map.as_str());
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
