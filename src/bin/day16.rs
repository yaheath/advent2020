use std::collections::HashSet;
use std::ops::Range;
use std::vec::Vec;
use std::str::FromStr;
extern crate advent_lib;
use advent_lib::range::range_from_str;
use advent_lib::read::read_grouped_input;

#[derive(Clone, Debug)]
struct Rule {
    name: String,
    valid: [Range<u64>; 2],
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split(": ");
        let name = spl.next().unwrap();
        let mut spl = spl.next().unwrap().split(" or ");
        let range1 = range_from_str(spl.next().unwrap()).unwrap();
        let range2 = range_from_str(spl.next().unwrap()).unwrap();
        Ok(Self {
            name: name.into(),
            valid: [range1, range2],
        })
    }
}

#[derive(Clone, Debug)]
struct Ticket {
    vals: Vec<u64>,
}

impl FromStr for Ticket {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            vals: s.split(',').map(|s| s.parse::<u64>().unwrap()).collect(),
        })
    }
}

struct Input {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    tickets: Vec<Ticket>,
}

fn setup(input: &Vec<Vec<String>>) -> Input {
    let rules = input[0].iter().map(|s| s.parse::<Rule>().unwrap()).collect();
    let my_ticket = input[1][1].parse::<Ticket>().unwrap();
    let tickets = input[2].iter().skip(1).map(|s| s.parse::<Ticket>().unwrap()).collect();
    Input {
        rules,
        my_ticket,
        tickets,
    }
}

fn part1(input: &Input) -> u64 {
    let valid_ranges: Vec<Range<u64>> = input.rules
        .iter()
        .flat_map(|rule| rule.valid.iter())
        .cloned()
        .collect();
    input.tickets
        .iter()
        .flat_map(|row| row.vals.iter())
        .filter(|val| !valid_ranges.iter().any(|r| r.contains(val)))
        .sum()
}

fn part2(input: &Input) -> u64 {
    let valid_ranges: Vec<Range<u64>> = input.rules
        .iter()
        .flat_map(|rule| rule.valid.iter())
        .cloned()
        .collect();
    let tickets: Vec<Ticket> = input.tickets
        .iter()
        .filter(|row| row.vals.iter().all(|val|
            valid_ranges.iter().any(|r| r.contains(val))
        ))
        .cloned()
        .collect();

    let mut columns: Vec<HashSet<u64>> = Vec::with_capacity(input.my_ticket.vals.len());
    for _ in 0..input.my_ticket.vals.len() {
        columns.push(HashSet::new());
    }

    tickets
        .iter()
        .flat_map(|row| row.vals.iter().enumerate())
        .for_each(|(idx, val)| {
            columns[idx].insert(*val);
        });

    let mut matches:Vec<(&String, Vec<usize>)> = input.rules
        .iter()
        .map(|r| {
            (
                &r.name,
                columns.iter()
                    .enumerate()
                    .filter(|(_, set)|
                        set.iter()
                        .all(|v| r.valid[0].contains(v) || r.valid[1].contains(v))
                    )
                    .map(|(idx, _)| idx)
                    .collect::<Vec<usize>>(),
            )
        })
        .collect();

    let mut eliminated: HashSet<usize> = HashSet::new();
    while let Some(n) = matches.iter()
            .find(|(_, v)| v.len() == 1 && !eliminated.contains(&v[0]))
            .map(|(_, v)| v[0]) {
        for (_, v) in matches.iter_mut() {
            if v.len() > 1 {
                if let Ok(idx) = v.binary_search(&n) {
                    v.splice(idx..=idx, []);
                }
            }
        }
        eliminated.insert(n);
    }
    assert!(matches.iter().all(|(_, v)| v.len() == 1));
    matches.iter()
        .filter(|(n,_)| n.starts_with("departure"))
        .map(|(_,v)| input.my_ticket.vals[v[0]])
        .product()
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input();
    let input = setup(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

