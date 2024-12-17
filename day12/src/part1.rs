use aoc_core::grid::Grid;
use aoc_core::spatial::{Direction, Point, PointData};
use aoc_core::{end_measure, read, start_measure};
use std::collections::HashSet;

#[derive(Debug)]
struct PlantRegion {
    plant: char,
    points: HashSet<Point>,
}

impl PlantRegion {
    fn from_grid(grid: &Grid<char>) -> Vec<PlantRegion> {
        let mut regions: Vec<PlantRegion> = Vec::new();

        for pos in grid.iter() {
            if regions
                .iter()
                .all(|p| !PlantRegion::is_within(pos.point, p))
            {
                regions.push(PlantRegion::collect_region(pos, &grid))
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
        };

        region.points.insert(pos.point);
        region.walk_region(pos, grid);

        region
    }

    fn walk_region(&mut self, pos: PointData<char>, grid: &Grid<char>) {
        for dir in Direction::cardinal() {
            if let Some(moved) = grid.move_to(&pos.point, &dir) {
                if moved.value == &self.plant && !self.points.contains(&moved.point) {
                    self.points.insert(moved.point);

                    self.walk_region(moved, grid);
                }
            }
        }
    }

    fn get_perimeter(&self, grid: &Grid<char>) -> u32 {
        let mut sum = 0;

        for point in self.points.iter() {
            for dir in Direction::cardinal() {
                match grid.move_to(point, &dir) {
                    None => sum += 1,
                    Some(pos) => {
                        if pos.value != &self.plant {
                            sum += 1;
                        }
                    }
                }
            }
        }

        sum
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

    fn get_fence_costs(&self) -> u32 {
        self.plants
            .iter()
            .map(|p| p.get_perimeter(&self.grid) * p.points.len() as u32)
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
