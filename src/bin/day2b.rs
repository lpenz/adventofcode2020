// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;
use std::io;
use std::io::BufRead;

fn get(m: &regex::Captures, name: &str) -> Result<String> {
    Ok(m.name(name)
        .ok_or_else(|| anyhow!("{} not found", name))?
        .as_str()
        .to_string())
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let re = Regex::new(r"^(?P<pos1>[0-9]+)-(?P<pos2>[0-9]+) (?P<letter>.): (?P<password>.*)$")?;
    let mut num_valid = 0;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let m = re
            .captures(&line)
            .ok_or_else(|| anyhow!("regex not matched, line {:?}", line))?;
        let pos1 = get(&m, "pos1")?.parse::<usize>()? - 1; // now zero-based
        let pos2 = get(&m, "pos2")?.parse::<usize>()? - 1;
        let letter = get(&m, "letter")?.parse::<char>()?;
        let password = get(&m, "password")?;
        let in_pos = password
            .chars()
            .enumerate()
            .filter(|&(pos, c)| c == letter && (pos == pos1 || pos == pos2))
            .count();
        if in_pos == 1 {
            num_valid += 1;
        }
    }
    Ok(num_valid)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n";
    assert_eq!(process(input)?, 1);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(io::stdin().lock())?);
    Ok(())
}
