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
    while i != 30000000 - 1 {
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

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
