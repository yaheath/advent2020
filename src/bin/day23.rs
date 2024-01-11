use std::ops::Range;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

#[allow(dead_code)]
fn dbg_print(cups: &[usize]) {
    let mut c: usize = 1;
    while cups[c-1] != 1 {
        print!("{} ", ((c as u8) + b'0') as char);
        c = cups[c-1];
    }
    println!();
}

fn do_move(current: usize, cups: &mut Vec<usize>) -> usize {
    let taken1 = cups[current-1];
    let taken2 = cups[taken1-1];
    let taken3 = cups[taken2-1];
    let next = cups[taken3-1];
    let mut dest = current - 1;
    loop {
        if dest == 0 { dest = cups.len(); }
        if dest != taken1 && dest != taken2 && dest != taken3 {
            break;
        }
        dest -= 1;
    }
    let afterdest = cups[dest-1];
    cups[dest-1] = taken1;
    cups[taken3-1] = afterdest;
    cups[current-1] = next;
    next
}

fn populate_initial(seq: &[usize], cups: &mut Vec<usize>, extra: Range<usize>) -> usize{
    let start = seq[0];
    let mut last = 0;
    assert_eq!(cups.len(), seq.len());
    seq
        .iter()
        .circular_tuple_windows()
        .for_each(|(&n, &nxt)| {
            last = n;
            cups[n-1] = nxt;
        });
    if !extra.is_empty() {
        assert_eq!(extra.start, seq.len()+1);
        cups[last-1] = extra.start;
        for v in extra.clone() { cups.push(v+1); }
        cups[extra.end-2] = start;
    }
    start
}

fn part1(input: &str) -> String {
    let mut cups:Vec<usize> = vec![0; input.chars().count()];
    let initial_seq = input.chars()
        .map(|c| ((c as u8) - b'0') as usize)
        .collect::<Vec<_>>();
    let mut current = populate_initial(&initial_seq, &mut cups, 0..0);
    for _i in 0..100 {
        current = do_move(current, &mut cups);
        // if _i < 10 {
        //     dbg_print(&cups);
        // }
    }
    let mut out = String::new();
    let mut c = cups[0];
    while c != 1 {
        out += &String::from((c as u8 + b'0') as char);
        c = cups[c-1];
    }
    out
}

fn part2(input: &str) -> usize {
    let mut cups:Vec<usize> = Vec::with_capacity(1_000_000);
    for _ in input.chars() {
        cups.push(0);
    }
    let initial_seq = input.chars()
        .map(|c| ((c as u8) - b'0') as usize)
        .collect::<Vec<_>>();
    let extra = initial_seq.len() + 1 .. 1_000_001;
    let mut current = populate_initial(&initial_seq, &mut cups, extra);
    for _ in 0..10_000_000 {
        current = do_move(current, &mut cups);
    }
    cups[0] * cups[cups[0]-1]
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23_test() {
        let input:String = "389125467".into();
        assert_eq!(part1(&input), "67384529");
        assert_eq!(part2(&input), 149245887792);
    }
}
