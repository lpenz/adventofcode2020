// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead};
use std::u64;

fn re_get<'a>(m: &'a regex::Captures, name: &str) -> Result<&'a str> {
    Ok(m.name(name)
        .ok_or_else(|| anyhow!("{} not found", name))?
        .as_str())
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<u64> {
    let mut mask_or = 0_u64;
    let mut mask_and = u64::MAX;
    let re = Regex::new(r"^mem\[(?P<index>[0-9]+)\] *= *(?P<value>[0-9]+)$")?;
    let mut mem = BTreeMap::new();
    for line_opt in bufin.lines() {
        let line = line_opt?;
        if &line[0..7] == "mask = " {
            mask_or = 0_u64;
            mask_and = u64::MAX;
            for (i, c) in line[7..].chars().enumerate() {
                let m = 1_u64 << (35 - i);
                match c {
                    '0' => {
                        mask_and &= !m;
                    }
                    '1' => {
                        mask_or |= m;
                    }
                    'X' => {}
                    _ => return Err(anyhow!("invalid char {}", c)),
                }
            }
        } else {
            let m = re
                .captures(&line)
                .ok_or_else(|| anyhow!("error matching regex"))?;
            let index = re_get(&m, "index")?.parse::<u64>()?;
            let value0 = re_get(&m, "value")?.parse::<u64>()?;
            let value = (value0 | mask_or) & mask_and;
            mem.insert(index, value);
        }
    }
    Ok(mem.iter().map(|(_, v)| v).sum())
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] =
        b"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0\n";
    assert_eq!(process(input)?, 165);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
