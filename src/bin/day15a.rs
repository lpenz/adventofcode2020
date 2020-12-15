// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead};

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut births = BTreeMap::new();
    let mut i = 0;
    let mut next = 0;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        for num_str in line.split(',') {
            let num = num_str.parse::<i32>()?;
            let birth_opt = births.insert(num, i);
            if let Some(birth) = birth_opt {
                next = i - birth + 1;
            } else {
                next = 0;
            }
            i += 1;
        }
    }
    while i != 2019 {
        let birth_opt = births.insert(next, i);
        if let Some(birth) = birth_opt {
            next = i - birth;
        } else {
            next = 0;
        }
        i += 1;
    }
    Ok(next)
}

#[test]
fn test1() -> Result<()> {
    let input: &[u8] = b"0,3,6\n";
    assert_eq!(process(input)?, 436);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let input: &[u8] = b"1,3,2\n";
    assert_eq!(process(input)?, 1);
    Ok(())
}

#[test]
fn test3() -> Result<()> {
    let input: &[u8] = b"2,1,3\n";
    assert_eq!(process(input)?, 10);
    Ok(())
}

#[test]
fn test4() -> Result<()> {
    let input: &[u8] = b"1,2,3\n";
    assert_eq!(process(input)?, 27);
    Ok(())
}

#[test]
fn test5() -> Result<()> {
    let input: &[u8] = b"2,3,1\n";
    assert_eq!(process(input)?, 78);
    Ok(())
}

#[test]
fn test6() -> Result<()> {
    let input: &[u8] = b"3,2,1\n";
    assert_eq!(process(input)?, 438);
    Ok(())
}

#[test]
fn test7() -> Result<()> {
    let input: &[u8] = b"3,1,2\n";
    assert_eq!(process(input)?, 1836);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
