// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead};

pub fn dfs(cache: &mut BTreeMap<usize, i64>, target: i64, nums: &[i64]) -> i64 {
    if let Some(val) = cache.get(&nums.len()) {
        return *val;
    }
    if nums.len() == 1 {
        return if nums[0] == target { 1 } else { 0 };
    }
    let mut total = 0;
    for next in 1..=3 {
        if next < nums.len() && nums[next] - nums[0] <= 3 {
            total += dfs(cache, target, &nums[next..]);
        }
    }
    cache.insert(nums.len(), total);
    total
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i64> {
    let mut nums = vec![0];
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let num: i64 = line.parse()?;
        nums.push(num);
    }
    nums.sort_unstable();
    let last = nums[nums.len() - 1] + 3;
    nums.push(last);
    let mut cache = BTreeMap::new();
    Ok(dfs(&mut cache, last, &nums))
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n";
    assert_eq!(process(input)?, 8);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let input: &[u8] = b"28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n";
    assert_eq!(process(input)?, 19208);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
