use std::collections::{HashSet, VecDeque};
use std::vec::Vec;
extern crate advent_lib;
use advent_lib::read::read_grouped_input;

enum Winner {
    P1,
    P2,
}

fn recursive_game(mut p1deck: VecDeque<usize>, mut p2deck: VecDeque<usize>) -> (Winner, VecDeque<usize>) {
    let mut states: HashSet<(VecDeque<usize>,VecDeque<usize>)> = HashSet::new();

    loop {
        let card1 = p1deck.pop_front().unwrap();
        let card2 = p2deck.pop_front().unwrap();
        let winner;

        if card1 <= p1deck.len() && card2 <= p2deck.len() {
            let newp1deck = p1deck.iter().take(card1).cloned().collect();
            let newp2deck = p2deck.iter().take(card2).cloned().collect();
            let result = recursive_game(newp1deck, newp2deck);
            winner = result.0;
        }
        else {
            winner = if card1 > card2 {Winner::P1} else {Winner::P2};
        }

        match winner {
            Winner::P1 => {
                p1deck.push_back(card1);
                p1deck.push_back(card2);
            },
            Winner::P2 => {
                p2deck.push_back(card2);
                p2deck.push_back(card1);
            },
        }

        if p1deck.len() == 0 {
            return (Winner::P2, p2deck);
        }
        if p2deck.len() == 0 {
            return (Winner::P1, p1deck);
        }

        let state = (p1deck.clone(), p2deck.clone());
        if states.contains(&state) {
            return (Winner::P1, p1deck);
        }
        states.insert(state);
    }
}

fn part1(input: &Vec<Vec<String>>) -> usize {
    let mut p1deck: VecDeque<usize> = input[0].iter()
        .skip(1).map(|l| l.parse::<usize>().unwrap()).collect();
    let mut p2deck: VecDeque<usize> = input[1].iter()
        .skip(1).map(|l| l.parse::<usize>().unwrap()).collect();
    let winner;
    loop {
        let card1 = p1deck.pop_front().unwrap();
        let card2 = p2deck.pop_front().unwrap();
        if card1 > card2 {
            p1deck.push_back(card1);
            p1deck.push_back(card2);
        } else {
            p2deck.push_back(card2);
            p2deck.push_back(card1);
        }
        if p1deck.len() == 0 {
            winner = p2deck;
            break;
        }
        if p2deck.len() == 0 {
            winner = p1deck;
            break;
        }
    }
    winner.iter().rev().enumerate()
        .map(|(idx, card)| card * (idx+1))
        .sum()
}

fn part2(input: &Vec<Vec<String>>) -> usize {
    let p1deck: VecDeque<usize> = input[0].iter()
        .skip(1).map(|l| l.parse::<usize>().unwrap()).collect();
    let p2deck: VecDeque<usize> = input[1].iter()
        .skip(1).map(|l| l.parse::<usize>().unwrap()).collect();
    let (_, deck) = recursive_game(p1deck, p2deck);
    deck.iter().rev().enumerate()
        .map(|(idx, card)| card * (idx+1))
        .sum()
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::grouped_test_input;

    #[test]
    fn day22_test() {
        let input:Vec<Vec<String>> = grouped_test_input(include_str!("day22.testinput"));
        assert_eq!(part1(&input), 306);
        assert_eq!(part2(&input), 291);
    }
}
