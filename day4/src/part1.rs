use aoc_core::grid::Grid;
use aoc_core::read;
use aoc_core::spatial::{Direction, PointData};

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
}

fn main() {
    let input = read("in/input");
    let xmas_grid = XmasGrid::from_string(&input);

    let mut count = 0;

    for pos in xmas_grid.grid.iter() {
        let mut directions: Vec<Direction> = vec![];

        if pos.value == &'X' {
            directions = xmas_grid.has_word(&pos, String::from("MAS"));
        } else if pos.value == &'S' {
            directions = xmas_grid.has_word(&pos, String::from("AMX"));
        }

        count += directions.len();
    }

    println!("{}", count);
}