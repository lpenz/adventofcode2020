// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{Error, Result};
use std::io::{stdin, BufRead};
use std::str::FromStr;

/// Only keep the max value inside the give Option
pub fn max_set<T: Ord>(max: &mut Option<T>, item: T) {
    if let Some(v) = max {
        if *v < item {
            *max = Some(item);
        }
    } else {
        *max = Some(item);
    }
}

/// Boarding pass
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BPass {
    num: u32,
}

impl BPass {
    pub fn new(num: u32) -> BPass {
        BPass { num }
    }
}

impl FromStr for BPass {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num = 0;
        for (i, c) in s.chars().enumerate() {
            let letter = if i < 7 { 'B' } else { 'R' };
            let v = if c == letter { 1 << (9 - i) } else { 0 };
            num |= v;
        }
        Ok(BPass::new(num))
    }
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let mut max = None;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let bpass = line.parse::<BPass>()?;
        max_set(&mut max, bpass);
    }
    Ok(max.unwrap().num)
}

#[test]
fn test() -> Result<()> {
    let input = "BFFFBBFRRR\n";
    assert_eq!(input.parse::<BPass>()?, BPass::new(567));
    let input = "FFFBBBFRRR\n";
    assert_eq!(input.parse::<BPass>()?, BPass::new(119));
    let input = "BBFFBBFRLL\n";
    assert_eq!(input.parse::<BPass>()?, BPass::new(820));
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let input: &[u8] = b"BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL\n";
    assert_eq!(process(input)?, 820);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
