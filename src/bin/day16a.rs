// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use regex::Regex;
use std::io::{stdin, BufRead};

pub fn line_next(lines_iter: &mut std::io::Lines<impl BufRead>) -> Result<String> {
    lines_iter
        .next()
        .ok_or_else(|| anyhow!("error reading line"))?
        .map_err(|e| anyhow!(e))
}

fn get<'a>(m: &'a regex::Captures, name: &str) -> Result<&'a str> {
    Ok(m.name(name)
        .ok_or_else(|| anyhow!("{} not found", name))?
        .as_str())
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut lines_iter = bufin.lines();
    let rule_re = Regex::new(
        r"^(?P<name>[^:]+): (?P<range1min>[0-9]+)-(?P<range1max>[0-9]+) or (?P<range2min>[0-9]+)-(?P<range2max>[0-9]+)$",
    )?;
    let mut rules: Vec<(i32, i32)> = vec![];
    loop {
        let line = line_next(&mut lines_iter)?;
        if line.is_empty() {
            break;
        }
        let m = rule_re
            .captures(&line)
            .ok_or_else(|| anyhow!("rule regex not matched, line {:?}", line))?;
        rules.push((
            get(&m, "range1min")?.parse()?,
            get(&m, "range1max")?.parse()?,
        ));
        rules.push((
            get(&m, "range2min")?.parse()?,
            get(&m, "range2max")?.parse()?,
        ));
    }
    assert_eq!(line_next(&mut lines_iter)?, "your ticket:".to_string());
    let _myticket = line_next(&mut lines_iter)?;
    assert_eq!(line_next(&mut lines_iter)?, "".to_string());
    assert_eq!(line_next(&mut lines_iter)?, "nearby tickets:".to_string());
    let mut invalid = 0;
    for line_opt in lines_iter {
        let line = line_opt?;
        for num_str in line.split(',') {
            let num = num_str.parse()?;
            let valid = rules.iter().any(|r| r.0 <= num && num <= r.1);
            if !valid {
                invalid += num;
            }
        }
    }
    Ok(invalid)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12\n";
    assert_eq!(process(input)?, 71);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
