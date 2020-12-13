// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::io::{stdin, BufRead};

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i64> {
    let mut lines_iter = bufin.lines();
    lines_iter.next();
    let busses_str = lines_iter
        .next()
        .ok_or_else(|| anyhow!("error reading bus times"))??;
    // Using the chinese remainder theorem:
    let busses = busses_str
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| {
            if let Ok(n) = s.parse::<i64>() {
                Some(((n - i as i64) % n, n))
            } else {
                None
            }
        })
        .collect::<Vec<(i64, i64)>>();
    let big_n: i64 = busses.iter().map(|(_, n)| n).product();
    let big_n_is: Vec<i64> = busses.iter().map(|(_, n)| big_n / n).collect();
    let xis: Vec<i64> = big_n_is
        .iter()
        .enumerate()
        .map(|(i, cp)| {
            let v = *cp % busses[i].1;
            let mut xi = 1;
            loop {
                if (xi * v) % busses[i].1 == 1 {
                    break xi;
                }
                xi += 1;
            }
        })
        .collect();
    let sum: i64 = busses
        .iter()
        .enumerate()
        .map(|(i, (b, _))| b * big_n_is[i] * xis[i])
        .sum();
    Ok(sum % big_n)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"0\n7,13,x,x,59,x,31,19\n";
    assert_eq!(process(input)?, 1068781);
    let input: &[u8] = b"0\n67,7,59,61\n";
    assert_eq!(process(input)?, 754018);
    let input: &[u8] = b"0\n67,x,7,59,61\n";
    assert_eq!(process(input)?, 779210);
    let input: &[u8] = b"0\n67,7,x,59,61\n";
    assert_eq!(process(input)?, 1261476);
    let input: &[u8] = b"0\n1789,37,47,1889\n";
    assert_eq!(process(input)?, 1202161486);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
