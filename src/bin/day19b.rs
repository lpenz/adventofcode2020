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
            if res.len() == 1 {
                res = format!("{}|", res);
            } else {
                res = format!("({})|", res);
            }
        } else {
            let num = t.parse::<usize>()?;
            let r = expand(rules, cache, num)?;
            if r.len() == 1 {
                res = format!("{}{}", res, r);
            } else {
                res = format!("{}({})", res, r);
            }
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
        if line == "" {
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
    {
        // old 8: 42
        // new 8: 42 | 42 8
        expand(&rules, &mut cache, 42)?;
        let rule42 = cache.get(&42).unwrap().clone();
        cache.insert(8, format!("({})+", rule42));
    }
    {
        // old 11: 42 31
        // new 11: 42 31 | 42 11 31
        expand(&rules, &mut cache, 42)?;
        expand(&rules, &mut cache, 31)?;
        let rule42 = cache.get(&42).unwrap();
        let rule31 = cache.get(&31).unwrap();
        let mut rule11 = format!("(({})({}))", rule42, rule31);
        for i in 1..10 {
            rule11 = format!("{}|(({}){{{}}}({}){{{}}})", rule11, rule42, i, rule31, i);
        }
        cache.insert(11, rule11);
    }
    let rstr = expand(&rules, &mut cache, 0)?;
    eprintln!("rule 0 regex: {}", rstr);
    let re_rule0 = Regex::new(&format!("^{}$", rstr))?;
    let mut match0 = 0;
    for line_opt in lines_iter {
        let line = line_opt?;
        if re_rule0.is_match(&line) {
            eprintln!("match {}", line);
            match0 += 1;
        }
    }
    Ok(match0)
}

#[test]
fn test1() -> Result<()> {
    let input: &[u8] = b"42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba\n";
    eprintln!("");
    assert_eq!(process(input)?, 12);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
