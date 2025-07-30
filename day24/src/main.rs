use core::fmt;
use std::{
    cmp::Ordering,
    collections::VecDeque,
    mem::{self},
};

use aoc_core::{end_measure, read, start_measure};

#[derive(Debug)]
struct Instruction {
    a_reg: String,
    b_reg: String,
    operation: String,
    res_reg: String,
    res_map: Option<String>,
}

impl Instruction {
    fn from_string(input: String) -> Self {
        let split: Vec<_> = input.split_whitespace().collect();

        Instruction {
            a_reg: split.get(0).unwrap().to_string(),
            b_reg: split.get(2).unwrap().to_string(),
            operation: split.get(1).unwrap().to_string(),
            res_reg: split.get(4).unwrap().to_string(),
            res_map: None,
        }
    }

    fn get_a_num(&self) -> &str {
        &self.a_reg[self.a_reg.len() - 2..]
    }

    fn get_b_num(&self) -> &str {
        &self.b_reg[self.b_reg.len() - 2..]
    }

    fn get_res_num(&self) -> &str {
        &self.res_reg[self.res_reg.len() - 2..]
    }

    fn is(&self, reg_a: String, operation: &str, reg_b: String) -> bool {
        return self.operation == operation
            && ((self.a_reg == reg_a && self.b_reg == reg_b)
                || (self.a_reg == reg_b && self.b_reg == reg_a))
            && !self.res_reg.starts_with("z"); // never replace a z
    }

    fn replace(&mut self, from: &str, to: &str) {
        if self.a_reg == from {
            self.a_reg = to.to_string();
        }

        if self.b_reg == from {
            self.b_reg = to.to_string();
        }

        if self.res_reg == from {
            self.res_map = Some(self.res_reg.clone());
            self.res_reg = to.to_string();
        }
    }

    fn get_num(&self) -> Option<&str> {
        let a_num = self.get_a_num();
        let b_num = self.get_b_num();

        if a_num == b_num {
            Some(a_num)
        } else {
            None
        }
    }

    fn set_get_cin(&mut self) -> Option<(String, String)> {
        if !self.res_reg.starts_with("z") {
            return None;
        }

        let a_num = self.get_a_num();
        let b_num = self.get_b_num();
        let z_num = self.get_res_num().to_string();

        if self.a_reg.starts_with("o") && a_num == z_num {
            let cin = self.b_reg.clone();
            self.b_reg = format!("cin{z_num}");

            return Some((cin, z_num));
        } else if self.b_reg.starts_with("o") && b_num == z_num {
            let cin = self.a_reg.clone();
            self.a_reg = format!("cin{z_num}");

            return Some((cin, z_num));
        }

        None
    }

    fn sort(&mut self) {
        match self.a_reg.cmp(&self.b_reg) {
            Ordering::Greater => {
                mem::swap(&mut self.a_reg, &mut self.b_reg);
            }
            _ => {}
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:<3} {} -> {:<10} ({:?})",
            self.a_reg, self.operation, self.b_reg, self.res_reg, self.res_map
        )?;

        Ok(())
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");
    let mut instructions: VecDeque<_> = input
        .split_once("\n\n")
        .unwrap()
        .1
        .lines()
        .map(|l| Instruction::from_string(l.to_string()))
        .collect();

    let mut mapped = vec![];
    let mut loop_count = 0;

    // RESULT
    swap(&mut instructions, "wbw", "wgb");
    swap(&mut instructions, "gwh", "z09");
    swap(&mut instructions, "rcb", "z21");
    swap(&mut instructions, "jct", "z39");

    while instructions.len() > 0 && loop_count < instructions.len() {
        let mut inst = instructions.pop_front().unwrap();

        // if o00 in a or b && z is res get cin
        let cin_o = inst.set_get_cin();

        if let Some((cin, num)) = cin_o {
            replace(&mut instructions, cin, &format!("cin{num}"));
        }

        if let Some(num_str) = inst.get_num() {
            let num = num_str.parse::<u8>().unwrap();

            // if x00 XOR y00 || y00 XOR y00 -> replace (and all other) with o00
            if inst.is(format!("x{num:02}"), "XOR", format!("y{num:02}")) {
                replace_res_to(
                    &mut instructions,
                    &mut inst,
                    &format!("o{num:02}"),
                );
            }

            // if x00 AND y00 || y00 AND y00 -> replace (and all other) with i000
            if inst.is(format!("x{num:02}"), "AND", format!("y{num:02}")) {
                replace_res_to(
                    &mut instructions,
                    &mut inst,
                    &format!("i0{num:02}"),
                );
            }

            // if o00 AND cin00 || cin00 AND o00 -> replace (and all other) with i100
            if inst.is(
                format!("o{num:02}"),
                "AND",
                format!("cin{num:02}"),
            ) {
                replace_res_to(
                    &mut instructions,
                    &mut inst,
                    &format!("i1{num:02}"),
                );
            }

            // if i000 OR i100 || i100 AND i000 -> replace (and all other) with cin01
            if inst.is(format!("i0{num:02}"), "OR", format!("i1{num:02}")) {
                replace_res_to(
                    &mut instructions,
                    &mut inst,
                    &format!("cin{:02}", num + 1),
                );
            }

            // if o00 XOR cin00 || cin00 XOR o00 -> should be z00

            mapped.push(inst);
            loop_count = 0;
        } else {
            loop_count += 1;
            instructions.push_back(inst);
        }
    }

    let mut result = vec![];

    for inst in &mut instructions {
        inst.sort();
        result.push(inst);
    }

    for inst in &mut mapped {
        inst.sort();
        result.push(inst);
    }

    println!("--- result ---");
    result.sort_by_key(|i| (i.a_reg.clone(), i.operation.clone()));
    for d in result {
        println!("{d}");
    }

    end_measure(mes);
}

fn swap(instructions: &mut VecDeque<Instruction>, from: &str, with: &str) {
    println!("{from}");
    println!("{with}");

    let slice = instructions.make_contiguous();

    let pos_from = slice
        .iter()
        .position(|i| i.res_reg == from)
        .expect("`from` register not found");
    let pos_with = slice
        .iter()
        .position(|i| i.res_reg == with)
        .expect("`with` register not found");

    let (left, right) = if pos_from < pos_with {
        let (l, r) = slice.split_at_mut(pos_with);
        (&mut l[pos_from].res_reg, &mut r[0].res_reg)
    } else {
        let (l, r) = slice.split_at_mut(pos_from);
        (&mut r[0].res_reg, &mut l[pos_with].res_reg)
    };

    mem::swap(left, right);
}

fn replace_res_to(
    instructions: &mut VecDeque<Instruction>,
    from_inst: &mut Instruction,
    to: &str,
) {
    for inst in instructions {
        inst.replace(&from_inst.res_reg, to);
    }

    if from_inst.res_reg != to.to_string() {
        from_inst.res_map = Some(from_inst.res_reg.clone());
        from_inst.res_reg = to.to_string();
    }
}

fn replace(instructions: &mut VecDeque<Instruction>, from: String, to: &str) {
    for inst in instructions {
        inst.replace(&from, to);
    }
}
