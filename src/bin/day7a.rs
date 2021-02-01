// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};
use std::str::FromStr;

fn re_get<'a>(m: &'a regex::Captures, name: &str) -> Result<&'a str> {
    Ok(m.name(name)
        .ok_or_else(|| anyhow!("{} not found", name))?
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
        let rule_m = RULE_RE
            .captures(s)
            .ok_or_else(|| anyhow!("error matching top rule RE"))?;
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
                .ok_or_else(|| anyhow!("error matching VEC_RE in \"{}\"", entry))?;
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

#[derive(Default)]
struct Rules {
    edges: BTreeMap<Bag, Vec<Bag>>,
}

impl Rules {
    pub fn insert(&mut self, rule: Rule) {
        for (_, innerbag) in rule.iter() {
            let entry = self.edges.entry(innerbag.clone());
            let outervec = entry.or_insert_with(Vec::new);
            outervec.push(rule.bag.clone());
        }
    }

    pub fn num_outer_colors(&self, bag: &Bag) -> Result<BTreeSet<Bag>> {
        let mut visited = BTreeSet::new();
        self.dfs(&bag, &mut visited)?;
        Ok(visited)
    }

    pub fn dfs(&self, current: &Bag, visited: &mut BTreeSet<Bag>) -> Result<()> {
        if let Some(outervec) = self.edges.get(current) {
            for outer in outervec {
                if visited.contains(outer) {
                    continue;
                }
                visited.insert(outer.clone());
                self.dfs(outer, visited)?;
            }
        }
        Ok(())
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut rules = Rules::default();
    for line_opt in bufin.lines() {
        let line = line_opt?;
        rules.insert(line.parse::<Rule>()?);
    }
    let outer = rules.num_outer_colors(&Bag::new("shiny gold"))?;
    // eprintln!("outer {:?}", outer);
    Ok(outer.len())
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.\n";
    assert_eq!(process(input)?, 4);
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

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
