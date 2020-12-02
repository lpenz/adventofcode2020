// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use regex::Regex;
use std::io::{stdin, BufRead};

fn get<'a>(m: &'a regex::Captures, name: &str) -> Result<&'a str> {
    Ok(m.name(name)
        .ok_or_else(|| anyhow!("{} not found", name))?
        .as_str())
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let re = Regex::new(r"^(?P<min>[0-9]+)-(?P<max>[0-9]+) (?P<letter>.): (?P<password>.*)$")?;
    let mut num_valid = 0;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let m = re
            .captures(&line)
            .ok_or_else(|| anyhow!("regex not matched, line {:?}", line))?;
        let min = get(&m, "min")?.parse::<usize>()?;
        let max = get(&m, "max")?.parse::<usize>()?;
        let letter = get(&m, "letter")?.parse::<char>()?;
        let password = get(&m, "password")?;
        let count = password.chars().filter(|&c| c == letter).count();
        if min <= count && count <= max {
            num_valid += 1;
        }
    }
    Ok(num_valid)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n";
    assert_eq!(process(input)?, 2);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
