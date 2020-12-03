// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct XY {
    x: usize,
    y: usize,
}

impl XY {
    pub fn new(x: usize, y: usize) -> XY {
        XY { x, y }
    }
}

#[derive(Default)]
struct Param {
    /// Equation parameter for this slope
    t: usize,
    /// Number of trees
    trees: u64,
}

fn process(bufin: impl BufRead) -> Result<u64> {
    let mut slope_trees = BTreeMap::new();
    slope_trees.insert(XY::new(1, 1), Param::default());
    slope_trees.insert(XY::new(3, 1), Param::default());
    slope_trees.insert(XY::new(5, 1), Param::default());
    slope_trees.insert(XY::new(7, 1), Param::default());
    slope_trees.insert(XY::new(1, 2), Param::default());
    for (y, line_opt) in bufin.lines().enumerate() {
        let line = line_opt?;
        let width = line.len();
        for (x, c) in line.chars().enumerate() {
            for (slope, param) in slope_trees.iter_mut() {
                if y == param.t * slope.y && x == (param.t * slope.x) % width {
                    if c == '#' {
                        param.trees += 1;
                    }
                    param.t += 1;
                }
            }
        }
    }
    for (slope, param) in &slope_trees {
        eprintln!("slope x {} y {} trees {}", slope.x, slope.y, param.trees);
    }
    Ok(slope_trees.values().map(|p| p.trees).product())
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n";
    assert_eq!(process(input)?, 336);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
