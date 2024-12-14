use aoc_core::grid::Grid;
use aoc_core::spatial::{Direction, Point, PointData};
use aoc_core::{end_measure, read, start_measure};
use std::collections::HashSet;

const WALK_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

struct MapGrid {
    grid: Grid<u8>,
}

impl<'a> MapGrid {
    pub fn from_str(input: &str) -> MapGrid {
        MapGrid {
            grid: Grid::<u8>::from_string(input),
        }
    }

    fn walk_trailheads(&'a self) -> usize {
        let mut trailends = vec![];

        for pos in self.grid.iter() {
            if pos.value == &0 {
                let mut inner: HashSet<Point> = HashSet::new();
                self.walk(pos, &mut inner);

                trailends.push(inner);
            }
        }

        trailends.iter().map(|t| t.len()).sum()
    }

    fn walk(&'a self, origin: PointData<'a, u8>, trailend: &mut HashSet<Point>) {
        if origin.value == &9 {
            trailend.insert(origin.point);
            return;
        }

        for dir in WALK_DIRECTIONS {
            let neighbour = self.grid.move_to(&origin.point, &dir);

            match neighbour {
                None => {}
                Some(n) => {
                    if origin.value + 1 == *n.value {
                        self.walk(n, trailend);
                    }
                }
            }
        }
    }
}

fn main() {
    let mes = start_measure();

    let input = read("in/input");
    let map = MapGrid::from_str(&input);

    let sum = map.walk_trailheads();

    println!("{}", sum);

    end_measure(mes);
}
