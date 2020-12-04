// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn re_get<'a>(m: &'a regex::Captures, name: &str) -> Result<&'a str> {
    Ok(m.name(name)
        .ok_or_else(|| anyhow!("{} not found", name))?
        .as_str())
}

#[derive(Default, Debug, Clone)]
pub struct Passport {
    entries: BTreeMap<String, String>,
}

impl Passport {
    pub fn getentry<'a>(&'a self, name: &str) -> Result<&'a String> {
        self.entries
            .get(name)
            .ok_or_else(|| anyhow!("field {} not found", name))
    }

    pub fn insert(&mut self, name: &str, value: &str) {
        self.entries.insert(name.to_string(), value.to_string());
    }

    pub fn check_year(&self, name: &str, min: i32, max: i32) -> Result<()> {
        let yr: i32 = self.getentry(name)?.parse()?;
        if yr < min || yr > max {
            return Err(anyhow!("invalid year {} for {}", yr, name));
        }
        Ok(())
    }

    pub fn check_regex(&self, name: &str, re: &Regex) -> Result<()> {
        if !re.is_match(self.getentry(name)?) {
            return Err(anyhow!("invalid {}", name));
        }
        Ok(())
    }

    pub fn analyse(&self) -> Result<()> {
        // byr: birth year
        self.check_year("byr", 1920, 2002)?;
        // iyr: issue year
        self.check_year("iyr", 2010, 2020)?;
        // eyr: expiration year
        self.check_year("eyr", 2020, 2030)?;
        // hgt: height
        lazy_static! {
            static ref HGT_RE: Regex = Regex::new(r"^(?P<value>[0-9]+)(?P<unit>(cm|in))$").unwrap();
        }
        let hgt_m = HGT_RE
            .captures(self.getentry("hgt")?)
            .ok_or_else(|| anyhow!("error matching hgt regex"))?;
        let val = re_get(&hgt_m, "value")?.parse::<i32>()?;
        match re_get(&hgt_m, "unit")? {
            "cm" => {
                if val < 150 || val > 193 {
                    return Err(anyhow!("invalid cm height {}", val));
                }
            }
            "in" => {
                if val < 59 || val > 76 {
                    return Err(anyhow!("invalid in height {}", val));
                }
            }
            u => {
                return Err(anyhow!("unknown unit {}", u));
            }
        }
        // hcl: hair color
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
        self.check_regex("hcl", &HCL_RE)?;
        // ecl: eye color
        lazy_static! {
            static ref ECL_SET: HashSet<&'static str> = {
                let mut s = HashSet::new();
                s.insert("amb");
                s.insert("blu");
                s.insert("brn");
                s.insert("gry");
                s.insert("grn");
                s.insert("hzl");
                s.insert("oth");
                s
            };
        }
        if !ECL_SET.contains(self.getentry("ecl")?.as_str()) {
            return Err(anyhow!("invalid ecl"));
        }
        // pid: self id
        lazy_static! {
            static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        self.check_regex("pid", &PID_RE)?;
        // cid: country id - ignore
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        self.analyse().is_ok()
    }
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut passport = Passport::default();
    let mut num_valid = 0;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        if line == "" {
            // passport is complete, check validity
            if passport.is_valid() {
                num_valid += 1;
            }
            passport = Passport::default();
        } else {
            // one more field line
            let inputs = line.split(' ').collect::<Vec<_>>();
            for entry in inputs {
                passport.insert(&entry[0..3], &entry[4..]);
            }
        }
    }
    if passport.is_valid() {
        num_valid += 1;
    }
    Ok(num_valid)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007\n\npid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\n";
    assert_eq!(process(input)?, 4);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let valid_passport = {
        let mut p = Passport::default();
        p.insert("byr", "1950");
        p.insert("iyr", "2015");
        p.insert("eyr", "2025");
        p.insert("hgt", "175cm");
        p.insert("hcl", "#0000ff");
        p.insert("ecl", "gry");
        p.insert("pid", "123456789");
        p
    };
    assert!(valid_passport.is_valid());
    for (name, val, valid) in &vec![
        ("byr", "1919", false),
        ("byr", "1920", true),
        ("byr", "2002", true),
        ("byr", "2003", false),
        ("iyr", "2009", false),
        ("iyr", "2010", true),
        ("iyr", "2020", true),
        ("iyr", "2021", false),
        ("eyr", "2019", false),
        ("eyr", "2020", true),
        ("eyr", "2030", true),
        ("eyr", "2031", false),
        ("hgt", "149cm", false),
        ("hgt", "150cm", true),
        ("hgt", "193cm", true),
        ("hgt", "194cm", false),
        ("hgt", "58in", false),
        ("hgt", "59in", true),
        ("hgt", "76in", true),
        ("hgt", "77in", false),
        ("hcl", "#000000", true),
        ("hcl", "#ffffff", true),
        ("hcl", "z000000", false),
        ("hcl", "#0000000", false),
        ("hcl", "#gggggg", false),
        ("ecl", "aaa", false),
        ("ecl", "amb", true),
        ("ecl", "blu", true),
        ("ecl", "brn", true),
        ("ecl", "gry", true),
        ("ecl", "grn", true),
        ("ecl", "hzl", true),
        ("ecl", "oth", true),
        ("ecl", "zzz", false),
        ("pid", "0", false),
        ("pid", "012345678", true),
        ("pid", "a12345678", false),
        ("pid", "0123456789", false),
        ("pid", "01234567", false),
    ] {
        let mut p = valid_passport.clone();
        p.insert(name, val);
        if *valid {
            assert_eq!(p.analyse()?, ());
        } else {
            assert!(p.analyse().is_err());
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
