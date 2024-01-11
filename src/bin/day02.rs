use std::vec::Vec;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;

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

fn part1(input: &[Pass]) -> usize {
    input.iter().filter(|p| p.is_valid_part1()).count()
}

fn part2(input: &[Pass]) -> usize {
    input.iter().filter(|p| p.is_valid_part2()).count()
}

fn main() {
    let input: Vec<Pass> = read_input::<Pass>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day02_test() {
        let input:Vec<Pass> = test_input(include_str!("day02.testinput"));
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 1);
    }
}
