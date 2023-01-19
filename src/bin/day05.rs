use std::vec::Vec;
extern crate advent_lib;
use advent_lib::read::read_input;

fn part1(input: &Vec<usize>) {
    let mut max = 0usize;
    for val in input {
        if *val > max {
            max = *val;
        }
    }
    println!("Part 1: {}", max);
}

fn part2(input: &Vec<usize>) {
    let mut last = input.get(0).unwrap();
    for val in input {
        if *val == last + 2 {
            println!("Part 2: {}", last + 1);
            return;
        }
        last = val;
    }
}

fn main() {
    let mut input: Vec<usize> = read_input::<String>().iter().map(
        |s| usize::from_str_radix(
            &s.replace('B', "1").replace('F', "0").replace('R', "1").replace('L', "0"),
            2
        ).unwrap()
    ).collect();
    input.sort_unstable();
    part1(&input);
    part2(&input);
}
