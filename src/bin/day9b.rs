// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::io::{stdin, BufRead};

// NotsumHunter //

#[derive(Clone)]
struct NotsumHunter {
    preamble: usize,
    numvec: VecDeque<i64>,
    // Key is lower number, value is sum with numbers higher than key
    sums: BTreeMap<i64, BTreeSet<i64>>,
}

impl NotsumHunter {
    fn new(preamble: usize) -> NotsumHunter {
        NotsumHunter {
            preamble,
            numvec: Default::default(),
            sums: Default::default(),
        }
    }

    fn insert(&mut self, num: i64) -> Result<bool> {
        let mut valid = false;
        if self.numvec.len() >= self.preamble {
            // preamble done, check if valid
            for lower in &self.numvec {
                if *lower <= num / 2 {
                    // if lower > num / 2, it's already been checked
                    if self
                        .sums
                        .get(lower)
                        .with_context(|| format!("lower {} not found in sums!", lower))?
                        .contains(&num)
                    {
                        valid = true;
                        break;
                    }
                }
            }
            // remove first num
            let old = self
                .numvec
                .pop_front()
                .with_context(|| "preamble is zero?")?;
            self.sums.remove(&old);
        } else {
            // preamble not done, it's valid, and there's nothing to remove
            valid = true;
        }
        // Put num in structures
        let mut sums: BTreeSet<i64> = Default::default();
        for other in &self.numvec {
            if num < *other {
                sums.insert(num + other);
            } else if let Some(ref mut osums) = self.sums.get_mut(other) {
                osums.insert(num + other);
            }
        }
        self.numvec.push_back(num);
        self.sums.insert(num, sums);
        Ok(valid)
    }
}

// KeyHunter //

#[derive(Default, Clone)]
struct KeyHunter {
    allnums: Vec<i64>,
}

impl KeyHunter {
    pub fn insert(&mut self, num: i64) {
        self.allnums.push(num);
    }

    pub fn findsum(&self, target: i64) -> Result<(usize, usize)> {
        let mut l = 0;
        let mut h = 1;
        let mut sum = self.allnums[l] + self.allnums[h];
        while sum != target {
            while sum < target || h == l {
                h += 1;
                sum += self.allnums[h];
            }
            while sum > target {
                sum -= self.allnums[l];
                l += 1;
            }
        }
        // assert_eq!((l..=h).map(|i| self.allnums[i]).sum::<i64>(), sum);
        Ok((l, h))
    }

    pub fn hunt(&self, target: i64) -> Result<i64> {
        let (l, h) = self.findsum(target)?;
        let mut min = self.allnums[l];
        let mut max = self.allnums[l];
        for i in l..=h {
            let n = self.allnums[i];
            if n < min {
                min = n;
            }
            if n > max {
                max = n
            }
        }
        Ok(min + max)
    }
}

// Process, etc //

fn process(preamble: usize, bufin: impl BufRead) -> Result<i64> {
    let mut notnum = NotsumHunter::new(preamble);
    let mut key = KeyHunter::default();
    let mut invalid_opt = None;
    for line_opt in bufin.lines() {
        let line = line_opt?;
        let num = line.parse()?;
        if invalid_opt.is_none() && !notnum.insert(num)? {
            invalid_opt = Some(num);
        }
        key.insert(num);
    }
    let invalid = invalid_opt.context("all numbers are valid")?;
    key.hunt(invalid)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] =
        b"35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
    assert_eq!(process(5, input)?, 62);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let mut state0 = NotsumHunter::new(25);
    for i in 1..=25 {
        assert!(state0.insert(i)?);
    }
    let mut state = state0.clone();
    assert!(state.insert(26)?);
    let mut state = state0.clone();
    assert!(state.insert(49)?);
    let mut state = state0.clone();
    assert!(!state.insert(100)?);
    let mut state = state0.clone();
    assert!(!state.insert(50)?);
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(25, stdin().lock())?);
    Ok(())
}
