// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{Context, Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead};
use std::str::FromStr;

fn re_get<'a>(m: &'a regex::Captures, name: &str) -> Result<&'a str> {
    Ok(m.name(name)
        .with_context(|| format!("{} not found", name))?
        .as_str())
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Bag {
    pub name: String,
}

impl Bag {
    fn new(name: &str) -> Bag {
        Bag {
            name: name.to_string(),
        }
    }
}

impl FromStr for Bag {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inputs = s.split(' ').collect::<Vec<_>>();
        Ok(Bag {
            name: format!("{} {}", inputs[0], inputs[1]),
        })
    }
}

#[derive(Default, Debug, Clone)]
struct Rule {
    bag: Bag,
    contents: Vec<(usize, Bag)>,
}

impl Rule {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &(usize, Bag)> + 'a {
        self.contents.iter()
    }
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RULE_RE: Regex =
                Regex::new(r"^(?P<bag>[^ ]+ [^ ]+) bags contain (?P<contents>.*)\.$").unwrap();
        }
        let rule_m = RULE_RE.captures(s).context("error matching top rule RE")?;
        lazy_static! {
            static ref VEC_RE: Regex =
                Regex::new(r"^\s*(?P<num>[0-9]+) (?P<bag>[^ ]+ [^ ]+) bags?$").unwrap();
        }
        let mut bagvec = vec![];
        for entry in re_get(&rule_m, "contents")?.split(',') {
            if entry == "no other bags" {
                break;
            }
            let entry_m = VEC_RE
                .captures(entry)
                .with_context(|| format!("error matching VEC_RE in \"{}\"", entry))?;
            bagvec.push((
                re_get(&entry_m, "num")?.parse()?,
                re_get(&entry_m, "bag")?.parse()?,
            ));
        }
        Ok(Rule {
            bag: Bag::new(re_get(&rule_m, "bag")?),
            contents: bagvec,
        })
    }
}

#[derive(Default, Debug)]
struct Rules {
    pub edges: BTreeMap<Bag, Vec<(usize, Bag)>>,
}

impl Rules {
    pub fn insert(&mut self, rule: Rule) {
        let entry = self.edges.entry(rule.bag.clone());
        let innervec = entry.or_insert(vec![]);
        for (innernum, innerbag) in rule.iter() {
            innervec.push((*innernum, innerbag.clone()));
        }
    }

    pub fn num_inner_bags(&self, bag: &Bag) -> Result<BTreeMap<Bag, usize>> {
        let mut solved: BTreeMap<Bag, usize> = BTreeMap::new();
        self.dfs(&bag, &mut solved)?;
        Ok(solved)
    }

    pub fn dfs(&self, current: &Bag, solved: &mut BTreeMap<Bag, usize>) -> Result<usize> {
        if let Some(innernum) = solved.get(current) {
            Ok(*innernum)
        } else if let Some(innervec) = self.edges.get(current) {
            let mut total = 0;
            for (innernum, innerbag) in innervec {
                total += innernum * (1 + self.dfs(innerbag, solved)?);
            }
            solved.insert(current.clone(), total);
            Ok(total)
        } else {
            solved.insert(current.clone(), 0);
            Ok(0)
        }
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut rules = Rules::default();
    for line_opt in bufin.lines() {
        let line = line_opt?;
        rules.insert(line.parse::<Rule>()?);
    }
    let bag = Bag::new("shiny gold");
    let visited = rules.num_inner_bags(&bag)?;
    visited
        .get(&bag)
        .cloned()
        .with_context(|| format!("{:?} not found", bag))
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.\n";
    assert_eq!(process(input)?, 32);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let rule = "light red bags contain 1 bright white bag, 2 muted yellow bags.".parse::<Rule>()?;
    assert_eq!(rule.bag, Bag::new("light red"));
    let mut it = rule.iter();
    assert_eq!(it.next().unwrap(), &(1, Bag::new("bright white")));
    assert_eq!(it.next().unwrap(), &(2, Bag::new("muted yellow")));
    Ok(())
}

#[test]
fn test3() -> Result<()> {
    let input: &[u8] = b"shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.\n";
    assert_eq!(process(input)?, 126);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
