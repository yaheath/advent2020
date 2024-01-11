use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

fn part1(input: &[i32]) -> i32 {
    input.iter()
        .tuple_combinations()
        .find(|(a, b)| *a + *b == 2020)
        .map(|(a, b)| a * b)
        .unwrap()
}

fn part2(input: &[i32]) -> i32 {
    input.iter()
        .tuple_combinations()
        .find(|(a, b, c)| *a + *b + *c == 2020)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}

fn main() {
    let input: Vec<i32> = read_input::<i32>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day01_test() {
        let input:Vec<i32> = test_input(include_str!("day01.testinput"));
        assert_eq!(part1(&input), 514579);
        assert_eq!(part2(&input), 241861950);
    }
}
