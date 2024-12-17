use aoc_core::grid::Grid;
use aoc_core::spatial::{Direction, Point, PointData};
use aoc_core::{end_measure, read, start_measure};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct PlantRegion {
    plant: char,
    points: HashSet<Point>,
    sides: HashMap<Direction, HashSet<Point>>,
}

impl PlantRegion {
    fn from_grid(grid: &Grid<char>) -> Vec<PlantRegion> {
        let mut regions: Vec<PlantRegion> = Vec::new();

        for pos in grid.iter() {
            if regions
                .iter()
                .all(|p| !PlantRegion::is_within(pos.point, p))
            {
                regions.push(PlantRegion::collect_region(pos, grid))
            }
        }

        regions
    }

    fn is_within(point: Point, other: &PlantRegion) -> bool {
        other.points.contains(&point)
    }

    fn collect_region(pos: PointData<char>, grid: &Grid<char>) -> PlantRegion {
        let mut region = PlantRegion {
            plant: *pos.value,
            points: HashSet::new(),
            sides: HashMap::new(),
        };

        region.points.insert(pos.point);
        region.walk_region(pos, grid);

        region
    }

    fn walk_region(&mut self, pos: PointData<char>, grid: &Grid<char>) {
        for dir in Direction::cardinal() {
            match grid.move_to(&pos.point, &dir) {
                Some(moved) => {
                    if moved.value == &self.plant && !self.points.contains(&moved.point) {
                        self.points.insert(moved.point);

                        self.walk_region(moved, grid);
                    }

                    if moved.value != &self.plant {
                        self.sides
                            .entry(dir)
                            .or_default()
                            .insert(pos.point);
                    }
                }
                None => {
                    self.sides
                        .entry(dir)
                        .or_default()
                        .insert(pos.point);
                }
            }
        }
    }

    fn get_side_count(&self) -> usize {
        Direction::cardinal()
            .map(|dir| self.get_side_count_for(&dir))
            .iter()
            .sum()
    }

    fn get_side_count_for(&self, dir: &Direction) -> usize {
        let mut sides: Vec<_> = self.sides.get(dir).unwrap().iter().collect();

        match dir {
            Direction::Left | Direction::Right => {
                sides.sort_by_key(|p| (p.x, p.y));

                sides
                    .chunk_by(|a, b| a.x == b.x && (a.y - 1 == b.y || a.y + 1 == b.y))
                    .count()
            }
            Direction::Up | Direction::Down => {
                sides.sort_by_key(|p| (p.y, p.x));

                sides
                    .chunk_by(|a, b| a.y == b.y && (a.x - 1 == b.x || a.x + 1 == b.x))
                    .count()
            }
            _ => 0,
        }
    }
}

struct GardenGrid {
    grid: Grid<char>,
    plants: Vec<PlantRegion>,
}

impl GardenGrid {
    fn from_string(input: &str) -> GardenGrid {
        let grid = Grid::<char>::from_string(input);
        let plants = PlantRegion::from_grid(&grid);

        GardenGrid { grid, plants }
    }

    fn get_fence_costs(&self) -> usize {
        self.plants
            .iter()
            .map(|p| p.get_side_count() * p.points.len())
            .sum()
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let garden = GardenGrid::from_string(&input);
    let costs = garden.get_fence_costs();

    println!("{}", costs);

    end_measure(mes);
}
