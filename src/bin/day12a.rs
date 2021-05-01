// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Error, Result};
use std::io::{stdin, BufRead};
use std::ops;
use std::str::FromStr;

// Direction //

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    N,
    S,
    E,
    W,
}

impl FromStr for Dir {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letter = &s[0..1];
        Ok(match letter {
            "N" => Dir::N,
            "S" => Dir::S,
            "E" => Dir::E,
            "W" => Dir::W,
            _ => return Err(anyhow!("invalid Dir {}", s)),
        })
    }
}

impl ops::Add<&Act> for Dir {
    type Output = Dir;
    fn add(self, act: &Act) -> Self::Output {
        match act {
            Act::D(_) => self,
            Act::F(_) => self,
            Act::L => match self {
                Dir::N => Dir::W,
                Dir::W => Dir::S,
                Dir::S => Dir::E,
                Dir::E => Dir::N,
            },
            Act::R => match self {
                Dir::N => Dir::E,
                Dir::E => Dir::S,
                Dir::S => Dir::W,
                Dir::W => Dir::N,
            },
            Act::B => match self {
                Dir::N => Dir::S,
                Dir::W => Dir::E,
                Dir::S => Dir::N,
                Dir::E => Dir::W,
            },
        }
    }
}

// Xy //

#[derive(Debug, Copy, Clone)]
pub struct Xy {
    pub x: i32,
    pub y: i32,
}

impl Xy {
    pub fn new(x: i32, y: i32) -> Xy {
        Xy { x, y }
    }
}

// DirDist //

#[derive(Debug, Clone, Copy)]
pub struct DirDist {
    pub dir: Dir,
    pub dist: i32,
}

impl DirDist {
    pub fn new(dir: Dir, dist: i32) -> DirDist {
        DirDist { dir, dist }
    }
}

impl ops::Add<&DirDist> for Xy {
    type Output = Xy;
    fn add(self, other: &DirDist) -> Self::Output {
        Xy::new(
            self.x
                + match other.dir {
                    Dir::E => other.dist,
                    Dir::W => -other.dist,
                    _ => 0,
                },
            self.y
                + match other.dir {
                    Dir::N => other.dist,
                    Dir::S => -other.dist,
                    _ => 0,
                },
        )
    }
}

// Ship //

#[derive(Debug)]
struct Ship {
    xy: Xy,
    dir: Dir,
}

impl Default for Ship {
    fn default() -> Ship {
        Ship {
            xy: Xy::new(0, 0),
            dir: Dir::E,
        }
    }
}

impl Ship {
    pub fn manhattan(&self) -> i32 {
        self.xy.x.abs() + self.xy.y.abs()
    }
}

impl ops::AddAssign<&Act> for Ship {
    fn add_assign(&mut self, act: &Act) {
        let selfdir = self.dir;
        match act {
            Act::D(dirdist) => self.xy = self.xy + dirdist,
            Act::F(dist) => self.xy = self.xy + &DirDist::new(selfdir, *dist),
            _ => {}
        };
        self.dir = self.dir + act;
    }
}

// Action //

#[derive(Debug, Clone, Copy)]
enum Act {
    D(DirDist),
    F(i32),
    L,
    R,
    B,
}

impl FromStr for Act {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letter = &s[0..1];
        let num = s[1..].parse::<i32>()?;
        Ok(match letter {
            "N" => Act::D(DirDist::new(s.parse::<Dir>()?, num)),
            "S" => Act::D(DirDist::new(s.parse::<Dir>()?, num)),
            "E" => Act::D(DirDist::new(s.parse::<Dir>()?, num)),
            "W" => Act::D(DirDist::new(s.parse::<Dir>()?, num)),
            "F" => Act::F(num),
            "L" => match num {
                90 => Act::L,
                180 => Act::B,
                270 => Act::R,
                _ => return Err(anyhow!("invalid angle {}", num)),
            },
            "R" => match num {
                90 => Act::R,
                180 => Act::B,
                270 => Act::L,
                _ => return Err(anyhow!("invalid angle {}", num)),
            },
            _ => return Err(anyhow!("invalid first letter {}", s)),
        })
    }
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut ship = Ship::default();
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let act = line.parse::<Act>()?;
        ship += &act;
    }
    Ok(ship.manhattan())
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"F10\nN3\nF7\nR90\nF11\n";
    assert_eq!(process(input)?, 25);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
