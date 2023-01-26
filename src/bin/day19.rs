use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent_lib;
use advent_lib::read::read_grouped_input;

type RuleNum = usize;

#[derive(Clone, Debug)]
enum Rule {
    AltRule(Vec<RuleNum>, Vec<RuleNum>),
    Seq(Vec<RuleNum>),
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
        if rule.contains("\"") {
            return Ok(RuleEntry { id, rule: Rule::Char(rule.chars().skip(1).next().unwrap()) });
        }
        let mut seqs: Vec<Vec<RuleNum>> = rule.split(" | ").map(
            |sp| sp.split(' ').map(|n| n.parse::<usize>().unwrap()).collect()
        ).collect();
        match seqs.len() {
            1 => Ok(RuleEntry { id, rule: Rule::Seq(seqs.pop().unwrap()) }),
            2 => Ok(RuleEntry { id, rule: Rule::AltRule(
                        seqs.swap_remove(0), seqs.pop().unwrap()
                    ) }),
            _ => Err(())
        }
    }
}

fn parse_rules(input: &Vec<String>) -> HashMap<RuleNum, Rule> {
    input
        .iter()
        .map(|line| line.parse::<RuleEntry>().unwrap())
        .map(|re| (re.id, re.rule))
        .collect()
}

fn mkregex_id(id: RuleNum, rules: &HashMap<RuleNum, Rule>) -> String {
    mkregex(&rules[&id], rules)
}
fn mkregex(rule: &Rule, rules: &HashMap<RuleNum, Rule>) -> String {
    match rule {
        Rule::Char(c) => String::from_iter([c]),
        Rule::AltRule(a, b) =>
            format!("(?:(?:{})|(?:{}))", mkregex(&Rule::Seq(a.clone()), rules), mkregex(&Rule::Seq(b.clone()), rules)),
        Rule::Seq(v) =>
            v.iter().map(|n| mkregex_id(*n, rules)).collect::<Vec<_>>().join(""),
    }
}

fn part1(rules: &HashMap<RuleNum, Rule>, strings: &Vec<String>) -> usize {
    let reg = format!("^{}$", mkregex_id(0, rules));
    let regex = Regex::new(&reg).unwrap();
    strings.iter().filter(|s| regex.is_match(s)).count()
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input::<String>();
    let rules = parse_rules(&input[0]);
    println!("Part 1: {}", part1(&rules, &input[1]));
}
