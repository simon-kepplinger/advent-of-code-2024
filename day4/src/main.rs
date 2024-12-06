use aoc_core::{read, Direction, Grid, PointData};
use aoc_core::Direction::{DownLeft, DownRight, UpLeft, UpRight};

struct XmasGrid {
    grid: Grid
}

impl XmasGrid {

    fn from_string(input: &str) -> Self {
        XmasGrid {
            grid: Grid::from_string(input)
        }
    }

    fn has_word(&self,
                pos: &PointData,
                remaining: String) -> Vec<Direction> {
        let mut actual: Vec<Direction> = vec![];

        for dir in Direction::read_directions() {
            let direction = self.has_word_in_direction(
                pos,
                dir.clone(),
                remaining.clone()
            );

            if direction.is_some() {
                actual.push(dir);
            }
        }

        actual
    }

    fn has_word_in_direction<'a>(&'a self,
                                 pos: &PointData,
                                 direction: Direction,
                                 remaining: String) -> Option<Direction> {
        if remaining.is_empty() {
            return Some(direction);
        }

        let new_pos = self.grid.move_to(&pos.point, &direction);
        let expected_char = remaining.chars().next().unwrap();

        match new_pos {
            None => None,
            Some(pos) => {
                if pos.value == &expected_char {
                    self.has_word_in_direction(
                      &pos,
                      direction,
                      remaining.chars().skip(1).collect()
                    )
                } else {
                    None
                }
            }
        }
    }

    fn is_mas_x(&self, pos: &PointData) -> bool {
        match (
            self.grid.move_to(&pos.point, &UpLeft),
            self.grid.move_to(&pos.point, &UpRight),
            self.grid.move_to(&pos.point, &DownLeft),
            self.grid.move_to(&pos.point, &DownRight),
        ) {
            (Some(ul), Some(ur), Some(ll), Some(lr)) => {
                (
                    ul.value == &'M' && lr.value == &'S'
                    || ul.value == &'S' && lr.value == &'M'
                ) && (
                    ur.value == &'M' && ll.value == &'S'
                        || ur.value == &'S' && ll.value == &'M'
                )
            },
            _ => false
        }
    }
}

fn main() {
    let input = read("in/input");
    let xmas_grid = XmasGrid::from_string(&input);

    let mut count = 0;

    for pos in xmas_grid.grid.iter() {
        if pos.value == &'A' && xmas_grid.is_mas_x(&pos) {
            count += 1;
        }
    }

    println!("{}", count);
}