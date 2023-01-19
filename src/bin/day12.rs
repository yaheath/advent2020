#[macro_use] extern crate lazy_static;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent2020;
use advent2020::read::read_input;

enum Nav {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}
impl FromStr for Nav {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_INST: Regex = Regex::new(
                r"^(\w)(\d+)",
            ).unwrap();
        }
        if let Some(caps) = RE_INST.captures(s) {
            let a = caps.get(1).unwrap().as_str();
            let v = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            match a {
                "N" => Ok(Self::North(v)),
                "S" => Ok(Self::South(v)),
                "E" => Ok(Self::East(v)),
                "W" => Ok(Self::West(v)),
                "L" => Ok(Self::Left(v)),
                "R" => Ok(Self::Right(v)),
                "F" => Ok(Self::Forward(v)),
                _ => Err(format!("invalid input: {}", s)),
            }
        }
        else {
            Err(format!("invalid input: {}", s))
        }
    }
}

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    heading: i32,
}
impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            heading: 90,
        }
    }
    fn update(&mut self, instr: &Nav) {
        match instr {
            Nav::North(v) => { self.y += v; },
            Nav::South(v) => { self.y -= v; },
            Nav::East(v) => { self.x += v; },
            Nav::West(v) => { self.x -= v; },
            Nav::Left(v) => { self.heading = (self.heading - v + 360) % 360; },
            Nav::Right(v) => { self.heading = (self.heading + v) % 360; },
            Nav::Forward(v) => { match self.heading {
                0 => { self.y += v; },
                90 => { self.x += v; },
                180 => { self.y -= v; },
                270 => { self.x -= v; },
                _ => panic!(),
            };},
        }
    }
}

#[derive(Debug)]
struct Ship2 {
    x: i32,
    y: i32,
    wp_x: i32,
    wp_y: i32,
}
impl Ship2 {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            wp_x: 10,
            wp_y: 1,
        }
    }
    fn update(&mut self, instr: &Nav) {
        match instr {
            Nav::North(v) => { self.wp_y += v; },
            Nav::South(v) => { self.wp_y -= v; },
            Nav::East(v) => { self.wp_x += v; },
            Nav::West(v) => { self.wp_x -= v; },
            Nav::Left(180) | Nav::Right(180) => {
                (self.wp_x, self.wp_y) = (-self.wp_x, -self.wp_y);
            },
            Nav::Left(90) | Nav::Right(270) => {
                (self.wp_x, self.wp_y) = (-self.wp_y, self.wp_x);
            },
            Nav::Left(270) | Nav::Right(90) => {
                (self.wp_x, self.wp_y) = (self.wp_y, -self.wp_x);
            },
            Nav::Left(_) | Nav::Right(_) => panic!(),
            Nav::Forward(v) => {
                self.x += v * self.wp_x;
                self.y += v * self.wp_y;
            },
        }
    }
}

fn part1(input: &Vec<Nav>) -> i32 {
    let mut ship = Ship::new();
    for i in input {
        ship.update(i);
    }
    ship.x.abs() + ship.y.abs()
}

fn part2(input: &Vec<Nav>) -> i32 {
    let mut ship = Ship2::new();
    for i in input {
        ship.update(i);
    }
    ship.x.abs() + ship.y.abs()
}

fn main() {
    let input = read_input::<Nav>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_test() {
        let input: Vec<Nav> = vec![
            Nav::from_str("F10").unwrap(),
            Nav::from_str("N3").unwrap(),
            Nav::from_str("F7").unwrap(),
            Nav::from_str("R90").unwrap(),
            Nav::from_str("F11").unwrap(),
        ];
        assert_eq!(part1(&input), 25);
        assert_eq!(part2(&input), 286);
    }
}
