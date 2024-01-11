use std::collections::HashMap;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn part1(input: &[i64]) -> i64 {
    let mut list = input.to_owned();
    let mut hist: HashMap<i64,i64> = HashMap::new();

    list.push(0);
    list.sort_unstable();
    list.push(list[list.len() - 1] + 3);
    list.windows(2)
        .map(|w| w[1] - w[0])
        .for_each(|v| {
            hist.entry(v).and_modify(|c| {*c += 1;}).or_insert(1);
        });

    hist[&1] * hist[&3]
}

fn rsearch(slice: &[i64]) -> usize {
    let mut sum = 0;
    if slice.len() == 0 { return 0; }
    if slice.len() == 1 { return 1; }
    if slice[1] - slice[0] >= 3 { panic!(); }
    sum += rsearch(&slice[1..]);
    if slice.len() == 2 { return sum; }
    if slice[2] - slice[0] > 3 { return sum; }
    sum += rsearch(&slice[2..]);
    if slice.len() == 3 { return sum; }
    if slice[3] - slice[0] > 3 { return sum; }
    sum += rsearch(&slice[3..]);
    sum
}

fn part2(input: &[i64]) -> usize {
    let mut list = input.to_owned();
    list.push(0);
    list.sort_unstable();
    list.push(list[list.len() - 1] + 3);
    let mut sublist: Vec<i64> = Vec::new();
    let mut result = 1usize;
    for (idx, w) in list.windows(2).enumerate() {
        sublist.push(w[0]);
        if w[1] - w[0] >= 3 {
            let start = idx - (sublist.len() - 1);
            let slice = &list[start..=idx];
            result *= rsearch(slice);
            sublist.clear();
        }
    }
    result
}

fn main() {
    let input = read_input::<i64>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_test() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(part1(&input), 35);
        assert_eq!(part2(&input), 8);

        let input = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        assert_eq!(part1(&input), 220);
        assert_eq!(part2(&input), 19208);
    }
}
