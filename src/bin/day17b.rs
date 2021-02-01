// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use itertools::iproduct;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct XYZW {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

lazy_static! {
    static ref COORDS: Vec<XYZW> = {
        iproduct!(0..3, 0..3, 0..3, 0..3)
            .filter_map(|(a, b, c, d)| {
                if a == 1 && b == 1 && c == 1 && d == 1 {
                    None
                } else {
                    Some(XYZW::new(a - 1, b - 1, c - 1, d - 1))
                }
            })
            .collect()
    };
}

impl XYZW {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> XYZW {
        XYZW { x, y, z, w }
    }

    pub fn neighs(&self) -> impl Iterator<Item = XYZW> + '_ {
        COORDS.iter().map(move |xyzw| {
            XYZW::new(
                xyzw.x + self.x,
                xyzw.y + self.y,
                xyzw.z + self.z,
                xyzw.w + self.w,
            )
        })
    }
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut cubes = BTreeSet::default();
    for (y, line_opt) in bufin.lines().enumerate() {
        let line = line_opt?;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cubes.insert(XYZW::new(x as i32, y as i32, 0_i32, 0_i32));
            }
        }
    }
    for _ in 0..6 {
        let mut neighs = BTreeMap::default();
        for cube in &cubes {
            for neigh in cube.neighs() {
                let e = neighs.entry(neigh).or_insert(0);
                *e += 1;
            }
        }
        let mut newcubes = BTreeSet::default();
        for (xyz, n) in neighs {
            if n == 3 || n == 2 && cubes.contains(&xyz) {
                newcubes.insert(xyz);
            }
        }
        cubes = newcubes;
    }
    Ok(cubes.len())
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b".#.\n..#\n###\n";
    eprintln!("");
    assert_eq!(process(input)?, 848);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
