// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io;
use std::io::BufRead;

use std::collections::BTreeSet;

fn main() -> Result<()> {
    let mut numbers: BTreeSet<u128> = BTreeSet::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        numbers.insert(line?.parse()?);
    }
    for n in &numbers {
        let m = 2020 - n;
        if !numbers.contains(&m) {
            continue;
        }
        println!("{}", m * n);
        break;
    }
    Ok(())
}
