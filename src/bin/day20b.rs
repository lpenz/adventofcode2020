// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use regex::Regex;
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

    pub fn string(&self) -> String {
        self.rows
            .iter()
            .skip(1)
            .map(|r| r.iter().skip(1).cloned().take(8).collect::<String>())
            .take(8)
            .collect()
    }

    pub fn hash_count(&self) -> usize {
        self.rows.iter().fold(0, |acc, r| {
            acc + r.iter().fold(0, |acc, c| acc + charval(*c) as usize)
        })
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

pub fn merge_tiles(geom: usize, tiles: &[Tile]) -> String {
    let mut ret = String::default();
    for tilerow in 0..geom {
        for irow in 1..9 {
            for tilecol in 0..geom {
                let t = tiles[tilerow * geom + tilecol];
                let s = t.rows[irow]
                    .iter()
                    .cloned()
                    .skip(1)
                    .take(8)
                    .collect::<String>();
                ret.push_str(&s);
            }
            ret.push('\n');
        }
    }
    ret
}

pub fn rotate_right(geom: usize, tiles: &str) -> String {
    let b = tiles.as_bytes();
    let mut ret = String::default();
    for y in 0..geom {
        for x in 0..geom {
            let offset = (geom - x - 1) * (geom + 1) + y;
            ret.push(b[offset] as char);
            if x == geom - 1 {
                ret.push('\n');
            }
        }
    }
    ret
}

pub fn flip_v(geom: usize, tiles: &str) -> String {
    let b = tiles.as_bytes();
    let mut ret = String::default();
    for y in 0..geom {
        for x in 0..geom {
            let offset = (geom - y - 1) * (geom + 1) + x;
            ret.push(b[offset] as char);
            if x == geom - 1 {
                ret.push('\n');
            }
        }
    }
    ret
}

pub fn flip_h(geom: usize, tiles: &str) -> String {
    let b = tiles.as_bytes();
    let mut ret = String::default();
    for y in 0..geom {
        for x in 0..geom {
            let offset = y * (geom + 1) + (geom - x - 1);
            ret.push(b[offset] as char);
            if x == geom - 1 {
                ret.push('\n');
            }
        }
    }
    ret
}

#[test]
fn test_string() -> Result<()> {
    let str0 = "123\n456\n789\n";
    // 123
    // 456
    // 789
    assert_eq!(flip_v(3, str0), "789\n456\n123\n");
    assert_eq!(flip_h(3, str0), "321\n654\n987\n");
    assert_eq!(rotate_right(3, str0), "741\n852\n963\n");
    Ok(())
}

pub fn monster_see(geom: usize, tiles_string: &str, row: usize, col: usize) -> bool {
    //00000000001111111111
    //01234567890123456789
    //..................#.
    //#....##....##....###
    //.#..#..#..#..#..#...
    let m1_re = Regex::new("^..................#.").unwrap();
    let m2_re = Regex::new("^#....##....##....###").unwrap();
    let m3_re = Regex::new("^.#..#..#..#..#..#...").unwrap();
    m1_re.is_match(&tiles_string[(row * (geom + 1) + col)..])
        && m2_re.is_match(&tiles_string[((row + 1) * (geom + 1) + col)..])
        && m3_re.is_match(&tiles_string[((row + 2) * (geom + 1) + col)..])
}

pub fn monster_count(geom: usize, tiles_string: &str) -> usize {
    let mut complete = tiles_string.to_string();
    for _ in 0..4 {
        for _ in 0..2 {
            for _ in 0..2 {
                let mut c = 0;
                for row in 0..geom - 2 {
                    for col in 0..geom - 19 {
                        if monster_see(geom, &complete, row, col) {
                            c += 1;
                        }
                    }
                }
                if c > 0 {
                    return c;
                }
                complete = flip_h(geom, &complete);
            }
            complete = flip_v(geom, &complete);
        }
        complete = rotate_right(geom, &complete);
    }
    0
}

#[test]
fn test_monstercount() -> Result<()> {
    let str0=".####...#####..#...###..\n#####..#..#.#.####..#.#.\n.#.#...#.###...#.##.##..\n#.#.##.###.#.##.##.#####\n..##.###.####..#.####.##\n...#.#..##.##...#..#..##\n#.##.#..#.#..#..##.#.#..\n.###.##.....#...###.#...\n#.####.#.#....##.#..#.#.\n##...#..#....#..#...####\n..#.##...###..#.#####..#\n....#.##.#.#####....#...\n..##.##.###.....#.##..#.\n#...#...###..####....##.\n.#.##...#.##.#.#.###...#\n#.###.#..####...##..#...\n#.###...#.##...#.######.\n.###.###.#######..#####.\n..##.#..#..#.#######.###\n#.#..##.########..#..##.\n#.#####..#.#...##..#....\n#....##..#.#########..##\n#...#.....#..##...###.##\n#..###....##.#...##.##.#\n";
    assert_eq!(monster_count(24, str0), 2);
    let str0 = flip_h(24, str0);
    assert_eq!(monster_count(24, &str0), 2);
    let str0 = flip_v(24, &str0);
    assert_eq!(monster_count(24, &str0), 2);
    Ok(())
}

// Process, etc //

fn process(mut bufin: impl BufRead) -> Result<usize> {
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
    // Part B:
    let tiles_string = merge_tiles(geom, &placed);
    assert_eq!(tiles_string.len(), geom * geom * 8 * 8 + 8 * geom);
    let c = monster_count(geom * 8, &tiles_string);
    eprintln!("monsters found: {}", c);
    let hashcount: usize = tiles_string
        .chars()
        .fold(0, |acc, c| acc + charval(c) as usize);
    Ok(hashcount - 15 * c)
}

#[test]
fn test1() -> Result<()> {
    let input: &[u8] = b"Tile 2311:\n..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#.#..##\n..#....#..\n###...#.#.\n..###..###\n\nTile 1951:\n#.##...##.\n#.####...#\n.....#..##\n#...######\n.##.#....#\n.###.#####\n###.##.##.\n.###....#.\n..#.#..#.#\n#...##.#..\n\nTile 1171:\n####...##.\n#..##.#..#\n##.#..#.#.\n.###.####.\n..###.####\n.##....##.\n.#...####.\n#.##.####.\n####..#...\n.....##...\n\nTile 1427:\n###.##.#..\n.#..#.##..\n.#.##.#..#\n#.#.#.##.#\n....#...##\n...##..##.\n...#.#####\n.#.####.#.\n..#..###.#\n..##.#..#.\n\nTile 1489:\n##.#.#....\n..##...#..\n.##..##...\n..#...#...\n#####...#.\n#..#.#.#.#\n...#.#.#..\n##.#...##.\n..##.##.##\n###.##.#..\n\nTile 2473:\n#....####.\n#..#.##...\n#.##..#...\n######.#.#\n.#...#.#.#\n.#########\n.###.#..#.\n########.#\n##...##.#.\n..###.#.#.\n\nTile 2971:\n..#.#....#\n#...###...\n#.#.###...\n##.##..#..\n.#####..##\n.#..####.#\n#..#.#..#.\n..####.###\n..#.#.###.\n...#.#.#.#\n\nTile 2729:\n...#.#.#.#\n####.#....\n..#.#.....\n....#..#.#\n.##..##.#.\n.#.####...\n####.#.#..\n##.####...\n##..#.##..\n#.##...##.\n\nTile 3079:\n#.#.#####.\n.#..######\n..#.......\n######....\n####.#..#.\n.#...#.##.\n#.#####.##\n..#.###...\n..#.......\n..#.###...\n";
    eprintln!("");
    assert_eq!(process(input)?, 273);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
