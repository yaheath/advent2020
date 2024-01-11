use std::vec::Vec;
use peg;
use ya_advent_lib::read::read_input;

peg::parser!{
    grammar day18_parse() for str {
        pub rule expression() -> u64
            = arithmetic()

        pub rule expression2() -> u64
            = arithmetic2()

        rule _ = [' ' | '\n']*

        rule arithmetic() -> u64 = precedence!{
            l:(@) _ "+" _ r:@ { l + r }
            l:(@) _ "*" _ r:@ { l * r }
            --
            n:number() { n }
            "(" _  e:arithmetic() _ ")" { e }
        }

        rule arithmetic2() -> u64 = precedence!{
            l:(@) _ "*" _ r:@ { l * r }
            --
            l:(@) _ "+" _ r:@ { l + r }
            --
            n:number() { n }
            "(" _  e:arithmetic2() _ ")" { e }
        }

        rule number() -> u64
            = n:$(['0'..='9']+) { n.parse::<u64>().unwrap() }
    }
}

fn part1(input: &Vec<String>) -> u64 {
    input
        .iter()
        .map(|row| day18_parse::expression(row).unwrap())
        .sum()
}

fn part2(input: &Vec<String>) -> u64 {
    input
        .iter()
        .map(|row| day18_parse::expression2(row).unwrap())
        .sum()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_test() {
        assert_eq!(day18_parse::expression("8 + 9"), Ok(17));
        assert_eq!(day18_parse::expression("8 * 9"), Ok(72));
        assert_eq!(day18_parse::expression("1 + 2 * 3 + 4 * 5 + 6"), Ok(71));
        assert_eq!(day18_parse::expression("1 + (2 * 3) + (4 * (5 + 6))"), Ok(51));
        assert_eq!(day18_parse::expression(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), Ok(13632));

        assert_eq!(day18_parse::expression2("1 + 2 * 3 + 4 * 5 + 6"), Ok(231));
        assert_eq!(day18_parse::expression2("1 + (2 * 3) + (4 * (5 + 6))"), Ok(51));
        assert_eq!(day18_parse::expression2(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), Ok(23340));
    }
}

