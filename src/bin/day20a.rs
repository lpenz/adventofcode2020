// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::io::{stdin, BufRead};

pub fn reverse_bits(n: u16) -> u16 {
    (0..10).fold(0, |acc, i| {
        acc | if n & (1 << (9 - i)) > 0 { 1 << i } else { 0 }
    })
}

pub fn charval(c: char) -> u16 {
    if c == '#' {
        1
    } else {
        0
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile {
    pub id: i64,
    pub rows: [[char; 10]; 10],
}

impl Tile {
    pub fn new(id: i64, rows: [[char; 10]; 10]) -> Tile {
        Tile { id, rows }
    }

    pub fn top(&self) -> u16 {
        (0..10).fold(0, |acc, i| (acc << 1) | charval(self.rows[0][i]))
    }

    pub fn bottom(&self) -> u16 {
        (0..10).fold(0, |acc, i| (acc << 1) | charval(self.rows[9][9 - i]))
    }

    pub fn right(&self) -> u16 {
        (0..10).fold(0, |acc, i| (acc << 1) | charval(self.rows[i][9]))
    }

    pub fn left(&self) -> u16 {
        (0..10).fold(0, |acc, i| (acc << 1) | charval(self.rows[9 - i][0]))
    }

    pub fn rotate_right(&mut self) {
        let old = self.rows;
        for y in 0..10 {
            for x in 0..10 {
                self.rows[y][x] = old[9 - x][y];
            }
        }
    }

    pub fn flip_v(&mut self) {
        let old = self.rows;
        for y in 0..10 {
            self.rows[y] = old[9 - y];
        }
    }

    pub fn flip_h(&mut self) {
        let old = self.rows;
        for (y, oldrow) in old.iter().enumerate() {
            for x in 0..10 {
                self.rows[y][x] = oldrow[9 - x];
            }
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for y in 0..10 {
            for x in 0..10 {
                write!(f, "{}", self.rows[y][x])?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

pub fn fits(geom: usize, grid: &[Tile], tile: &Tile) -> bool {
    let pos = grid.len();
    if pos == 0 {
        return true;
    }
    if pos < geom {
        return tile.left() == reverse_bits(grid[pos - 1].right());
    }
    if pos % geom == 0 {
        return tile.top() == reverse_bits(grid[pos - geom].bottom());
    }
    tile.top() == reverse_bits(grid[pos - geom].bottom())
        && tile.left() == reverse_bits(grid[pos - 1].right())
}

pub fn allids(tiles: &[Tile]) -> Vec<i64> {
    tiles.iter().map(|t| t.id).collect()
}

pub fn placeall(geom: usize, cache: &Cache, placed: &mut Vec<Tile>, rest0: &BTreeSet<i64>) -> bool {
    if rest0.is_empty() {
        return true;
    }
    let pos = placed.len();
    let candidates = if pos == 0 {
        Some(&cache.all)
    } else if pos < geom {
        cache.lefts.get(&reverse_bits(placed[pos - 1].right()))
    } else if pos % geom == 0 {
        cache.tops.get(&reverse_bits(placed[pos - geom].bottom()))
    } else {
        cache.lefttops.get(&(
            reverse_bits(placed[pos - 1].right()),
            reverse_bits(placed[pos - geom].bottom()),
        ))
    };
    if let Some(cands) = candidates {
        for t in cands {
            if !rest0.contains(&t.id) {
                continue;
            }
            let mut rest = rest0.clone();
            rest.remove(&t.id);
            placed.push(*t);
            if placeall(geom, cache, placed, &rest) {
                return true;
            }
            placed.pop();
        }
    }
    false
}

#[derive(Default)]
pub struct Cache {
    all: BTreeSet<Tile>,
    lefts: BTreeMap<u16, BTreeSet<Tile>>,
    tops: BTreeMap<u16, BTreeSet<Tile>>,
    lefttops: BTreeMap<(u16, u16), BTreeSet<Tile>>,
}

impl Cache {
    pub fn populate(tiles: &[Tile]) -> Cache {
        let mut cache = Cache::default();
        for t0 in tiles {
            let mut t = *t0;
            for _ in 0..4 {
                for _ in 0..2 {
                    for _ in 0..2 {
                        cache.all.insert(t);
                        let e = cache.lefts.entry(t.left()).or_default();
                        e.insert(t);
                        let e = cache.tops.entry(t.top()).or_default();
                        e.insert(t);
                        let klefttop = (t.left(), t.top());
                        let e = cache.lefttops.entry(klefttop).or_default();
                        e.insert(t);
                        t.flip_v();
                    }
                    t.flip_h();
                }
                t.rotate_right();
            }
        }
        cache
    }
}

// Parser: //
pub mod parser {
    use super::Tile;
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::char, character::complete::digit1,
        combinator::all_consuming, combinator::map, multi::count, multi::many0,
        multi::separated_list1, IResult,
    };
    use std::convert::TryInto;
    use std::str::FromStr;

    pub fn cell(input: &str) -> IResult<&str, char> {
        alt((char('.'), char('#')))(input)
    }

    pub fn row(input: &str) -> IResult<&str, [char; 10]> {
        let (input, cells) = count(cell, 10)(input)?;
        let (input, _) = char('\n')(input)?;
        let arr = cells.try_into().unwrap();
        Ok((input, arr))
    }

    pub fn tile(input: &str) -> IResult<&str, Tile> {
        let (input, _) = tag("Tile ")(input)?;
        let (input, id_res) = map(digit1, i64::from_str)(input)?;
        let (input, _) = tag(":\n")(input)?;
        let (input, rows0) = count(row, 10)(input)?;
        let arr = rows0.try_into().unwrap();
        let id = id_res.unwrap();
        Ok((input, Tile::new(id, arr)))
    }

    pub fn alltiles(input: &str) -> IResult<&str, Vec<Tile>> {
        let (input, tiles) = separated_list1(char('\n'), tile)(input)?;
        let (input, _) = many0(char('\n'))(input)?;
        Ok((input, tiles))
    }

    pub fn allinput(input: &str) -> IResult<&str, Vec<Tile>> {
        all_consuming(alltiles)(input)
    }
}

// Process, etc //

fn process(mut bufin: impl BufRead) -> Result<i64> {
    let mut input = String::default();
    bufin.read_to_string(&mut input)?;
    let tiles = parser::allinput(&input)
        .map_err(|e| anyhow!("error reading input: {:?}", e))?
        .1;
    let mut placed = vec![];
    let geom = if tiles.len() == 9 { 3 } else { 12 };
    assert_eq!(geom * geom, tiles.len());
    let cache = Cache::populate(&tiles);
    eprintln!("cache populated");
    let rest = tiles.iter().map(|t| t.id).collect();
    placeall(geom, &cache, &mut placed, &rest);
    assert_eq!(placed.len(), geom * geom);
    Ok(placed[0].id
        * placed[geom - 1].id
        * placed[geom * (geom - 1)].id
        * placed[geom * geom - 1].id)
}

#[test]
fn test_parser_tile() -> Result<()> {
    let t = parser::tile("Tile 1:\n........#.\n#.........\n..........\n..........\n..........\n..........\n..........\n..........\n.........#\n.#........\n")?;
    assert_eq!(t.1.id, 1);
    assert_eq!(t.1.top(), 2);
    assert_eq!(t.1.right(), 2);
    assert_eq!(t.1.bottom(), 2);
    assert_eq!(t.1.left(), 2);
    Ok(())
}

#[test]
fn test1() -> Result<()> {
    let input: &[u8] = b"Tile 2311:\n..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#.#..##\n..#....#..\n###...#.#.\n..###..###\n\nTile 1951:\n#.##...##.\n#.####...#\n.....#..##\n#...######\n.##.#....#\n.###.#####\n###.##.##.\n.###....#.\n..#.#..#.#\n#...##.#..\n\nTile 1171:\n####...##.\n#..##.#..#\n##.#..#.#.\n.###.####.\n..###.####\n.##....##.\n.#...####.\n#.##.####.\n####..#...\n.....##...\n\nTile 1427:\n###.##.#..\n.#..#.##..\n.#.##.#..#\n#.#.#.##.#\n....#...##\n...##..##.\n...#.#####\n.#.####.#.\n..#..###.#\n..##.#..#.\n\nTile 1489:\n##.#.#....\n..##...#..\n.##..##...\n..#...#...\n#####...#.\n#..#.#.#.#\n...#.#.#..\n##.#...##.\n..##.##.##\n###.##.#..\n\nTile 2473:\n#....####.\n#..#.##...\n#.##..#...\n######.#.#\n.#...#.#.#\n.#########\n.###.#..#.\n########.#\n##...##.#.\n..###.#.#.\n\nTile 2971:\n..#.#....#\n#...###...\n#.#.###...\n##.##..#..\n.#####..##\n.#..####.#\n#..#.#..#.\n..####.###\n..#.#.###.\n...#.#.#.#\n\nTile 2729:\n...#.#.#.#\n####.#....\n..#.#.....\n....#..#.#\n.##..##.#.\n.#.####...\n####.#.#..\n##.####...\n##..#.##..\n#.##...##.\n\nTile 3079:\n#.#.#####.\n.#..######\n..#.......\n######....\n####.#..#.\n.#...#.##.\n#.#####.##\n..#.###...\n..#.......\n..#.###...\n";
    eprintln!("");
    assert_eq!(process(input)?, 20899048083289);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let input = "Tile 2311:\n..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#.#..##\n..#....#..\n###...#.#.\n..###..###\n\nTile 1951:\n#.##...##.\n#.####...#\n.....#..##\n#...######\n.##.#....#\n.###.#####\n###.##.##.\n.###....#.\n..#.#..#.#\n#...##.#..\n\nTile 1171:\n####...##.\n#..##.#..#\n##.#..#.#.\n.###.####.\n..###.####\n.##....##.\n.#...####.\n#.##.####.\n####..#...\n.....##...\n\nTile 1427:\n###.##.#..\n.#..#.##..\n.#.##.#..#\n#.#.#.##.#\n....#...##\n...##..##.\n...#.#####\n.#.####.#.\n..#..###.#\n..##.#..#.\n\nTile 1489:\n##.#.#....\n..##...#..\n.##..##...\n..#...#...\n#####...#.\n#..#.#.#.#\n...#.#.#..\n##.#...##.\n..##.##.##\n###.##.#..\n\nTile 2473:\n#....####.\n#..#.##...\n#.##..#...\n######.#.#\n.#...#.#.#\n.#########\n.###.#..#.\n########.#\n##...##.#.\n..###.#.#.\n\nTile 2971:\n..#.#....#\n#...###...\n#.#.###...\n##.##..#..\n.#####..##\n.#..####.#\n#..#.#..#.\n..####.###\n..#.#.###.\n...#.#.#.#\n\nTile 2729:\n...#.#.#.#\n####.#....\n..#.#.....\n....#..#.#\n.##..##.#.\n.#.####...\n####.#.#..\n##.####...\n##..#.##..\n#.##...##.\n\nTile 3079:\n#.#.#####.\n.#..######\n..#.......\n######....\n####.#..#.\n.#...#.##.\n#.#####.##\n..#.###...\n..#.......\n..#.###...\n";
    eprintln!("");
    let tiles = parser::allinput(input.into())
        .map_err(|e| anyhow!("error reading input: {:?}", e))?
        .1;
    let mut tile2311 = tiles[0];
    assert_eq!(tile2311.id, 2311);
    let mut tile1951 = tiles[1];
    assert_eq!(tile1951.id, 1951);
    let mut tile1171 = tiles[2];
    assert_eq!(tile1171.id, 1171);
    let mut tile1427 = tiles[3];
    assert_eq!(tile1427.id, 1427);
    let mut tile1489 = tiles[4];
    assert_eq!(tile1489.id, 1489);
    let mut tile2473 = tiles[5];
    assert_eq!(tile2473.id, 2473);
    let mut tile2971 = tiles[6];
    assert_eq!(tile2971.id, 2971);
    let mut tile2729 = tiles[7];
    assert_eq!(tile2729.id, 2729);
    let tile3079 = tiles[8];
    assert_eq!(tile3079.id, 3079);
    tile1951.flip_v();
    assert_eq!(tile1951.top(), 0x234);
    tile2311.flip_v();
    assert_eq!(tile2311.top(), 0x0e7);
    assert_eq!(tile3079.top(), 0x2be);
    tile2729.flip_v();
    assert_eq!(tile2729.top(), 0x2c6);
    tile1427.flip_v();
    assert_eq!(tile1427.top(), 0x0d2);
    tile2473.rotate_right();
    tile2473.flip_v();
    assert_eq!(tile2473.top(), 0x0b8);
    tile2971.flip_v();
    assert_eq!(tile2971.top(), 0x055);
    tile1489.flip_v();
    assert_eq!(tile1489.top(), 0x3b4);
    tile1171.flip_h();
    assert_eq!(tile1171.top(), 0x18f);
    let tiles = vec![tile1951, tile2311, tile2729, tile1427];
    let cache = Cache::populate(&tiles);
    let rest = tiles.iter().map(|t| t.id).collect();
    let mut placed = vec![];
    placeall(2, &cache, &mut placed, &rest);
    assert_eq!(placed.len(), 4);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
