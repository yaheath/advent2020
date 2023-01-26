use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
extern crate advent_lib;
use advent_lib::read::read_input;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Mask {
    or: u64,
    and: u64,
    float: u64,
}
impl FromStr for Mask {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let or_str = s.replace("X", &"0");
        let and_str = "1".repeat(28) + &s.replace("X", &"1");
        let float_str = s.replace("1", &"0").replace("X", &"1");
        Ok(Mask {
            or: u64::from_str_radix(&or_str, 2)?,
            and: u64::from_str_radix(&and_str, 2)?,
            float: u64::from_str_radix(&float_str, 2)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    SetMask(Mask),
    SetMem(usize,u64), // (location, value)
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            let bstr = s.split(" = ").skip(1).next().unwrap();
            Ok(Instruction::SetMask(
                bstr.parse::<Mask>().unwrap()
            ))
        }
        else {
            let mut si = s.split("] = ");
            Ok(Instruction::SetMem(
                si.next().unwrap().split('[').skip(1).next().unwrap().parse::<usize>().unwrap(),
                si.next().unwrap().parse::<u64>().unwrap(),
            ))
        }
    }
}

fn splode_bits(val: u64) -> Vec<u64> {
    let mut out = Vec::new();
    for b in 0..36u64 {
        if val & (1<<b) != 0 {
            out.push(1<<b);
        }
    }
    out
}

struct VM {
    mem: HashMap<usize, u64>,
    mask: Mask,
}

impl VM {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
            mask: Mask { or: 0, and: !0, float: 0 },
        }
    }
    fn process(&mut self, inst: &Instruction) {
        match inst {
            Instruction::SetMask(m) => { self.mask = m.clone(); },
            Instruction::SetMem(loc, val) => {
                self.mem.insert(*loc, (*val | self.mask.or) & self.mask.and);
            },
        }
    }
    fn process_v2(&mut self, inst: &Instruction) {
        match inst {
            Instruction::SetMask(m) => { self.mask = m.clone(); },
            Instruction::SetMem(loc, val) => {
                let loc:u64 = (*loc as u64 | self.mask.or) & !self.mask.float;
                splode_bits(self.mask.float)
                    .iter()
                    .map(|&m| [0, m].into_iter())
                    .multi_cartesian_product()
                    .map(|v| v.into_iter().reduce(|acc, e| acc | e).unwrap() | loc)
                    .for_each(|l| {self.mem.insert(l as usize, *val);});
            },
        }
    }
}

fn part1(input: &Vec<Instruction>) -> u64 {
    let mut vm = VM::new();
    for i in input {
        vm.process(i);
    }
    vm.mem.iter()
        .map(|(_,v)| v)
        .sum()
}

fn part2(input: &Vec<Instruction>) -> u64 {
    let mut vm = VM::new();
    for i in input {
        vm.process_v2(i);
    }
    vm.mem.iter()
        .map(|(_,v)| v)
        .sum()
}

fn main() {
    let input = read_input::<Instruction>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_test() {
        let instr = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        assert_eq!(
            instr.parse::<Instruction>(),
            Ok(Instruction::SetMask(Mask { or: 64, and: !2, float: 0xFFFFFFFBD}))
        );
        let instr = "mem[7] = 101";
        assert_eq!(
            instr.parse::<Instruction>(),
            Ok(Instruction::SetMem(7, 101)),
        );
    }
}
