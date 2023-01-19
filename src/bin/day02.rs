#[macro_use] extern crate lazy_static;
use std::vec::Vec;
use std::str::FromStr;
use regex::Regex;
extern crate advent2020;
use advent2020::read::read_input;

#[derive(Debug)]
struct Pass {
    min: usize,
    max: usize,
    c: char,
    pass: String,
}

impl Pass {
    fn is_valid_part1(&self) -> bool {
        let count = self.pass
            .chars()
            .filter(|c| *c == self.c)
            .count();

        count >= self.min && count <= self.max
    }

    fn is_valid_part2(&self) -> bool {
        let chars: Vec<char> = self.pass.chars().collect();
        let a = chars[self.min-1] == self.c;
        let b = chars[self.max-1] == self.c;

        a != b
    }
}

impl FromStr for Pass {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)").unwrap();
        }
        match RE.captures(s) {
            None => return Err(format!("invalid input: {}", s)),
            Some(caps) => {
                let min:usize = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let max:usize = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let c:char = caps.get(3).unwrap().as_str().chars().next().unwrap();
                let pass:String = caps.get(4).unwrap().as_str().to_string();
                return Ok(Pass {min: min, max: max, c: c, pass: pass});
            },
        }
    }
}

fn part1(input: &Vec<Pass>) {
    let result = input.iter().filter(|p| p.is_valid_part1()).count();
    println!("Part 1: {}", result);
}

fn part2(input: &Vec<Pass>) {
    let result = input.iter().filter(|p| p.is_valid_part2()).count();
    println!("Part 2: {}", result);
}

fn main() {
    let input: Vec<Pass> = read_input::<Pass>();
    part1(&input);
    part2(&input);
}
