use std::vec::Vec;
extern crate advent2020;
use advent2020::read::read_input;

fn part1(input: &Vec<i32>) {
    let len = input.len();
    for i in 0..(len-1) {
        for j in (i+1)..len {
            if input[i] + input[j] == 2020 {
                println!("Part 1: {}", input[i] * input[j]);
                return;
            }
        }
    }
}

fn part2(input: &Vec<i32>) {
    let len = input.len();
    for i in 0..(len-1) {
        for j in (i+1)..len {
            for k in (j+1)..len {
                if input[i] + input[j] + input[k] == 2020 {
                    println!("Part 2: {}", input[i] * input[j] * input[k]);
                    return;
                }
            }
        }
    }
}

fn main() {
    let input: Vec<i32> = read_input::<i32>();
    part1(&input);
    part2(&input);
}
