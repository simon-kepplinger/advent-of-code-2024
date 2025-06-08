use core::panic;
use std::usize;

use aoc_core::{end_measure, read, start_measure};

#[derive(Debug)]
struct ThreeBitComp {
    a: u64,
    b: u64,
    c: u64,
    ptr: usize,
    prog: Vec<u8>,
    opcode: u8,
    operand: u8,
    out: String,

    corrupt: bool,
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

        let nums: Vec<u64> = lines
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
            out: "".to_string(),
            corrupt: false,
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
            0 => self.a = self.a / 2_u64.pow(self.get_comb() as u32),
            1 => self.b = self.b ^ self.operand as u64,
            2 => self.b = self.get_comb() % 8,
            3 => {
                if self.a != 0 {
                    self.ptr = self.operand as usize
                }
            }
            4 => self.b = self.b ^ self.c,
            5 => {
                if self.out.len() != 0 {
                    self.out.push(',');
                }

                self.out
                    .push_str((self.get_comb() % 8).to_string().as_str());

                let prog_str = self
                    .prog
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                self.corrupt = !prog_str.starts_with(&self.out);
            }
            6 => self.b = self.a / 2_u64.pow(self.get_comb() as u32),
            7 => self.c = self.a / 2_u64.pow(self.get_comb() as u32),
            _ => (),
        }
    }

    pub fn get_comb(&self) -> u64 {
        match self.operand {
            0 | 1 | 2 | 3 => self.operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Could not intepret combo."),
        }
    }

    pub fn init(&mut self, val: u64) {
        self.a = val;
        self.b = 0;
        self.c = 0;

        self.ptr = 0;
        self.out = "".to_string();
        self.corrupt = false;
    }

    pub fn is_copied(&self) -> bool {
        let prog_str = self
            .prog
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(",");

        return prog_str == self.out;
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let mut comp = ThreeBitComp::from_str(&input);

    let mut initial: u64 = 625681307; // found through empirical analysis
    let mut literal_last = initial;

    while !comp.is_copied() {
        comp.init(initial);

        while comp.mov() == 0 && !comp.corrupt {
            comp.exec();
        }

        if comp.out.len() > 28 {
            println!(
                "{}: {} [{}] (ptr: {}, a: {})",
                initial,
                comp.out,
                initial - literal_last,
                comp.ptr,
                comp.a
            );
            literal_last = initial;
        }

        initial = initial + 536870912; // found through empirical analysis
    }

    println!("copied with init: {}", initial - 1);

    end_measure(mes);
}
