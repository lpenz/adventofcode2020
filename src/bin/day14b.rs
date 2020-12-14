// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead};
use std::u128;

fn re_get<'a>(m: &'a regex::Captures, name: &str) -> Result<&'a str> {
    Ok(m.name(name)
        .ok_or_else(|| anyhow!("{} not found", name))?
        .as_str())
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<u128> {
    let mut masks_or: Vec<u128> = vec![];
    let mut mask_and = u128::MAX;
    let re = Regex::new(r"^mem\[(?P<index>[0-9]+)\] *= *(?P<value>[0-9]+)$")?;
    let mut mem = BTreeMap::new();
    for line_opt in bufin.lines() {
        let line = line_opt?;
        if &line[0..7] == "mask = " {
            masks_or = vec![0];
            mask_and = u128::MAX;
            let mut mask_or = 0_u128;
            for (i, c) in line[7..].chars().enumerate() {
                let m = 1_u128 << (35 - i);
                match c {
                    '0' => {}
                    '1' => {
                        mask_or |= m;
                    }
                    'X' => {
                        masks_or = masks_or
                            .iter()
                            .flat_map(|&mo| vec![mo, mo | m].into_iter())
                            .collect();
                        mask_and &= !m;
                    }
                    _ => return Err(anyhow!("invalid char {}", c)),
                }
            }
            for m in &mut masks_or {
                *m |= mask_or;
            }
        } else {
            let m = re
                .captures(&line)
                .ok_or_else(|| anyhow!("error matching regex"))?;
            let index = re_get(&m, "index")?.parse::<u128>()?;
            let value = re_get(&m, "value")?.parse::<u128>()?;
            for mo in &masks_or {
                let addr: u128 = (index & mask_and) | mo;
                // eprintln!("mem[{}] = {}", addr, value);
                mem.insert(addr, value);
            }
        }
    }
    Ok(mem.iter().map(|(_, v)| v).sum::<u128>())
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1\n";
    assert_eq!(process(input)?, 208);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
