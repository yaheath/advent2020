use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn part1(input: &Vec<String>) -> usize {
    let ts = input[0].parse::<usize>().unwrap();
    let buses = input[1]
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse::<usize>().unwrap());
    let mut times: Vec<(usize, usize)> = buses
        .map(|b| (((ts - 1 + b) / b) * b - ts, b))
        .collect();
    times.sort();
    times[0].0 * times[0].1
}

fn part2(input: &Vec<String>) -> usize {
    let mut buses = input[1]
        .split(',')
        .enumerate()
        .map(|(idx, s)| (idx, s.parse::<usize>()))
        .filter(|(_, s)| s.is_ok())
        .map(|(idx, s)| (idx, s.unwrap()));
    let mut period = buses.next().unwrap().1;
    let mut time = period;
    for (idx, bus) in buses {
        while (time + idx) % bus != 0 {
            time += period;
        }
        period *= bus;
    }
    time

}

fn main() {
    let input = read_input::<String>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_test() {
        let input: Vec<String> = vec![
            "939".into(),
            "7,13,x,x,59,x,31,19".into(),
        ];

        assert_eq!(part1(&input), 295);
        assert_eq!(part2(&input), 1068781);
    }
}
