// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut numbers: BTreeSet<i32> = BTreeSet::new();
    for line in bufin.lines() {
        let n = line?.parse()?;
        let m = 2020 - n;
        if numbers.contains(&m) {
            return Ok(m * n);
        }
        numbers.insert(n);
    }
    Err(anyhow!("numbers not found"))
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"1721\n979\n366\n299\n675\n1456\n";
    assert_eq!(process(input)?, 514579);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
