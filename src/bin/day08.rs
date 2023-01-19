#[macro_use] extern crate lazy_static;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent_lib;
use advent_lib::read::read_input;

#[derive(Clone)]
enum Instruction {
    Nop(isize),
    Jmp(isize),
    Acc(isize),
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_INST: Regex = Regex::new(
                r"^(\w+) (.)(\d+)",
            ).unwrap();
        }
        if let Some(caps) = RE_INST.captures(s) {
            let o = caps.get(1).unwrap().as_str();
            let sgn = caps.get(2).unwrap().as_str();
            let n = caps.get(3).unwrap().as_str().parse::<isize>().unwrap();
            let arg = if sgn == "-" { -n } else { n };
            match o {
                "nop" => Ok(Self::Nop(arg)),
                "jmp" => Ok(Self::Jmp(arg)),
                "acc" => Ok(Self::Acc(arg)),
                _ => Err(format!("Invalid isntruction: {o}")),
            }
        }
        else {
            Err(format!("invalid input: {s}"))
        }
    }
}

enum RunResult {
    Ok,
    Halt(isize),
    Loop(isize),
}

struct VM<'a> {
    acc: isize,
    pc: usize,
    program: &'a Vec<Instruction>,
    inst_counter: Vec<usize>,
}
impl<'a> VM<'a> {
    fn new(program: &'a Vec<Instruction>) -> Self {
        let len = program.len();
        VM {
            acc: 0,
            pc: 0,
            program: program,
            inst_counter: vec![0; len],
        }
    }
    fn exec(&mut self) -> RunResult {
        let inst = &self.program[self.pc];
        self.inst_counter[self.pc] += 1;

        match inst{
            Instruction::Nop(_) => {},
            Instruction::Acc(arg) => {self.acc += arg;},
            Instruction::Jmp(arg) => {self.pc = (self.pc as isize + arg - 1) as usize;},
        }
        self.pc += 1;
        if self.pc >= self.program.len() {
            return RunResult::Halt(self.acc);
        }
        RunResult::Ok
    }
}

fn main() {
    let input = read_input::<Instruction>();
    part1(&input);
    part2(&input);
}

fn run(program: &Vec<Instruction>) -> RunResult {
    let mut vm = VM::new(program);
    loop {
        match vm.exec() {
            RunResult::Halt(a) => {
                return RunResult::Halt(a);
            },
            RunResult::Ok => {},
            _ => panic!(),
        };
        if vm.inst_counter[vm.pc] > 0 {
            return RunResult::Loop(vm.acc);
        }
    }
}

fn part1(input: &Vec<Instruction>) {
    let acc = match run(input) {
        RunResult::Loop(a) => a,
        _ => panic!(),
    };
    println!("Part 1: {acc}");
}

fn part2(input: &Vec<Instruction>) {
    for (idx, inst) in input.iter().enumerate() {
        let replace = match inst {
            Instruction::Nop(a) => Instruction::Jmp(*a),
            Instruction::Jmp(a) => Instruction::Nop(*a),
            Instruction::Acc(_) => {continue;},
        };
        let mut program: Vec<Instruction> = input.clone();
        program[idx] = replace;
        match run(&program) {
            RunResult::Halt(a) => {
                println!("Part 2: {a}");
                return;
            },
            RunResult::Loop(_) => {
                continue;
            },
            _ => panic!(),
        }
    }
}
