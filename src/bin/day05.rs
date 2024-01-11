use std::vec::Vec;
use std::str::FromStr;
use ya_advent_lib::read::read_input;

struct SeatID(String);

impl FromStr for SeatID {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SeatID(s.to_owned()))
    }
}

impl From<SeatID> for usize {
    fn from(s: SeatID) -> Self {
        usize::from_str_radix(
            &(s.0).replace('B', "1").replace('F', "0").replace('R', "1").replace('L', "0"),
            2,
        ).unwrap()
    }
}

fn part1(input: &[usize]) -> usize {
    input.iter().copied().max().unwrap()
}

fn part2(input: &[usize]) -> usize {
    input.windows(2)
        .find(|a| a[0] + 2 == a[1])
        .map(|a| a[0] + 1)
        .unwrap()
}

fn main() {
    let input: Vec<SeatID> = read_input();
    let mut input: Vec<usize> = input.into_iter().map(|s| s.into()).collect();
    input.sort_unstable();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_test() {
        let si: usize = "FBFBBFFRLR".parse::<SeatID>().unwrap().into();
        assert_eq!(si, 357);
        let si: usize = "BFFFBBFRRR".parse::<SeatID>().unwrap().into();
        assert_eq!(si, 567);
        let si: usize = "FFFBBBFRRR".parse::<SeatID>().unwrap().into();
        assert_eq!(si, 119);
        let si: usize = "BBFFBBFRLL".parse::<SeatID>().unwrap().into();
        assert_eq!(si, 820);
        let p2 = [6,7,8,9,11,12];
        assert_eq!(part2(&p2), 10);
    }
}
