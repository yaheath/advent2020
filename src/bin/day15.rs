//use std::collections::HashMap;
use std::vec::Vec;
extern crate advent_lib;
use advent_lib::read::input_as_string;

fn doit(input: &str, target: usize) -> usize {
    let starting: Vec<usize> = input.split(',').flat_map(|s| s.parse::<usize>()).collect();
    let mut turn = 1usize;
    let mut last = 0;
    //let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut arr: Vec<(usize,usize)> = Vec::with_capacity(target);
    for _ in 0..target { arr.push((0,0)); }
    for s in &starting {
        //map.insert(*s, (0, turn));
        arr[*s] = (0, turn);
        last = *s;
        turn += 1;
    }
    while turn <= target {
        let next;
        //if map[&last].0 == 0 {
        if arr[last].0 == 0 {
            next = 0;
        }
        else {
            //next = map[&last].1 - map[&last].0;
            next = arr[last].1 - arr[last].0;
        }

        //map.entry(next)
        //    .and_modify(|v| *v = (v.1, turn))
        //    .or_insert((0, turn));
        arr[next] = (arr[next].1, turn);

        last = next;
        turn += 1;
    }
    last
}

fn part1(input: &str) -> usize {
    doit(input, 2020)
}

fn part2(input: &str) -> usize {
    doit(input, 30000000)
}

fn main() {
    let inputstr = input_as_string();
    let input = inputstr.trim();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_test() {
        let input = "0,3,6";
        assert_eq!(part1(&input), 436);
        assert_eq!(part2(&input), 175594);
        let input = "1,3,2";
        assert_eq!(part1(&input), 1);
        let input = "2,1,3";
        assert_eq!(part1(&input), 10);
        let input = "1,2,3";
        assert_eq!(part1(&input), 27);
        let input = "2,3,1";
        assert_eq!(part1(&input), 78);
        let input = "3,2,1";
        assert_eq!(part1(&input), 438);
        let input = "3,1,2";
        assert_eq!(part1(&input), 1836);
    }
}
