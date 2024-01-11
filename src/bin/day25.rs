use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn modpow(base: u64, exp: u64, m: u64) -> u64 {
    if exp == 0 { return 1; }
    let mut res = 1;
    let mut base = base % m;
    let mut exp = exp;
    loop {
        if exp % 2 == 1 {
            res *= base;
            res %= m;
        }
        if exp == 1 {
            return res;
        }
        base *= base;
        base %= m;
        exp /= 2;
    }
}

fn part1(input: &Vec<u64>) -> u64 {
    let mut card_loop = 1u64;
    while modpow(7, card_loop, 20201227) != input[0] {
        card_loop += 1;
    }
    let card_key = modpow(input[1], card_loop, 20201227);

    #[cfg(debug_assertions)]
    {
        let mut door_loop = 1u64;
        while modpow(7, door_loop, 20201227) != input[1] {
            door_loop += 1;
        }
        let door_key = modpow(input[0], door_loop, 20201227);
        assert_eq!(card_key, door_key);
    }

    card_key
}

fn main() {
    let input:Vec<u64> = read_input();
    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day25_test() {
        let input:Vec<u64> = vec![5764801, 17807724];
        assert_eq!(part1(&input), 14897079);
    }
}
