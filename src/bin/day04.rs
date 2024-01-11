use std::collections::HashMap;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_grouped_input;

const EXPECTED_FIELDS: &[&str] = &[
    "byr", // (Birth Year)
    "iyr", // (Issue Year)
    "eyr", // (Expiration Year)
    "hgt", // (Height)
    "hcl", // (Hair Color)
    "ecl", // (Eye Color)
    "pid", // (Passport ID)
    //"cid", // (Country ID)
];

struct Passport {
    data: HashMap<String, String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        EXPECTED_FIELDS.iter().all(|f| {
            self.data.contains_key(&(**f).to_string())
        })
    }
    fn is_valid_2(&self) -> bool {
        if !self.is_valid() { return false; }
        for (key, val) in &self.data {
            match key.as_str() {
                "byr" => {
                    if let Ok(v) = i32::from_str(val) {
                        if v < 1920 || v > 2002 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                },
                "iyr" => {
                    if let Ok(v) = i32::from_str(val) {
                        if v < 2010 || v > 2020 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                },
                "eyr" => {
                    if let Ok(v) = i32::from_str(val) {
                        if v < 2020 || v > 2030 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                },
                "hgt" => {
                    lazy_static! {
                        static ref RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
                    }
                    if let Some(cap) = RE.captures(val) {
                        let n = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                        let u = cap.get(2).unwrap().as_str();
                        if u == "in" {
                            if n < 59 || n > 76 {
                                return false;
                            }
                        } else {
                            if n < 150 || n > 193 {
                                return false;
                            }
                        }
                    } else {
                        return false;
                    }
                },
                "hcl" => {
                    lazy_static! {
                        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                    }
                    if !RE.is_match(val) {
                        return false;
                    }
                },
                "ecl" => {
                    lazy_static! {
                        static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                    }
                    if !RE.is_match(val) {
                        return false;
                    }
                },
                "pid" => {
                    lazy_static! {
                        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
                    }
                    if !RE.is_match(val) {
                        return false;
                    }
                },
                _ => {},
            }
        }
        true
    }
}

impl FromStr for Passport {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w{3}):(\S+)").unwrap();
        }
        let mut map: HashMap<String, String> = HashMap::new();
        for cap in RE.captures_iter(s) {
            let key:String = cap.get(1).unwrap().as_str().to_string();
            let val:String = cap.get(2).unwrap().as_str().to_string();
            map.insert(key, val);
        }
        if map.len() > 0 {
            Ok(Passport {data: map})
        }
        else {
            Err(format!("invalid input: {}", s))
        }
    }
}

fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid()).count()
}

fn part2(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid_2()).count()
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input();
    let input = input.into_iter()
        .map(|lineset| Passport::from_str(&lineset.join(" ")).unwrap())
        .collect::<Vec<_>>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day04_test() {
        let input:Vec<Vec<String>> = grouped_test_input(include_str!("day04.testinput"));
        let input = input.into_iter()
            .map(|lineset| Passport::from_str(&lineset.join(" ")).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 2);

        for i in [
"eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
"iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
"hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
"hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
        ] {
            let p = i.parse::<Passport>().unwrap();
            assert_eq!(p.is_valid_2(), false);
        }

        for i in [
"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
"eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
"hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
"iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ] {
            let p = i.parse::<Passport>().unwrap();
            assert_eq!(p.is_valid_2(), true);
        }
    }
}
