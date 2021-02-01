// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

type Passport = BTreeSet<String>;

const FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
// cid is not necessary

pub fn valid(passport: &Passport) -> bool {
    FIELDS.iter().all(|f| passport.contains(&f.to_string()))
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut passport = BTreeSet::new();
    let mut num_valid = 0;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        if line.is_empty() {
            // passport is complete, check validity
            if valid(&passport) {
                num_valid += 1;
            }
            passport = BTreeSet::new();
        } else {
            // one more field line
            let inputs = line.split(' ').collect::<Vec<_>>();
            for entry in inputs {
                let e = entry[0..3].to_string();
                passport.insert(e);
            }
        }
    }
    if valid(&passport) {
        num_valid += 1;
    }
    Ok(num_valid)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in\n";
    assert_eq!(process(input)?, 2);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
