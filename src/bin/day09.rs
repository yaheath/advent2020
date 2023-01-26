use std::vec::Vec;
use itertools::Itertools;
extern crate advent_lib;
use advent_lib::read::read_input;

fn part1(input: &Vec<i64>) -> i64 {
    input
        .windows(26)
        .flat_map(|w| {
            let mut sums = w.iter()
                .take(25)
                .tuple_combinations()
                .filter(|(a,b)| a != b)
                .map(|(a,b)| a + b);
            let m:i64 = w[25];
            if !sums.any(|s| s == m) {
                Some(m)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn part2(input: &Vec<i64>) -> i64 {
    let target = part1(input);

    for start in 0..input.len() {
        let mut sum = 0i64;
        let mut min = input[start];
        let mut max = input[start];
        for i in start.. {
            sum += input[i];
            min = min.min(input[i]);
            max = max.max(input[i]);
            if sum > target { break; }
            if sum == target {
                return min + max;
            }
        }
    }
    panic!();
}

fn main() {
    let input = read_input::<i64>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
