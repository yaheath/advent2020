#[macro_use] extern crate lazy_static;
use std::collections::HashMap;
use std::iter::once;
use std::str::FromStr;
use regex::Regex;
extern crate advent2020;
use advent2020::read::read_input;

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

fn main() {
    let input: Vec<String> = read_input::<String>();
    let mut lineset: Vec<String> = Vec::new();
    let mut passports: Vec<Passport> = Vec::new();
    for line in input.iter().map(|s| s.as_str()).chain(once("")) {
        if line == "" {
            passports.push(Passport::from_str(&lineset.join(" ")).unwrap());
            lineset.clear();
        } else {
            lineset.push(line.to_string());
        }
    }
    let numvalid = passports.iter().filter(|p| p.is_valid()).count();
    println!("Part 1: {}", numvalid);
    let numvalid2 = passports.iter().filter(|p| p.is_valid_2()).count();
    println!("Part 2: {}", numvalid2);
}
