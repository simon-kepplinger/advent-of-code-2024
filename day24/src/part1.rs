use std::collections::{HashMap, VecDeque};

use aoc_core::{end_measure, read, start_measure};

#[derive(Debug)]
struct Instruction {
    a_reg: String,
    b_reg: String,
    operation: String,
    res_reg: String,
}

impl Instruction {
    fn from_string(input: String) -> Self {
        let split: Vec<_> = input.split_whitespace().collect();

        Instruction {
            a_reg: split.get(0).unwrap().to_string(),
            b_reg: split.get(2).unwrap().to_string(),
            operation: split.get(1).unwrap().to_string(),
            res_reg: split.get(4).unwrap().to_string(),
        }
    }
}

#[derive(Debug)]
struct Device {
    registers: HashMap<String, u8>,
    program: VecDeque<Instruction>,
}

impl Device {
    fn from_string(input: String) -> Self {
        let (reg_str, prg_str) = input.split_once("\n\n").unwrap();

        let registers: HashMap<_, _> = reg_str
            .lines()
            .map(|l| l.split_once(":").unwrap())
            .map(|(r, v)| (r.to_string(), v.trim().parse::<u8>().unwrap()))
            .collect();

        let program = prg_str
            .lines()
            .map(|s| Instruction::from_string(s.to_string()))
            .collect();

        Device { registers, program }
    }

    fn execute(&mut self) {
        while self.program.len() != 0 {
            let inst = self.program.pop_front().unwrap();

            let a_reg = self.registers.get(&inst.a_reg);
            let b_reg = self.registers.get(&inst.b_reg);

            if let (Some(a), Some(b)) = (a_reg, b_reg) {
                let res = Device::calc(&inst.operation, *a, *b);

                self.registers.insert(inst.res_reg.clone(), res);
            } else {
                self.program.push_back(inst);
            }
        }
    }

    fn calc(op: &str, a: u8, b: u8) -> u8 {
        match op {
            "AND" => a & b,
            "OR" => a | b,
            "XOR" => a ^ b,
            _ => 0,
        }
    }

    fn z_reg_dec(&self) -> u64 {
        let mut z_regs = self
            .registers
            .iter()
            .filter(|r| r.0.starts_with("z"))
            .collect::<Vec<_>>();

        z_regs.sort_by_key(|r| r.0);

        z_regs
            .iter()
            .enumerate()
            .map(|(i, &bit)| (*bit.1 as u64) << i)
            .sum()
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let mut device = Device::from_string(input);
    device.execute();

    let z_res = device.z_reg_dec();

    println!("{:#?}", z_res);

    end_measure(mes);
}
