// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;
use std::collections::VecDeque;
use std::io::{stdin, BufRead};

// Parser: //

pub mod parser {
    use anyhow::{anyhow, Result};
    use nom::{
        character::complete::char, character::complete::one_of, combinator::all_consuming,
        combinator::map, multi::many1, IResult,
    };
    use std::collections::VecDeque;
    use std::io::BufRead;

    pub fn cup(input: &str) -> IResult<&str, u8> {
        let (input, cup) = map(one_of("0123456789"), |c| c.to_digit(10).unwrap())(input)?;
        Ok((input, cup as u8))
    }

    pub fn cups(input: &str) -> IResult<&str, VecDeque<u8>> {
        let (input, cups) = many1(cup)(input)?;
        let (input, _) = char('\n')(input)?;
        Ok((input, cups.into_iter().collect()))
    }

    pub fn allinput(input: &str) -> IResult<&str, VecDeque<u8>> {
        all_consuming(cups)(input)
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<VecDeque<u8>> {
        let mut input = String::default();
        bufin.read_to_string(&mut input)?;
        Ok(allinput(&input)
            .map_err(|e| anyhow!("error reading input: {:?}", e))?
            .1)
    }
}

pub fn domove(cups: &mut VecDeque<u8>, icurr: &mut usize) {
    let mut removed = vec![];
    cups.rotate_left(*icurr);
    *icurr = 0;
    for _ in 0..3 {
        removed.push(cups.remove(1).unwrap());
    }
    let vmin = *cups.iter().min().unwrap();
    let mut vdest = cups[*icurr] - 1;
    while !cups.contains(&vdest) {
        if vdest > 0 {
            vdest -= 1;
        }
        if vdest < vmin {
            vdest = *cups.iter().max().unwrap();
        }
    }
    while cups[0] != vdest {
        cups.rotate_right(1);
        *icurr = (*icurr + 1) % cups.len();
    }
    for i in 0..3 {
        cups.insert(1, removed[2 - i]);
        *icurr += 1;
    }
    *icurr = (*icurr + 1) % cups.len();
}

pub fn domoves(cups: &mut VecDeque<u8>, icurr: &mut usize, num: usize) {
    for _ in 0..num {
        domove(cups, icurr);
    }
}

pub fn toresp(cups0: &VecDeque<u8>) -> String {
    let mut cups = cups0.clone();
    while cups[0] != 1 {
        cups.rotate_left(1);
    }
    cups.pop_front();
    cups.iter().map(|v| format!("{}", v)).collect::<String>()
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<String> {
    let mut cups = parser::parse(bufin)?;
    let mut icurr = 0;
    domoves(&mut cups, &mut icurr, 100);
    Ok(toresp(&cups))
}

#[test]
fn test0() -> Result<()> {
    eprintln!();
    let input: &[u8] = b"389125467\n";
    let mut cups = parser::parse(input)?;
    let mut icurr = 0;
    domoves(&mut cups, &mut icurr, 10);
    assert_eq!(toresp(&cups), "92658374");
    Ok(())
}

#[test]
fn test1() -> Result<()> {
    eprintln!();
    let input: &[u8] = b"389125467\n";
    let mut cups = parser::parse(input)?;
    let mut icurr = 0;
    domoves(&mut cups, &mut icurr, 100);
    assert_eq!(process(input)?, "67384529");
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    eprintln!();
    let input: &[u8] = b"389125467\n";
    let mut cups = parser::parse(input)?;
    let mut icurr = 0;
    eprintln!("-- move 1 --");
    domove(&mut cups, &mut icurr);
    eprintln!("-- move 2 --");
    assert_eq!(toresp(&cups), "54673289");
    assert_eq!(cups[icurr], 2);
    domove(&mut cups, &mut icurr);
    eprintln!("-- move 3 --");
    assert_eq!(toresp(&cups), "32546789");
    assert_eq!(cups[icurr], 5);
    domove(&mut cups, &mut icurr);
    eprintln!("-- move 4 --");
    assert_eq!(toresp(&cups), "34672589");
    assert_eq!(cups[icurr], 8);
    domove(&mut cups, &mut icurr);
    eprintln!("-- move 5 --");
    assert_eq!(toresp(&cups), "32584679");
    assert_eq!(cups[icurr], 4);
    domove(&mut cups, &mut icurr);
    eprintln!("-- move 6 --");
    assert_eq!(toresp(&cups), "36792584");
    assert_eq!(cups[icurr], 1);
    domove(&mut cups, &mut icurr);
    eprintln!("-- move 7 --");
    assert_eq!(toresp(&cups), "93672584");
    assert_eq!(cups[icurr], 9);
    domove(&mut cups, &mut icurr);
    eprintln!("-- move 8 --");
    assert_eq!(toresp(&cups), "92583674");
    assert_eq!(cups[icurr], 2);
    domove(&mut cups, &mut icurr);
    eprintln!("-- move 9 --");
    assert_eq!(toresp(&cups), "58392674");
    assert_eq!(cups[icurr], 6);
    domove(&mut cups, &mut icurr);
    eprintln!("-- move 10 --");
    assert_eq!(toresp(&cups), "83926574");
    assert_eq!(cups[icurr], 5);
    domove(&mut cups, &mut icurr);
    eprintln!("-- final --");
    assert_eq!(toresp(&cups), "92658374");
    assert_eq!(cups[icurr], 8);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
