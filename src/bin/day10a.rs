// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::collections::BinaryHeap;
use std::io::{stdin, BufRead};

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut nums = BinaryHeap::new();
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let num: i32 = line.parse()?;
        nums.push(num);
    }
    let mut dif1 = 0;
    let mut dif3 = 1; // my port
    let mut curr = 0;
    for num in nums.into_sorted_vec().into_iter() {
        match num - curr {
            1 => {
                dif1 += 1;
            }
            3 => {
                dif3 += 1;
            }
            _ => {
                return Err(anyhow!("unsupported diff {}", num - curr));
            }
        }
        curr = num;
    }
    Ok(dif1 * dif3)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n";
    assert_eq!(process(input)?, 7 * 5);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let input: &[u8] = b"28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n";
    assert_eq!(process(input)?, 22 * 10);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
