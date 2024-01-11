use itertools::Itertools;
use ya_advent_lib::read::read_input;

fn first_invalid(input: &[i64], preamble_len: usize) -> i64 {
    input
        .windows(preamble_len + 1)
        .flat_map(|w| {
            let mut sums = w.iter()
                .take(preamble_len)
                .tuple_combinations()
                .filter(|(a,b)| a != b)
                .map(|(a,b)| a + b);
            let m:i64 = w[preamble_len];
            if !sums.any(|s| s == m) {
                Some(m)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn part1(input: &[i64], preamble_len: usize) -> i64 {
    first_invalid(input, preamble_len)
}

fn part2(input: &[i64], preamble_len: usize) -> i64 {
    let target = first_invalid(input, preamble_len);

    for start in 0..input.len() {
        let mut sum = 0i64;
        let mut min = input[start];
        let mut max = input[start];
        for inp in input.iter().skip(start) {
            sum += *inp;
            min = min.min(*inp);
            max = max.max(*inp);
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
    println!("Part 1: {}", part1(&input, 25));
    println!("Part 2: {}", part2(&input, 25));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day09_test() {
        let input = test_input::<i64>(include_str!("day09.testinput"));
        assert_eq!(part1(&input, 5), 127);
        assert_eq!(part2(&input, 5), 62);
    }
}
