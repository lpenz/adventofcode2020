// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead};

pub fn line_next(lines_iter: &mut std::io::Lines<impl BufRead>) -> Result<String> {
    lines_iter
        .next()
        .ok_or_else(|| anyhow!("error reading line"))?
        .map_err(|e| anyhow!(e))
}

fn re_get<'a>(m: &'a regex::Captures, name: &str) -> Result<&'a str> {
    Ok(m.name(name)
        .with_context(|| format!("{} not found", name))?
        .as_str())
}

pub fn expand(
    rules: &BTreeMap<usize, String>,
    cache: &mut BTreeMap<usize, String>,
    ruleid: usize,
) -> Result<String> {
    if let Some(s) = cache.get(&ruleid) {
        return Ok(s.clone());
    }
    let rstr = rules.get(&ruleid).expect("invalid ruleid");
    lazy_static! {
        static ref RE_LETTER: Regex = Regex::new("^\"(?P<letter>.)\"$").unwrap();
    }
    let m_opt = RE_LETTER.captures(rstr);
    if let Some(m_letter) = m_opt {
        let letter = re_get(&m_letter, "letter")?.to_string();
        cache.insert(ruleid, letter.clone());
        return Ok(letter);
    }
    let toks = rstr.split(' ');
    let mut res = String::default();
    for t in toks {
        if t == "|" {
            res = format!("({})|", res);
        } else {
            let num = t.parse::<usize>()?;
            res = format!("{}({})", res, expand(rules, cache, num)?);
        }
    }
    cache.insert(ruleid, res.clone());
    Ok(res)
}

// Process, etc //

fn process(bufin: impl BufRead) -> Result<i64> {
    let mut rules = BTreeMap::new();
    let re_rule: Regex = Regex::new(r"^(?P<id>[^:]+): (?P<contents>.*)$")?;
    let mut lines_iter = bufin.lines();
    loop {
        let line = line_next(&mut lines_iter)?;
        if line.is_empty() {
            break;
        }
        let m_rule = re_rule
            .captures(&line)
            .with_context(|| format!("error matching top rule RE in {}", line))?;
        rules.insert(
            re_get(&m_rule, "id")?.parse::<usize>()?,
            re_get(&m_rule, "contents")?.to_string(),
        );
    }
    let mut cache = BTreeMap::default();
    let rstr = expand(&rules, &mut cache, 0)?;
    eprintln!("rule 0 regex: {}", rstr);
    let re_rule0 = Regex::new(&format!("^{}$", rstr))?;
    let mut match0 = 0;
    for line_opt in lines_iter {
        let line = line_opt?;
        if re_rule0.is_match(&line) {
            match0 += 1;
        }
    }
    Ok(match0)
}

#[test]
fn test1() -> Result<()> {
    let input: &[u8] = b"0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb\n";
    eprintln!("");
    assert_eq!(process(input)?, 2);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
