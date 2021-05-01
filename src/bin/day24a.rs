// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};
use std::ops;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Dir {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Dir {
    pub fn new(x: i8, y: i8, z: i8) -> Dir {
        Dir { x, y, z }
    }
}

impl FromStr for Dir {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ne" => Dir::new(1, 0, -1),
            "e" => Dir::new(1, -1, 0),
            "se" => Dir::new(0, -1, 1),
            "sw" => Dir::new(-1, 0, 1),
            "w" => Dir::new(-1, 1, 0),
            "nw" => Dir::new(0, 1, -1),
            _ => return Err(anyhow!("invalid Dir {}", s)),
        })
    }
}

impl ops::AddAssign<&Dir> for Coord {
    fn add_assign(&mut self, dir: &Dir) {
        self.x += dir.x as i32;
        self.y += dir.y as i32;
        self.z += dir.z as i32;
    }
}

// Parser: //

pub mod parser {
    use super::Dir;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::char, combinator::all_consuming,
        combinator::map, multi::many1, IResult,
    };
    use std::io::BufRead;

    pub fn dir(input: &str) -> IResult<&str, Dir> {
        map(
            alt((
                tag("ne"),
                tag("e"),
                tag("se"),
                tag("sw"),
                tag("w"),
                tag("nw"),
            )),
            |s: &str| s.parse::<Dir>().unwrap(),
        )(input)
    }

    pub fn path(input: &str) -> IResult<&str, Vec<Dir>> {
        let (input, dirs) = many1(dir)(input)?;
        let (input, _) = char('\n')(input)?;
        Ok((input, dirs))
    }

    pub fn allpaths(input: &str) -> IResult<&str, Vec<Vec<Dir>>> {
        many1(path)(input)
    }

    pub fn allinput(input: &str) -> IResult<&str, Vec<Vec<Dir>>> {
        all_consuming(allpaths)(input)
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Dir>>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        Ok(allinput(&input)
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut blacks = BTreeSet::new();
    let paths = parser::parse(bufin)?;
    for path in &paths {
        let mut h = Coord::default();
        for dir in path {
            h += dir;
        }
        if blacks.contains(&h) {
            blacks.remove(&h);
        } else {
            blacks.insert(h);
        }
    }
    Ok(blacks.len())
}

#[test]
fn test0() -> Result<()> {
    eprintln!();
    let input: &[u8] = b"sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew\n";
    assert_eq!(process(input)?, 10);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
