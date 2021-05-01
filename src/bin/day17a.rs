// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Xyz {
    x: i32,
    y: i32,
    z: i32,
}

impl Xyz {
    const COORDS: [Xyz; 26] = [
        Xyz {
            x: -1,
            y: -1,
            z: -1,
        },
        Xyz { x: -1, y: -1, z: 0 },
        Xyz { x: -1, y: -1, z: 1 },
        Xyz { x: -1, y: 0, z: -1 },
        Xyz { x: -1, y: 0, z: 0 },
        Xyz { x: -1, y: 0, z: 1 },
        Xyz { x: -1, y: 1, z: -1 },
        Xyz { x: -1, y: 1, z: 0 },
        Xyz { x: -1, y: 1, z: 1 },
        Xyz { x: 0, y: -1, z: -1 },
        Xyz { x: 0, y: -1, z: 0 },
        Xyz { x: 0, y: -1, z: 1 },
        Xyz { x: 0, y: 0, z: -1 },
        // Xyz{ x:0, y:0,z: 0},
        Xyz { x: 0, y: 0, z: 1 },
        Xyz { x: 0, y: 1, z: -1 },
        Xyz { x: 0, y: 1, z: 0 },
        Xyz { x: 0, y: 1, z: 1 },
        Xyz { x: 1, y: -1, z: -1 },
        Xyz { x: 1, y: -1, z: 0 },
        Xyz { x: 1, y: -1, z: 1 },
        Xyz { x: 1, y: 0, z: -1 },
        Xyz { x: 1, y: 0, z: 0 },
        Xyz { x: 1, y: 0, z: 1 },
        Xyz { x: 1, y: 1, z: -1 },
        Xyz { x: 1, y: 1, z: 0 },
        Xyz { x: 1, y: 1, z: 1 },
    ];

    pub fn new(x: i32, y: i32, z: i32) -> Xyz {
        Xyz { x, y, z }
    }

    pub fn neighs(&self) -> impl Iterator<Item = Xyz> + '_ {
        Xyz::COORDS
            .iter()
            .map(move |xyz| Xyz::new(xyz.x + self.x, xyz.y + self.y, xyz.z + self.z))
    }
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut cubes = BTreeSet::default();
    for (y, line_opt) in bufin.lines().enumerate() {
        let line = line_opt?;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cubes.insert(Xyz::new(x as i32, y as i32, 0_i32));
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
    eprintln!();
    assert_eq!(process(input)?, 112);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
