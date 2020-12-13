// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::io::{stdin, BufRead};

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i64> {
    let mut lines_iter = bufin.lines();
    let arrival = lines_iter
        .next()
        .ok_or_else(|| anyhow!("error reading arrival time"))??
        .parse::<i64>()?;
    let busses_str = lines_iter
        .next()
        .ok_or_else(|| anyhow!("error reading bus times"))??;
    let busses = busses_str
        .split(',')
        .filter(|&x| x != "x")
        .collect::<Vec<_>>();
    let mut besttime = i64::MAX;
    let mut bestbus = i64::MAX;
    for bus_str in &busses {
        let bus = bus_str.parse::<i64>()?;
        let bustime = ((arrival / bus) + 1) * bus;
        if bustime < besttime {
            besttime = bustime;
            bestbus = bus;
        }
    }
    Ok(bestbus * (besttime - arrival))
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"939\n7,13,x,x,59,x,31,19\n";
    assert_eq!(process(input)?, 295);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
