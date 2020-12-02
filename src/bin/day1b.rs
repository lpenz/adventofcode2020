// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

fn process(bufin: impl BufRead) -> Result<i64> {
    let mut numbers: BTreeSet<i64> = BTreeSet::new();
    for line in bufin.lines() {
        let n = line?.parse()?;
        for m in &numbers {
            let k = 2020 - m - n;
            if numbers.contains(&k) {
                return Ok(m * n * k);
            }
        }
        numbers.insert(n);
    }
    Err(anyhow!("numbers not found"))
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"1721\n979\n366\n299\n675\n1456\n";
    assert_eq!(process(input)?, 241861950);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
