// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut group = BTreeSet::new();
    let mut ans = 0;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        if line.is_empty() {
            // group is complete, sum answers
            ans += group.len();
            group = BTreeSet::new();
        } else {
            for c in line.chars() {
                group.insert(c);
            }
        }
    }
    // add last group
    ans += group.len();
    Ok(ans)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n";
    assert_eq!(process(input)?, 11);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
