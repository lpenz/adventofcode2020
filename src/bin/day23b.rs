// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::fmt::Write;
use std::io::{stdin, BufRead};

// Parser: //

pub mod parser {
    use anyhow::{anyhow, Result};
    use nom::{
        character::complete::char, character::complete::one_of, combinator::all_consuming,
        combinator::map, multi::many1, IResult,
    };
    use std::io::BufRead;

    pub fn cup(input: &str) -> IResult<&str, usize> {
        let (input, cup) = map(one_of("0123456789"), |c| c.to_digit(10).unwrap())(input)?;
        Ok((input, cup as usize))
    }

    pub fn cups(input: &str) -> IResult<&str, Vec<usize>> {
        let (input, cups) = many1(cup)(input)?;
        let (input, _) = char('\n')(input)?;
        let mut nextcup = Vec::with_capacity(1_000_000);
        let len = 1_000_000 + 1;
        for i in 0..len {
            nextcup.push(i + 1);
        }
        nextcup[len - 1] = cups[0];
        for i in 0..cups.len() - 1 {
            nextcup[cups[i]] = cups[(i + 1)];
        }
        nextcup[cups[cups.len() - 1]] = cups.iter().max().unwrap() + 1;
        nextcup[0] = cups[0];
        Ok((input, nextcup))
    }

    pub fn allinput(input: &str) -> IResult<&str, Vec<usize>> {
        all_consuming(cups)(input)
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<usize>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        Ok(allinput(&input)
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

pub fn domove(nextcup: &mut Vec<usize>) {
    let mut removed = vec![];
    let vcurr = nextcup[0];
    let mut vremove = nextcup[vcurr];
    for _ in 0..3 {
        removed.push(vremove);
        vremove = nextcup[vremove];
    }
    nextcup[vcurr] = vremove;
    let mut vdest = vcurr - 1;
    while vdest == 0 || removed.contains(&vdest) {
        if vdest <= 1 {
            vdest = nextcup.len() - 1;
        } else {
            vdest -= 1;
        }
    }
    let oldnext = nextcup[vdest];
    nextcup[vdest] = removed[0];
    nextcup[removed[2]] = oldnext;
    nextcup[0] = nextcup[nextcup[0]];
}

pub fn domoves(cups: &mut Vec<usize>, num: usize) {
    for _ in 0..num {
        domove(cups);
    }
}

pub fn getresp(nextcup: &[usize]) -> u64 {
    eprintln!("{} * {}", nextcup[1], nextcup[nextcup[1]]);
    nextcup[1] as u64 * nextcup[nextcup[1]] as u64
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<u64> {
    let mut nextcup = parser::parse(bufin)?;
    domoves(&mut nextcup, 10_000_000);
    Ok(getresp(&nextcup))
}

pub fn toresp(nextcups: &[usize]) -> String {
    let mut vcurr = nextcups[1];
    let mut ret = String::new();
    while vcurr != 1 {
        write!(ret, "{}", vcurr).unwrap();
        vcurr = nextcups[vcurr];
    }
    ret
}

#[test]
fn test_b() -> Result<()> {
    eprintln!();
    let input: &[u8] = b"389125467\n";
    let mut cups = parser::parse(input)?;
    eprintln!("{:?}", &cups[0..15]);
    domoves(&mut cups, 10_000_000);
    let resp = getresp(&cups);
    eprintln!("{:?}", &cups[0..15]);
    assert_eq!(resp, 149245887792);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
