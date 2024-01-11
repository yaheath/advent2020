#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use ya_advent_lib::read::read_input;

struct BagRule {
    color: String,
    contains: HashMap<String, usize>,
}

impl FromStr for BagRule {
    type Err = String;
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
            Err(format!("invalid input: {}", s))
        }
    }
}

struct BagTreeNode {
    contains: HashMap<String, usize>,
    contained_by: HashSet<String>,
    bag_count: RefCell<usize>,
}
impl BagTreeNode {
    fn new() -> Self {
        BagTreeNode { contained_by: HashSet::new(), contains: HashMap::new(), bag_count: RefCell::new(0) }
    }
    fn iter(&self) -> std::collections::hash_set::Iter<String> {
        self.contained_by.iter()
    }
}

fn main() {
    let input = read_input::<BagRule>();
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

    let mut traversed: HashSet<String> = HashSet::new();
    traverse_up(&bag_tree, "shiny gold".to_string(), &mut traversed);
    println!("Part 1: {}", traversed.len() - 1);

    traverse_down(&bag_tree, "shiny gold".to_string());
    println!("Part 2: {}", *(bag_tree.get(&"shiny gold".to_string()).unwrap().bag_count.borrow()) - 1);
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

fn traverse_down(bag_tree: &HashMap<String, BagTreeNode>, color: String) {
    let node = bag_tree.get(&color.to_string()).unwrap();
    let mut sum = 1;
    for (key, val) in node.contains.iter() {
        if *(bag_tree.get(key).unwrap().bag_count.borrow()) == 0 {
            traverse_down(bag_tree, key.to_string());
        }
        sum += *(bag_tree.get(key).unwrap().bag_count.borrow()) * val;
    }
    *(node.bag_count.borrow_mut()) = sum;
}
