use regex::Regex;
use crate::advent;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Field {
    BYR(String),
    IYR(String),
    EYR(String),
    HGT(String),
    HCL(String),
    ECL(String),
    PID(String),
    CID(String) // Ignored missing or not
}

impl Field {
    fn valid(&self) -> bool {
        match self {
            Field::BYR(val) => {
                val.len() == 4 &&
                match scan_fmt!(val, "{4d}", i32) {
                    Ok(year) => year >= 1920 && year <= 2002,
                    Err(_)   => false
                }
            },
            Field::IYR(val) => {
                val.len() == 4 &&
                match scan_fmt!(val, "{4d}", i32) {
                    Ok(year) => year >= 2010 && year <= 2020,
                    Err(_)   => false
                }
            },
            Field::EYR(val) => {
                val.len() == 4 &&
                match scan_fmt!(val, "{4d}", i32) {
                    Ok(year) => year >= 2020 && year <= 2030,
                    Err(_)   => false
                }
            },
            Field::HGT(val) => {
                match scan_fmt!(val, "{d}{}", i32, String) {
                    Ok((height, unit)) => match unit.as_str() {
                        "cm" => height >= 150 && height <= 193,
                        "in" => height >= 59 && height <= 76,
                        _ => false
                    }
                    Err(_)   => false
                }
            },
            Field::HCL(val) => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"#[a-f0-9]{6}").unwrap();
                }
                RE.is_match(val)
            },
            Field::ECL(val) => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"((amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth))").unwrap();
                }
                RE.is_match(val)
            },
            Field::PID(val) => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"[0-9]{9}").unwrap();
                }
                val.len() == 9 &&
                RE.is_match(val)
            },
            _ => true
        }
    }
    
}

type Passport = Vec<Field>;

pub const DAY04A: advent::Sol<Vec<Passport>, i32> = advent::Sol {
    day: 4,
    part: advent::Part::Part1,
    parse: parse,
    solve: solve1,
    show: i32::to_string,
};

pub const DAY04B: advent::Sol<Vec<Passport>, i32> = advent::Sol {
    day: 4,
    part: advent::Part::Part2,
    parse: parse,
    solve: solve2,
    show: i32::to_string,
};

fn solve2(passports: Vec<Passport>) -> i32 {
    passports.into_iter().filter(all_present).filter(valid).count() as i32
}

fn valid(passport: &Passport) -> bool {
    passport.into_iter().all(|f| f.valid())
}


fn solve1(passports: Vec<Passport>) -> i32 {
    passports.into_iter().filter(all_present).count() as i32
}

fn all_present(passport: &Passport) -> bool {
    passport.into_iter()
        .filter(|f| match f {Field::CID(_) => false, _ => true}) // Ignore CID
        .count() as i32 == 7 // Are all other fields present?
}

fn parse(input: String) -> Vec<Passport> {
    input.trim_end().split("\n\n").into_iter()
        .map(|x| x.replace("\n", " "))
        .map(parse_passport)
        .collect()
}

fn parse_passport(line: String) -> Passport {
    line.split(" ").into_iter()
        .map(parse_field)
        .collect()
}

fn parse_field(s: &str) -> Field {
    let mut iter = s.split(':');
    let name = iter.next().unwrap();
    let val  = iter.next().unwrap().to_string();
    match name {
        "byr" => Field::BYR(val),
        "iyr" => Field::IYR(val),
        "eyr" => Field::EYR(val),
        "hgt" => Field::HGT(val),
        "hcl" => Field::HCL(val),
        "ecl" => Field::ECL(val),
        "pid" => Field::PID(val),
        "cid" => Field::CID(val),
        _ => panic!("unknown field name: {}", name)
    }
}