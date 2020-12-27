// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Result};
use std::io::{stdin, BufRead};

// Parser: //

pub mod parser {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::char, character::complete::digit1,
        combinator::all_consuming, combinator::map, multi::separated_list1, IResult,
    };
    use std::collections::VecDeque;
    use std::iter::FromIterator;
    use std::str::FromStr;

    pub fn card(input: &str) -> IResult<&str, i32> {
        let (input, card) = map(digit1, i32::from_str)(input)?;
        Ok((input, card.unwrap()))
    }

    pub fn deck(input: &str) -> IResult<&str, VecDeque<i32>> {
        let (input, _) = tag("Player ")(input)?;
        let (input, _) = alt((tag("1"), tag("2")))(input)?;
        let (input, _) = tag(":\n")(input)?;
        let (input, cards) = separated_list1(char('\n'), card)(input)?;
        let (input, _) = char('\n')(input)?;
        Ok((input, VecDeque::from_iter(cards.into_iter())))
    }

    pub fn decks2(input: &str) -> IResult<&str, [VecDeque<i32>; 2]> {
        let (input, deck1) = deck(input)?;
        let (input, _) = char('\n')(input)?;
        let (input, deck2) = deck(input)?;
        Ok((input, [deck1, deck2]))
    }

    pub fn allinput(input: &str) -> IResult<&str, [VecDeque<i32>; 2]> {
        all_consuming(decks2)(input)
    }
}

// Process, etc //

fn process(mut bufin: impl BufRead) -> Result<i32> {
    let mut input = String::default();
    bufin.read_to_string(&mut input)?;
    let mut decks = parser::allinput(&input)
        .map_err(|e| anyhow!("error reading input: {:?}", e))?
        .1;
    while !decks[0].is_empty() && !decks[1].is_empty() {
        let card1 = decks[0].pop_front().unwrap();
        let card2 = decks[1].pop_front().unwrap();
        if card1 > card2 {
            decks[0].push_back(card1);
            decks[0].push_back(card2);
        } else {
            decks[1].push_back(card2);
            decks[1].push_back(card1);
        }
    }
    let winner = if decks[0].is_empty() { 1 } else { 0 };
    decks[winner].make_contiguous().reverse();
    let result = decks[winner]
        .iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + v * (i as i32 + 1));
    Ok(result)
}

#[test]
fn test1() -> Result<()> {
    let input: &[u8] = b"Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10\n";
    eprintln!("");
    assert_eq!(process(input)?, 306);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
