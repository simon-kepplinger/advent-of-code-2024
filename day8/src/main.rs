use aoc_core::grid::Grid;
use aoc_core::read;
use aoc_core::spatial::{Point, Vector};
use std::collections::{HashMap, HashSet};

struct City {
    grid: Grid,
    antennas: HashMap<char, Vec<Point>>,
}

#[derive(Debug)]
struct AntennaPair {
    vector: Vector,
}

impl AntennaPair {
    fn from_points(from: Point, to: Point) -> AntennaPair {
        AntennaPair {
            vector: Vector { from, to },
        }
    }

    fn get_antinodes(&self, grid: &Grid) -> Vec<Point> {
        let dir = self.vector.direction();

        let mut nodes: Vec<Point> = vec![self.vector.from.clone()];
        let mut is_within = true;

        while is_within {
            let new = nodes.last().unwrap().sub(&dir);

            if grid.is_within(&new) {
                nodes.push(new.clone());
            } else {
                is_within = false;
            }
        }

        nodes.push(self.vector.to.clone());
        let mut is_within = true;

        while is_within {
            let new = nodes.last().unwrap().add(&dir);

            if grid.is_within(&new) {
                nodes.push(new.clone());
            } else {
                is_within = false;
            }
        }

        nodes
    }
}

impl City {
    fn from_string(input: &str) -> City {
        let grid = Grid::from_string(input);
        let mut antennas = HashMap::new();

        for pos in grid.iter().filter(|pos| pos.value != &'.') {
            antennas
                .entry(*pos.value)
                .or_insert_with(Vec::new)
                .push(pos.point.clone());
        }

        City { grid, antennas }
    }

    fn antenna_pairs(&self) -> Vec<AntennaPair> {
        self.antennas
            .iter()
            .flat_map(|(_, v)| {
                let mut pairs: Vec<AntennaPair> = vec![];

                for i in 0..v.len() {
                    for j in i + 1..v.len() {
                        pairs.push(AntennaPair::from_points(
                            v[i].clone(),
                            v[j].clone(),
                        ));
                    }
                }

                pairs
            })
            .collect()
    }
}

fn main() {
    let input = read("in/input");
    let city = City::from_string(&input);

    let antenna_pairs = city.antenna_pairs();

    let antinodes: HashSet<_> = antenna_pairs
        .iter()
        .flat_map(|ap| ap.get_antinodes(&city.grid))
        .filter(|p| city.grid.is_within(p))
        .collect();

    println!("{}", antinodes.len());
}
