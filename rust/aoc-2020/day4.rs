use aoc_runner_derive::aoc;
use itertools::Itertools;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use validator::{Validate, ValidationError};
use std::str::FromStr;
use std::fmt;
use regex::Regex;

lazy_static! {
    static ref RE_HAIR_COLOR: Regex = Regex::new(r"^\\\#[0-9a-f]{6}$").unwrap();
    static ref RE_EYE_COLOR: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref RE_PASSPORT_ID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

#[derive(Debug, Eq, PartialEq)]
enum Unit {
    Cm,
    In
}
impl FromStr for Unit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Unit, &'static str> {
        match s {
            "cm" => Ok(Unit::Cm),
            "in" => Ok(Unit::In),
            _ => Err("Not a valid unit!"),
        }
    }
}
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Cm => write!(f, "cm"),
            Unit::In => write!(f, "in"),
        }
    }
}
/// I really didn't need this crap for the height, since I need a custom
/// validator anyhow :rolling_eyes:...
#[derive(Debug, Eq, PartialEq, Validate)]
#[validate(schema(function = "validate_height"))]
struct Height {
    hgt: u8,
    unit: Unit,
}
fn validate_height(h: &Height) -> Result<(), ValidationError> {
    match h {
        Height { hgt: 150..=193, unit: Unit::Cm } => Ok(()),
        Height { unit: Unit::Cm, .. } => Err(ValidationError::new("Height invalid for cm!")),
        Height { hgt: 59..=76, unit: Unit::In } => Ok(()),
        Height { unit: Unit::In, .. } => Err(ValidationError::new("Height invalid for in!")),
    }
}
impl FromStr for Height {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Height, &'static str> {
        match s.find(|c:char| c.is_ascii_alphabetic()) {
            Some(unit_idx) => {
                let (h, u) = s.split_at(unit_idx);
                match Unit::from_str(u) {
                    Ok(unit) => {
                        match h.parse::<u8>() {
                            Ok(hgt) => Ok(Height { hgt, unit }),
                            Err(_) => Err("Invalid number in height!"),
                        }
                    },
                    Err(e) => Err(e)
                }
            },
            None => Err("No unit in height string!")
        }
    }
}
impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.hgt, self.unit)
    }
}
impl Serialize for Height {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct HeightVisitor;

impl<'de> Visitor<'de> for HeightVisitor {
    type Value = Height;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 59 and 193 and a unit (cm|in)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Height::from_str(value).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Height {
    fn deserialize<D>(deserializer: D) -> Result<Height, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HeightVisitor)
    }
}
    
#[derive(Debug, PartialEq, Validate, Serialize, Deserialize)]
// #[validate(schema(function = "validate_passport"))]
struct Passport {
    // four digits; at least 1920 and at most 2002.
    #[validate(range(min = 1920, max = 2002))]
    byr: u16, //(Birth Year)
    // four digits; at least 2010 and at most 2020.
    #[validate(range(min = 2010, max = 2020))]
    iyr: u16, //(Issue Year)
    // four digits; at least 2020 and at most 2030.
    #[validate(range(min = 2020, max = 2030))]
    eyr: u16, //(Expiration Year)
    // a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    #[validate]
    hgt: Height, //(Height)
    // a # followed by exactly six characters 0-9 or a-f.
    // #[validate(custom(function = "validate_hcl"))]
    #[validate(regex = "RE_HAIR_COLOR")]
    hcl: String, //(Hair Color)
    // exactly one of: amb blu brn gry grn hzl oth.
    // #[validate(custom(function = "validate_ecl"))]
    #[validate(regex = "RE_EYE_COLOR")]
    ecl: String, //(Eye Color)
    // a nine-digit number, including leading zeroes.
    // #[validate(custom(function = "validate_pid"))]
    #[validate(regex = "RE_PASSPORT_ID")]
    pid: String, //(Passport ID)
    // ignored, missing or not.
    // cid: Option<&str>, //(Country ID)
}

fn validate_passport(pp: &Passport) -> Result<(), ValidationError> {
    println!("{:?}", pp);
    let Passport { hcl, ecl, pid, .. } = pp;
    let mut err_str: &'static str = "";
    if !RE_HAIR_COLOR.is_match(hcl) {
        println!("HAIRCOLOR::{:?}", hcl);
        err_str = "hcl problem";
    }
    if !RE_EYE_COLOR.is_match(ecl) {
        println!("EYECOLOR::{:?}", ecl);
        err_str = "ecl problem";
    }
    if !RE_PASSPORT_ID.is_match(pid) {
        println!("PASSIDNU::{:?}", pid);
        err_str = "pid problem";
    }
    if err_str == "" {
        return Ok(())
    }
    Err(ValidationError::new(err_str))
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SimplePassport {
    byr: String, //(Birth Year)
    iyr: String, //(Issue Year)
    eyr: String, //(Expiration Year)
    hgt: String, //(Height)
    hcl: String, //(Hair Color)
    ecl: String, //(Eye Color)
    pid: String, //(Passport ID)
    cid: Option<String>, //(Country ID)
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s| s
            .replace(char::is_whitespace, "\n")
            .replace(':', ": ")
            .replace('#', "\\#"))
        .map(|s| -> serde_yaml::Result<SimplePassport> { serde_yaml::from_str(&s) })
        .filter(|p| p.is_ok())
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let mut ct = 0;
    input
        .split("\n\n")
        .map(|s| s.replace(char::is_whitespace, "\n")
        .replace(':', ": ")
        .replace('#', "\\#"))
        .filter(|s| serde_yaml::from_str::<SimplePassport>(s).is_ok())
        .filter(|s| {
            for l in s.lines() {
                if let Some((key, val)) = l.split(": ").collect_tuple() {
                    match key {
                        "byr" if val.parse().unwrap_or(1919) >= 1920 && val.parse().unwrap_or(2003) <= 2002 => continue,
                        "byr" => {
                            println!("BYR:{}", val);
                            return false
                        },
                        "iyr" if val.parse().unwrap_or(2009) >= 2010 && val.parse().unwrap_or(2021) <= 2020 => continue,
                        "iyr" => {
                            println!("IYR:{}", val);
                            return false
                        },
                        "eyr" if val.parse().unwrap_or(2019) >= 2020 && val.parse().unwrap_or(2031) <= 2030 => continue,
                        "eyr" => {
                            println!("EYR:{}", val);
                            return false
                        },
                        "hgt" => {
                            match Height::from_str(val) {
                                Ok(h) => {
                                    match validate_height(&h) {
                                        Ok(_) => continue,
                                        Err(_) => {
                                            println!("HGT:{}", val);
                                            return false
                                        },
                                    }
                                },
                                Err(_) =>  {
                                    println!("HGT:{}", val);
                                    return false
                                },
                            }
                        },
                        "hcl" if RE_HAIR_COLOR.is_match(val) => continue,
                        "hcl" => {
                            println!("hcl:{}", val);
                            return false
                        },
                        "ecl" if RE_EYE_COLOR.is_match(val) => continue,
                        "ecl" => {
                            println!("ecl:{}", val);
                            return false
                        },
                        "pid" if RE_PASSPORT_ID.is_match(val) => continue,
                        "pid" => {
                            println!("pid:{}", val);
                            return false
                        },
                        "cid" => continue,
                        _ => return false,
                    }
                } 
            }
            true
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    const INPUT2: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT1), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT2), 4);
    }
}
