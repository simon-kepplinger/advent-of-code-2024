use std::usize;

use aoc_core::{end_measure, read, start_measure};

#[derive(Debug)]
struct ThreeBitComp {
    a: u32,
    b: u32,
    c: u32,
    ptr: usize,
    prog: Vec<u8>,
    opcode: u8,
    operand: u8,
    out: Vec<String>,
}

impl ThreeBitComp {
    fn from_str(input: &str) -> Self {
        let mut lines: Vec<_> = input
            .split("\n")
            .filter(|s| !s.is_empty())
            .collect();

        let prog = lines
            .pop()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        let nums: Vec<u32> = lines
            .iter()
            .map(|l| l.split(":").nth(1).unwrap().trim())
            .map(|s| s.parse().unwrap())
            .collect();

        return ThreeBitComp {
            a: nums[0],
            b: nums[1],
            c: nums[2],
            ptr: 0,
            prog: prog,
            opcode: 0,
            operand: 0,
            out: vec![],
        };
    }

    pub fn mov(&mut self) -> u8 {
        let opcode_opt = self.prog.get(self.ptr);

        match opcode_opt {
            Some(optcode) => {
                self.opcode = *optcode;
                self.operand = *self.prog.get(self.ptr + 1).unwrap();
                self.ptr += 2;

                0
            }
            None => 1,
        }
    }

    pub fn exec(&mut self) {
        match self.opcode {
            0 => self.a = self.a / 2_u32.pow(self.comb()),
            1 => self.b = self.b ^ self.operand as u32,
            2 => self.b = self.comb() % 8,
            3 => {
                if self.a != 0 {
                    self.ptr = self.operand as usize
                }
            }
            4 => self.b = self.b ^ self.c,
            5 => self.out.push((self.comb() % 8).to_string()),
            6 => self.b = self.a / 2_u32.pow(self.comb()),
            7 => self.c = self.a / 2_u32.pow(self.comb()),
            _ => (),
        }
    }

    pub fn comb(&self) -> u32 {
        match self.operand {
            0 | 1 | 2 | 3 => self.operand as u32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Could not intepret combo."),
        }
    }

    pub fn print(&self) {
        println!("{}", self.out.join(","));
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let mut comp = ThreeBitComp::from_str(&input);

    while comp.mov() == 0 {
        comp.exec();
    }

    comp.print();
    end_measure(mes);
}
