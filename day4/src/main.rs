use aoc_core::grid::Grid;
use aoc_core::read;
use aoc_core::spatial::{Direction, PointData};

struct XmasGrid {
    grid: Grid<char>,
}

impl XmasGrid {
    fn from_string(input: &str) -> Self {
        XmasGrid {
            grid: Grid::<char>::from_string(input),
        }
    }

    fn is_mas_x(&self, pos: &PointData<char>) -> bool {
        match (
            self.grid.move_to(&pos.point, &Direction::UpLeft),
            self.grid.move_to(&pos.point, &Direction::UpRight),
            self.grid
                .move_to(&pos.point, &Direction::DownLeft),
            self.grid
                .move_to(&pos.point, &Direction::DownRight),
        ) {
            (Some(ul), Some(ur), Some(ll), Some(lr)) => {
                (ul.value == &'M' && lr.value == &'S' || ul.value == &'S' && lr.value == &'M')
                    && (ur.value == &'M' && ll.value == &'S'
                        || ur.value == &'S' && ll.value == &'M')
            }
            _ => false,
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
W