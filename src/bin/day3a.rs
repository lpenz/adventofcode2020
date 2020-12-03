// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut num_trees = 0;
    for (y, line_opt) in bufin.lines().enumerate() {
        let line = line_opt?;
        let width = line.len();
        for (x, c) in line.chars().enumerate() {
            if x == (y * 3) % width && c == '#' {
                num_trees += 1;
            }
        }
    }
    Ok(num_trees)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n";
    assert_eq!(process(input)?, 7);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
