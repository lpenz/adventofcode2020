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

pub fn mask2value(mask: u64) -> usize {
    let mut i = 0;
    let mut p = mask;
    while p & 1 == 0 {
        p >>= 1;
        i += 1;
    }
    i
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i64> {
    eprintln!("");
    let mut lines_iter = bufin.lines();
    let rule_re = Regex::new(
        r"^(?P<name>[^:]+): (?P<range1min>[0-9]+)-(?P<range1max>[0-9]+) or (?P<range2min>[0-9]+)-(?P<range2max>[0-9]+)$",
    )?;
    let mut rule_names: Vec<String> = vec![];
    let mut rule_limits: Vec<(i64, i64, i64, i64)> = vec![];
    loop {
        let line = line_next(&mut lines_iter)?;
        if line == "" {
            break;
        }
        let m = rule_re
            .captures(&line)
            .ok_or_else(|| anyhow!("rule regex not matched, line {:?}", line))?;
        rule_names.push(get(&m, "name")?.to_string());
        rule_limits.push((
            get(&m, "range1min")?.parse()?,
            get(&m, "range1max")?.parse()?,
            get(&m, "range2min")?.parse()?,
            get(&m, "range2max")?.parse()?,
        ));
    }
    assert_eq!(line_next(&mut lines_iter)?, "your ticket:".to_string());
    let myticket_line = line_next(&mut lines_iter)?;
    let mut myticket = vec![];
    for s in myticket_line.split(',').into_iter() {
        myticket.push(s.parse::<i64>()?);
    }
    assert_eq!(line_next(&mut lines_iter)?, "".to_string());
    assert_eq!(line_next(&mut lines_iter)?, "nearby tickets:".to_string());
    let mut possible: Vec<u64> = vec![];
    let mask = (1 << rule_names.len()) - 1;
    for _ in &rule_names {
        possible.push(mask);
    }
    for line_opt in lines_iter {
        let line = line_opt?;
        let mut valid = false;
        let mut nums = vec![];
        for num_str in line.split(',') {
            valid = false;
            let num = num_str.parse()?;
            for r in &rule_limits {
                if r.0 <= num && num <= r.1 || r.2 <= num && num <= r.3 {
                    valid = true;
                    break;
                }
            }
            if !valid {
                break;
            }
            nums.push(num);
        }
        if !valid {
            continue;
        }
        for (ipos, &num) in nums.iter().enumerate() {
            for irule in 0..rule_names.len() {
                let mask = 1 << irule;
                if possible[ipos] & mask == 0 {
                    continue;
                }
                if rule_limits[irule].0 <= num && num <= rule_limits[irule].1
                    || rule_limits[irule].2 <= num && num <= rule_limits[irule].3
                {
                    continue;
                }
                // not valid, turn off
                possible[ipos] &= !mask;
                eprintln!(
                    "line {}, num {}, ipos {}, irule {}, mask {:x}, invalid {}, possible {:x}",
                    line, num, ipos, irule, mask, rule_names[irule], possible[ipos]
                );
                assert!(possible[ipos] > 0);
                // if only one left
                let v = mask2value(possible[ipos]);
                if possible[ipos] == 1 << v {
                    eprintln!(
                        "ipos {} can only be {}, impossible in others",
                        ipos, rule_names[v]
                    );
                    // we know this field, disable in others:
                    for ipos2 in 0..rule_names.len() {
                        if ipos == ipos2 {
                            continue;
                        }
                        possible[ipos2] &= !possible[ipos];
                    }
                }
            }
        }
    }
    loop {
        let last_possible = possible.clone();
        for ipos in 0..possible.len() {
            // if only one left
            let v = mask2value(possible[ipos]);
            if possible[ipos] == 1 << v {
                eprintln!(
                    "ipos {} can only be {}, impossible in others",
                    ipos, rule_names[v]
                );
                // we know this field, disable in others:
                for ipos2 in 0..rule_names.len() {
                    if ipos == ipos2 {
                        continue;
                    }
                    possible[ipos2] &= !possible[ipos];
                }
            }
        }
        if possible == last_possible {
            break;
        }
    }
    eprintln!("possible {:x?}", possible);
    for &p in &possible {
        let v = mask2value(p);
        assert_eq!(p, 1_u64 << v);
    }
    let mut ret = 1;
    for (ipos, v) in myticket.iter().enumerate() {
        let ifield = mask2value(possible[ipos]);
        eprintln!("myticket {} = {}", rule_names[ifield], v);
        assert_eq!(1 << ifield, possible[ipos]);
        if rule_names[ifield].starts_with("departure") {
            ret *= v;
        }
    }
    Ok(ret)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9";
    assert_eq!(process(input)?, 1);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
