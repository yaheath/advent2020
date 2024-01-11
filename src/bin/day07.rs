use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use lazy_static::lazy_static;
use ya_advent_lib::read::read_input;

struct BagRule {
    color: String,
    contains: HashMap<String, usize>,
}

impl FromStr for BagRule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w+ \w+) bags? contain (.*)$").unwrap();
        }
        lazy_static! {
            static ref SUBRE: Regex = Regex::new(r"(\d)+ (\w+ \w+) bag").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let color:String = caps.get(1).unwrap().as_str().to_string();
            let rest:&str = caps.get(2).unwrap().as_str();
            let mut contains:HashMap<String, usize> = HashMap::new();
            for cap in SUBRE.captures_iter(rest) {
                let n:usize = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let c:String = cap.get(2).unwrap().as_str().to_string();
                contains.insert(c, n);
            }
            Ok(BagRule {color: color, contains: contains})
        }
        else {
            Err(())
        }
    }
}

struct BagTreeNode {
    contains: HashMap<String, usize>,
    contained_by: HashSet<String>,
}
impl BagTreeNode {
    fn new() -> Self {
        BagTreeNode { contained_by: HashSet::new(), contains: HashMap::new() }
    }
    fn iter(&self) -> std::collections::hash_set::Iter<String> {
        self.contained_by.iter()
    }
}

fn setup(input: &[BagRule]) -> HashMap<String, BagTreeNode> {
    let mut bag_tree: HashMap<String, BagTreeNode> = HashMap::new();
    for br in input {
        for (key, _) in br.contains.iter() {
            let entry = bag_tree.entry(key.to_string()).or_insert(BagTreeNode::new());
            entry.contained_by.insert(br.color.to_string());
        }
        let bag_entry = bag_tree.entry(br.color.to_string()).or_insert(BagTreeNode::new());
        for (key, val) in br.contains.iter() {
            bag_entry.contains.insert(key.to_string(), *val);
        }
    }
    bag_tree
}

fn traverse_up(bag_tree: &HashMap<String, BagTreeNode>, color: String, traversed: &mut HashSet<String>) {
    if traversed.contains(&color) { return; }
    traversed.insert(color.to_string());
    if let Some(node) = bag_tree.get(&color) {
        for c in node.iter() {
            traverse_up(bag_tree, c.to_string(), traversed);
        }
    }
}

fn traverse_down(bag_tree: &HashMap<String, BagTreeNode>, color: String, bag_counts: &mut HashMap<String, usize>) {
    let node = bag_tree.get(&color.to_string()).unwrap();
    let mut sum = 1;
    for (key, val) in node.contains.iter() {
        let c = bag_counts.get(key).unwrap_or(&0);
        if *c == 0 {
            traverse_down(bag_tree, key.to_string(), bag_counts);
        }
        let c = bag_counts.get(key).unwrap_or(&0);
        sum += *c * val;
    }
    bag_counts.entry(color.to_string())
        .and_modify(|v| *v = sum)
        .or_insert(sum);
}

fn part1(bag_tree: &HashMap<String, BagTreeNode>) -> usize {
    let mut traversed: HashSet<String> = HashSet::new();
    traverse_up(&bag_tree, "shiny gold".to_string(), &mut traversed);
    traversed.len() - 1
}

fn part2(bag_tree: &HashMap<String, BagTreeNode>) -> usize {
    let mut bag_counts: HashMap<String, usize> = HashMap::new();
    traverse_down(bag_tree, "shiny gold".to_string(), &mut bag_counts);
    *(bag_counts.get(&"shiny gold".to_string()).unwrap()) - 1
}

fn main() {
    let input = read_input::<BagRule>();
    let bag_tree = setup(&input);

    println!("Part 1: {}", part1(&bag_tree));
    println!("Part 2: {}", part2(&bag_tree));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day07_test() {
        let input:Vec<BagRule> = test_input(include_str!("day07.testinput"));
        let bag_tree = setup(&input);
        assert_eq!(part1(&bag_tree), 4);
        assert_eq!(part2(&bag_tree), 32);
    }
}
