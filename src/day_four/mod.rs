extern crate regex;

use regex::Regex;
use std::fs;

pub fn solve_day_four() {
    let contents = fs::read_to_string("inputs/day-four.txt").expect("Oh no");
    let passports = contents
        .split("\n\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    println!(
        "Part one: there are {} valid passports",
        solve_part_one(&passports)
    );
    println!(
        "Part two: there are {} valid passports",
        solve_part_two(&passports)
    );
}

fn solve_part_one(passports: &Vec<String>) -> usize {
    let required_fields = vec!["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    passports
        .iter()
        .filter(|passport| required_fields.iter().all(|field| passport.contains(field)))
        .count()
}

fn solve_part_two(passports: &Vec<String>) -> usize {
    let validators: Vec<&dyn Validator> = vec![
        &BirthYear {} as &dyn Validator,
        &IssueYear {} as &dyn Validator,
        &ExpirationYear {} as &dyn Validator,
        &Height {} as &dyn Validator,
        &HairColor {} as &dyn Validator,
        &EyeColor {} as &dyn Validator,
        &PassportID {} as &dyn Validator,
    ];

    return passports
        .iter()
        .filter(|passport| {
            validators
                .iter()
                .all(|validator| validator.is_valid(passport.to_string()))
        })
        .count();
}

trait Validator {
    fn is_valid(&self, input: String) -> bool;
}

// A BirthYear matches the value byr: followed by a four digit
// number that is at least 1920 and at most 2002.
struct BirthYear {}
impl Validator for BirthYear {
    fn is_valid(&self, input: String) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\bbyr:(?P<year>\d{4})\b").unwrap();
        }

        match RE.captures(&input[..]) {
            None => false,
            Some(captures) => {
                let year = captures["year"].parse::<i32>().unwrap();
                year >= 1920 && year <= 2002
            }
        }
    }
}

// An IssueYear matches the value iyr: followed by a four digit
// number that is at least 2010 and at most 2020.
struct IssueYear {}
impl Validator for IssueYear {
    fn is_valid(&self, input: String) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\biyr:(?P<year>\d{4})\b").unwrap();
        }

        match RE.captures(&input[..]) {
            None => false,
            Some(captures) => {
                let year = captures["year"].parse::<i32>().unwrap();
                year >= 2010 && year <= 2020
            }
        }
    }
}

// An ExpirationYear matches the value eyr: followed by a four digit
// number that is at least 2020 and at most 2030.
struct ExpirationYear {}
impl Validator for ExpirationYear {
    fn is_valid(&self, input: String) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\beyr:(?P<year>\d{4})\b").unwrap();
        }

        match RE.captures(&input[..]) {
            None => false,
            Some(captures) => {
                let year = captures["year"].parse::<i32>().unwrap();
                year >= 2020 && year <= 2030
            }
        }
    }
}

// A Height matches the value hgt: followed by a number and either cm or
// in.  If cm, then the number is in the range 150-193, and if in, then
// in the range 59-76.
struct Height {}
impl Validator for Height {
    fn is_valid(&self, input: String) -> bool {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\bhgt:(?P<height>\d{2,3})(?P<unit>cm|in)\b").unwrap();
        }

        match RE.captures(&input[..]) {
            None => false,
            Some(captures) => {
                let height = captures["height"].parse::<i32>().unwrap();
                let unit = String::from(&captures["unit"]);

                if unit == "cm" {
                    return height >= 150 && height <= 193;
                } else {
                    return height >= 59 && height <= 76;
                }
            }
        }
    }
}

// An EyeColor matches the value ecl: follwed by
// exactly one of amb, blu, brn, gry, grn, hzl, or oth.
struct EyeColor {}
impl Validator for EyeColor {
    fn is_valid(&self, input: String) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
        }

        match RE.captures(&input[..]) {
            None => false,
            Some(_) => true,
        }
    }
}

// A HairColor matches the value hcl: followed by
// a # and exactly six characters from a-f or 0-9.
struct HairColor {}
impl Validator for HairColor {
    fn is_valid(&self, input: String) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\bhcl:#([a-f0-9]{6})\b").unwrap();
        }

        match RE.captures(&input[..]) {
            None => false,
            Some(_) => true,
        }
    }
}

// A PassportID matches the value pid: followed by
// a nine-digit number including leading zeroes.
struct PassportID {}
impl Validator for PassportID {
    fn is_valid(&self, input: String) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\bpid:\d{9}\b").unwrap();
        }

        match RE.captures(&input[..]) {
            None => false,
            Some(_) => true,
        }
    }
}
