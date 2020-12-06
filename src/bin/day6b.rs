// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

pub fn gt0(v: u32) -> usize {
    if v > 0 {
        1
    } else {
        0
    }
}

pub fn char_to_mask(c: char) -> u32 {
    1 << (c as u32 - 'a' as u32)
}

pub fn bitsum(v: u32) -> usize {
    ('a'..='z').map(|c| gt0(v & char_to_mask(c))).sum()
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut group: u32 = u32::MAX;
    let mut ans = 0;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        if line == "" {
            // group is complete, sum answers
            ans += bitsum(group);
            group = u32::MAX;
        } else {
            let mask = line.chars().fold(0_u32, |acc, c| acc | char_to_mask(c));
            group &= mask;
        }
    }
    // add last group
    ans += bitsum(group);
    Ok(ans)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n";
    assert_eq!(process(input)?, 6);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
