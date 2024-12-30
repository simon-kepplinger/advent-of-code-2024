use aoc_core::spatial::Point;
use aoc_core::{end_measure, read, start_measure};
use std::collections::HashSet;
use std::fmt;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
struct Robot {
    pos: Point,
    v: Point,
}

#[derive(Debug)]
struct BathroomLobby {
    width: u32,
    height: u32,
    robots: Vec<Robot>,
}

impl BathroomLobby {
    fn from_string(width: u32, height: u32, input: &str) -> Self {
        let robots: Vec<_> = input
            .lines()
            .map(|l| {
                l.replace("p=", "")
                    .split(" v=")
                    .map(|s| {
                        s.split(',')
                            .map(|p| p.parse::<i32>().unwrap())
                            .collect()
                    })
                    .map(|p: Vec<_>| Point { x: p[0], y: p[1] })
                    .collect()
            })
            .map(|p: Vec<_>| Robot { pos: p[0], v: p[1] })
            .collect();

        BathroomLobby {
            width,
            height,
            robots,
        }
    }

    fn pass_second(&mut self) {
        for robot in &mut self.robots {
            robot.pos.x = (robot.pos.x + robot.v.x) % self.width as i32;
            robot.pos.y = (robot.pos.y + robot.v.y) % self.height as i32;

            if robot.pos.x < 0 {
                robot.pos.x += self.width as i32;
            }

            if robot.pos.y < 0 {
                robot.pos.y += self.height as i32;
            }
        }
    }

    fn count_in_quadrants(&self) -> usize {
        let x_center = self.width.overflowing_div(2).0 as i32;
        let y_center = self.height.overflowing_div(2).0 as i32;

        self.robots
            .iter()
            .filter(|r| r.pos.x < x_center && r.pos.y < y_center)
            .count()
            * self
                .robots
                .iter()
                .filter(|r| r.pos.x > x_center && r.pos.y < y_center)
                .count()
            * self
                .robots
                .iter()
                .filter(|r| r.pos.x < x_center && r.pos.y > y_center)
                .count()
            * self
                .robots
                .iter()
                .filter(|r| r.pos.x > x_center && r.pos.y > y_center)
                .count()
    }
}

impl fmt::Display for BathroomLobby {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut robot_set = HashSet::new();

        for robot in self.robots.iter() {
            robot_set.insert(robot.pos);
        }

        writeln!(f);

        for y in 0..self.height {
            for x in 0..self.width {
                if robot_set.contains(&Point {
                    x: x as i32,
                    y: y as i32,
                }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        writeln!(f)
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let mut bathroom_lobby = BathroomLobby::from_string(101, 103, &input);

    for i in 0..10000 {
        bathroom_lobby.pass_second();

        if (i - 18) % 103 == 0 {
            // tree forms at second 8053 (see screenshot in `out/img.png`)
            sleep(Duration::from_millis(250));
            println!("Second {}", i + 1);
            println!("{}", bathroom_lobby)
        }
    }

    end_measure(mes);
}
