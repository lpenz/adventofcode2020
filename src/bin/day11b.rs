// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead};

type Xy = (i32, i32);

const ITER_VEC: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Default)]
struct Ferry {
    pub seats: BTreeMap<Xy, bool>,
    pub xymax: Xy,
}

impl Ferry {
    pub fn line_parse(&mut self, y: i32, line: &str) {
        for (x, c) in line.chars().enumerate() {
            self.seats.insert((x as i32, y as i32), c == 'L');
            self.xymax.0 = x as i32;
        }
        self.xymax.1 = y;
    }

    pub fn iter_xy(&self) -> impl Iterator<Item = Xy> + '_ {
        self.seats.iter().map(|(xy, _)| *xy)
    }

    pub fn iter_visible_xy<'a>(&'a self, xy: &'a Xy) -> impl Iterator<Item = Xy> + 'a {
        ITER_VEC.iter().filter_map(move |ixy| {
            let mut nxy = (xy.0 + ixy.0, xy.1 + ixy.1);
            while self.seats.get(&nxy) == Some(&false) {
                nxy = (nxy.0 + ixy.0, nxy.1 + ixy.1);
            }
            if self.seats.get(&nxy) == Some(&true) {
                Some(nxy)
            } else {
                None
            }
        })
    }

    pub fn is_seat(&self, xy: &Xy) -> bool {
        self.seats.get(xy) == Some(&true)
    }

    pub fn iter(&self) -> i32 {
        let mut changed = true;
        let mut last_occupied: BTreeMap<Xy, bool> = Default::default();
        let mut total = 0;
        while changed {
            changed = false;
            total = 0;
            let mut occupied: BTreeMap<Xy, bool> = Default::default();
            for xy in self.iter_xy() {
                let num = self
                    .iter_visible_xy(&xy)
                    .filter(|nxy| last_occupied.get(nxy) == Some(&true))
                    .count();
                if last_occupied.get(&xy) == Some(&true) {
                    occupied.insert(xy, num <= 4);
                    if self.is_seat(&xy) && num <= 4 {
                        total += 1;
                    }
                } else {
                    occupied.insert(xy, num < 1);
                    if self.is_seat(&xy) && num < 1 {
                        total += 1;
                    }
                }
            }
            if occupied != last_occupied {
                changed = true;
            }
            last_occupied = occupied;
        }
        total
    }
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut ferry = Ferry::default();
    for (y, line_opt) in bufin.lines().enumerate() {
        let line = line_opt?;
        ferry.line_parse(y as i32, &line);
    }
    Ok(ferry.iter())
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n";
    assert_eq!(process(input)?, 26);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
