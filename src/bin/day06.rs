use std::collections::HashMap;
use std::vec::Vec;
use ya_advent_lib::read::read_grouped_input;

fn setup(input: Vec<Vec<String>>) -> Vec<HashMap<char, usize>> {
    input.iter()
        .map(|lineset| {
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
            a
        })
        .collect()
}

fn part1(answers: &[HashMap<char, usize>]) -> usize {
    answers.iter().fold(0, |sum, a| sum + a.len() - 1)
}

fn part2(answers: &[HashMap<char, usize>]) -> usize {
    answers.iter().fold(0, |sum, a| {
        let n = a.get(&'_').unwrap();
        a.values().filter(|v| *v == n).count() - 1 + sum
    })
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input();
    let answers = setup(input);
    println!("Part 1: {}", part1(&answers));
    println!("Part 2: {}", part2(&answers));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day06_test() {
        let input:Vec<Vec<String>> = grouped_test_input(include_str!("day06.testinput"));
        let answers = setup(input);
        assert_eq!(part1(&answers), 11);
        assert_eq!(part2(&answers), 6);
    }
}
