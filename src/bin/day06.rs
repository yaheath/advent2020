use std::collections::HashMap;
use std::iter::once;
use std::vec::Vec;
extern crate advent2020;
use advent2020::read::read_input;

fn main() {
    let input: Vec<String> = read_input::<String>();
    let mut lineset: Vec<String> = Vec::new();
    let mut answers: Vec<HashMap<char, usize>> = Vec::new();
    for line in input.iter().map(|s| s.as_str()).chain(once("")) {
        if line == "" {
            let mut a: HashMap<char, usize> = HashMap::new();
            lineset
                .join("")
                .chars()
                .filter(|c| *c >= 'a' && *c <= 'z')
                .for_each(|c| {
                    let n = a.entry(c).or_insert(0);
                    *n += 1;
                });
            a.insert('_', lineset.len());
            answers.push(a);
            lineset.clear();
        } else {
            lineset.push(line.to_string());
        }
    }

    part1(&answers);
    part2(&answers);
}

fn part1(answers: &Vec<HashMap<char, usize>>) {
    let sum = answers.iter().fold(0, |sum, a| sum + a.len() - 1);
    println!("Part 1: {}", sum);
}
fn part2(answers: &Vec<HashMap<char, usize>>) {
    let sum = answers.iter().fold(0, |sum, a| {
        let n = a.get(&'_').unwrap();
        a.values().filter(|v| *v == n).count() - 1 + sum
    });
    println!("Part 2: {}", sum);
}
