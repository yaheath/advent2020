use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_grouped_input;

type RuleNum = usize;

#[derive(Clone, Debug)]
enum Rule {
    AltSeq(Vec<Vec<RuleNum>>),
    Char(char),
}

#[derive(Clone, Debug)]
struct RuleEntry {
    id: RuleNum,
    rule: Rule,
}

impl FromStr for RuleEntry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split(": ");
        let id = spl.next().unwrap().parse::<usize>().unwrap();
        let rule = spl.next().unwrap();
        if rule.contains('\"') {
            Ok(RuleEntry { id, rule: Rule::Char(rule.chars().nth(1).unwrap()) })
        }
        else {
            let seqs: Vec<Vec<RuleNum>> = rule.split(" | ").map(
                |sp| sp.split(' ').map(|n| n.parse::<usize>().unwrap()).collect()
            ).collect();
            Ok(RuleEntry { id, rule: Rule::AltSeq(seqs) })
        }
    }
}

fn parse_rules(input: &[String]) -> HashMap<RuleNum, Rule> {
    input
        .iter()
        .map(|line| line.parse::<RuleEntry>().unwrap())
        .map(|re| (re.id, re.rule))
        .collect()
}

fn process_seq<'a>(s: &'a str, seq: &[RuleNum], rules: &HashMap<RuleNum, Rule>) -> Vec<&'a str> {
    seq.iter()
        .try_fold(vec![s], |solutions, rule_num| {
            let next_solutions: Vec<&str> = solutions
                .iter()
                .flat_map(|input| process_rules(input, *rule_num, rules))
                .collect();
            if next_solutions.is_empty() {
                None
            } else {
                Some(next_solutions)
            }
        })
        .unwrap_or_else(Vec::new)
}

fn process_rules<'a>(s: &'a str, rule_num: RuleNum, rules: &HashMap<RuleNum, Rule>) -> Vec<&'a str> {
    let rule = &rules[&rule_num];
    match rule {
        Rule::Char(c) => s.chars()
            .next()
            .filter(|f| *f == *c)
            .map(|_| &s[1..])
            .into_iter()
            .collect(),

        Rule::AltSeq(v) => v.iter()
            .flat_map(|seq| process_seq(s, seq, rules))
            .collect(),
    }
}

fn is_match(s: &str, rules: &HashMap<RuleNum, Rule>) -> bool {
    process_rules(s, 0, rules)
        .iter()
        .any(|&r| r.is_empty())
}

fn part1(rules: &HashMap<RuleNum, Rule>, strings: &[String]) -> usize {
    strings.iter().filter(|s| is_match(s, rules)).count()
}

fn part2(rules: &HashMap<RuleNum, Rule>, strings: &[String]) -> usize {
    let mut rules = rules.clone();
    rules.insert(8, Rule::AltSeq(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::AltSeq(vec![vec![42, 31], vec![42, 11, 31]]));
    strings.iter().filter(|s| is_match(s, &rules)).count()
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input::<String>();
    let rules = parse_rules(&input[0]);
    println!("Part 1: {}", part1(&rules, &input[1]));
    println!("Part 2: {}", part2(&rules, &input[1]));
}


#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day19_test() {
        let input = grouped_test_input::<String>(include_str!("day19.testinput"));
        let rules = parse_rules(&input[0]);
        assert_eq!(part1(&rules, &input[1]), 2);
        let input = grouped_test_input::<String>(include_str!("day19.testinput2"));
        let rules = parse_rules(&input[0]);
        assert_eq!(part2(&rules, &input[1]), 12);
    }
}
