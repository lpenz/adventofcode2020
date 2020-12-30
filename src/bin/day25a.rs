// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::io::{stdin, BufRead};

// Parser: //

pub mod parser {
    use anyhow::{anyhow, Result};
    use nom::{
        character::complete::char, character::complete::digit1, combinator::all_consuming,
        combinator::map, multi::many1, IResult,
    };
    use std::io::BufRead;

    pub fn pk(input: &str) -> IResult<&str, i64> {
        let (input, pk) = map(digit1, |s: &str| s.parse::<i64>().unwrap())(input)?;
        let (input, _) = char('\n')(input)?;
        Ok((input, pk))
    }

    pub fn pks(input: &str) -> IResult<&str, [i64; 2]> {
        let (input, pks) = many1(pk)(input)?;
        Ok((input, [pks[0], pks[1]]))
    }

    pub fn allinput(input: &str) -> IResult<&str, [i64; 2]> {
        all_consuming(pks)(input)
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<[i64; 2]> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        Ok(allinput(&input)
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

pub fn transform(subj: i64, loopsize: i64) -> i64 {
    let mut curr = subj;
    for _ in 1..loopsize {
        curr = (curr * subj) % 20201227_i64;
    }
    curr
}

pub fn calc_loopsize(pk: i64) -> i64 {
    let subj = 7_i64;
    let divisor = 20201227_i64;
    let mut i = 1;
    let mut curr = subj;
    loop {
        curr = (curr * subj) % divisor;
        i += 1;
        if curr == pk {
            break i;
        }
    }
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i64> {
    let [cardpk, doorpk] = parser::parse(bufin)?;
    eprintln!("cardpk {}, doorpk {}", cardpk, doorpk);
    let cardls = calc_loopsize(cardpk);
    let doorls = calc_loopsize(doorpk);
    eprintln!("cardls {}, doorls {}", cardls, doorls);
    let ek1 = transform(doorpk, cardls);
    let ek2 = transform(cardpk, doorls);
    eprintln!("ek1 {}, ek2 {}", ek1, ek2);
    assert_eq!(ek1, ek2);
    Ok(ek1)
}

#[test]
fn test0() -> Result<()> {
    eprintln!("");
    let input: &[u8] = b"5764801\n17807724\n";
    assert_eq!(process(input)?, 14897079);
    Ok(())
}

#[test]
fn test_calc_loopsize() -> Result<()> {
    eprintln!("");
    assert_eq!(calc_loopsize(5764801), 8);
    assert_eq!(calc_loopsize(17807724), 11);
    Ok(())
}

#[test]
fn test_transform() -> Result<()> {
    eprintln!("");
    assert_eq!(transform(7, 8), 5764801);
    assert_eq!(transform(7, 11), 17807724);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
