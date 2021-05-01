// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, bail, Result};
use std::io::{stdin, BufRead};

use nom::{character::complete::char, multi::many0, multi::many1, IResult};

// Process, etc //

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Num(i64),
    Plus,
    Mult,
    ParOp,
    ParCl,
}

impl Token {
    pub fn get_num(&self) -> Result<i64> {
        if let Token::Num(i) = self {
            return Ok(*i);
        }
        bail!("token is not Num: {:?}", self)
    }
}

fn num(input: &str) -> IResult<&str, Token> {
    let (input, _) = many0(char(' '))(input)?;
    let n = nom::combinator::recognize(many1(nom::character::complete::digit1))(input)?;
    Ok((n.0, Token::Num(n.1.parse::<i64>().unwrap())))
}

fn plus(input: &str) -> IResult<&str, Token> {
    let (input, _) = many0(char(' '))(input)?;
    let r = char('+')(input)?;
    Ok((r.0, Token::Plus))
}

fn mult(input: &str) -> IResult<&str, Token> {
    let (input, _) = many0(char(' '))(input)?;
    let r = char('*')(input)?;
    Ok((r.0, Token::Mult))
}

fn parop(input: &str) -> IResult<&str, Token> {
    let (input, _) = many0(char(' '))(input)?;
    let r = char('(')(input)?;
    Ok((r.0, Token::ParOp))
}

fn parcl(input: &str) -> IResult<&str, Token> {
    let (input, _) = many0(char(' '))(input)?;
    let r = char(')')(input)?;
    Ok((r.0, Token::ParCl))
}

fn parse_tokens(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, _) = many0(char(' '))(input)?;
    many1(nom::branch::alt((num, plus, mult, parop, parcl)))(input)
}

fn evaluate(tokens0: &[Token]) -> Result<i64> {
    let mut oldtokens = tokens0.to_vec();
    let mut tokens = vec![];
    // Remove parenthesis recursively
    while !oldtokens.is_empty() {
        let t = oldtokens.remove(0);
        if t == Token::ParOp {
            let mut lvl = 0;
            let mut newtokens = vec![];
            loop {
                let newt = oldtokens.remove(0);
                if newt == Token::ParCl && lvl == 0 {
                    break;
                }
                if newt == Token::ParOp {
                    lvl += 1;
                } else if newt == Token::ParCl {
                    lvl -= 1;
                }
                newtokens.push(newt);
            }
            tokens.push(Token::Num(evaluate(&newtokens)?));
        } else {
            tokens.push(t);
        }
    }
    // Process operations
    while tokens.len() > 1 {
        let t0 = tokens.remove(0);
        let op = tokens.remove(0);
        let t1 = tokens.remove(0);
        match (t0, op, t1) {
            (Token::Num(a), Token::Plus, Token::Num(b)) => {
                tokens.insert(0, Token::Num(a + b));
                continue;
            }
            (Token::Num(a), Token::Mult, Token::Num(b)) => {
                tokens.insert(0, Token::Num(a * b));
                continue;
            }
            _ => {}
        }
    }
    tokens[0].get_num()
}

fn process(bufin: impl BufRead) -> Result<i64> {
    let mut sum = 0;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let (_, tokens) = parse_tokens(&line).map_err(|_| anyhow!("error parsing"))?;
        sum += evaluate(&tokens)?;
    }
    Ok(sum)
}

#[test]
fn test_parser1() -> Result<()> {
    eprintln!();
    assert_eq!(num("2345asdf")?, ("asdf", Token::Num(2345)));
    Ok(())
}

#[test]
fn test_parser2() -> Result<()> {
    eprintln!();
    assert_eq!(
        parse_tokens("1+2*3+(4*5)+8")?,
        (
            "",
            vec![
                Token::Num(1),
                Token::Plus,
                Token::Num(2),
                Token::Mult,
                Token::Num(3),
                Token::Plus,
                Token::ParOp,
                Token::Num(4),
                Token::Mult,
                Token::Num(5),
                Token::ParCl,
                Token::Plus,
                Token::Num(8)
            ]
        )
    );
    Ok(())
}

#[test]
fn test_parser3() -> Result<()> {
    eprintln!();
    assert_eq!(
        parse_tokens("1 +  2 * 3+(    4 *5)+  8")?,
        (
            "",
            vec![
                Token::Num(1),
                Token::Plus,
                Token::Num(2),
                Token::Mult,
                Token::Num(3),
                Token::Plus,
                Token::ParOp,
                Token::Num(4),
                Token::Mult,
                Token::Num(5),
                Token::ParCl,
                Token::Plus,
                Token::Num(8)
            ]
        )
    );
    Ok(())
}

#[test]
fn test1() -> Result<()> {
    let input: &[u8] = b"1 + 2 * 3 + 4 * 5 + 6\n";
    eprintln!();
    assert_eq!(process(input)?, 71);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let input: &[u8] = b"1 + (2 * 3) + (4 * (5 + 6))\n";
    eprintln!();
    assert_eq!(process(input)?, 51);
    Ok(())
}

#[test]
fn test3() -> Result<()> {
    let input: &[u8] = b"2 * 3 + (4 * 5)\n";
    eprintln!();
    assert_eq!(process(input)?, 26);
    Ok(())
}

#[test]
fn test4() -> Result<()> {
    let input: &[u8] = b"5 + (8 * 3 + 9 + 3 * 4 * 3)\n";
    eprintln!();
    assert_eq!(process(input)?, 437);
    Ok(())
}

#[test]
fn test5() -> Result<()> {
    let input: &[u8] = b"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n";
    eprintln!();
    assert_eq!(process(input)?, 12240);
    Ok(())
}

#[test]
fn test6() -> Result<()> {
    let input: &[u8] = b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2\n";
    eprintln!();
    assert_eq!(process(input)?, 13632);
    Ok(())
}

#[test]
fn test7() -> Result<()> {
    let input: &[u8] = b"1 + (2 * 3)\n";
    eprintln!();
    assert_eq!(process(input)?, 7);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
