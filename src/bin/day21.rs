use std::collections::{HashMap,HashSet};
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
extern crate advent_lib;
use advent_lib::read::read_input;

struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}
impl FromStr for Food {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splt = s.split(" (contains ");
        let ingredients = splt.next().unwrap().split(" ").map(|s| s.into()).collect();
        let alg = splt.next().unwrap();
        let alg2 = &alg[0 .. alg.len() - 1];
        let allergens = alg2.split(", ").map(|s| s.into()).collect();
        Ok(Self{ ingredients, allergens })
    }
}

fn bothparts(input: &Vec<Food>) -> (usize, String) {
    let mut allergen_map:HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ingredients:Vec<String> = Vec::new();
    for food in input {
        all_ingredients.extend(food.ingredients.iter().cloned());
        for al in &food.allergens {
            allergen_map.entry(al.clone())
                .and_modify(|set| *set = set.intersection(&food.ingredients).cloned().collect())
                .or_insert(food.ingredients.clone());
        }
    }
    let all_allergens:HashSet<String> = allergen_map
        .iter().map(|(_,v)| v.iter()).flatten().cloned().collect();
    let part1 = all_ingredients.iter()
        .filter(|i| !all_allergens.contains(*i))
        .count();

    let mut matched: HashMap<String,String> = HashMap::new();
    while allergen_map.len() > 0 {
        let (alg, ing) = allergen_map
            .iter()
            .filter(|(_,v)| v.len() == 1)
            .map(|(k,v)| (k, v.iter().next().unwrap().clone()))
            .next()
            .unwrap();
        let alg = alg.clone();
        allergen_map.remove(&alg);
        allergen_map.iter_mut().for_each(|(_,v)| { v.remove(&ing); });
        matched.insert(ing, alg);
    }
    let part2 = matched.iter()
        .sorted_by(|a,b| Ord::cmp(a.1, b.1))
        .map(|(k,_)| k)
        .join(",");
    (part1, part2)
}

fn main() {
    let input: Vec<Food> = read_input();
    let (p1, p2) = bothparts(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day21_test() {
        let input:Vec<Food> = test_input(include_str!("day21.testinput"));
        let (p1, p2) = bothparts(&input);
        assert_eq!(p1, 5);
        assert_eq!(p2, "mxmxvkd,sqjhc,fvjkl");
    }
}
