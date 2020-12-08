// Copyright (C) 2020 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::{anyhow, Error, Result};
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};
use std::str::FromStr;

// Op //

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

impl Default for Op {
    fn default() -> Op {
        Op::Nop
    }
}

impl FromStr for Op {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s[0..3] {
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            "nop" => Op::Nop,
            _ => return Err(anyhow!("could not parse Op in \"{}\"", s)),
        })
    }
}

// Instr //

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct Instr {
    op: Op,
    arg: i32,
}

impl Instr {
    fn new(op: Op, arg: i32) -> Instr {
        Instr { op, arg }
    }
}

impl FromStr for Instr {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instr::new(s.parse()?, s[4..].parse::<i32>()?))
    }
}

// Cpu //

#[derive(Default)]
struct Cpu {
    acc: i32,
    pc: usize,
    program: Vec<Instr>,
}

impl Cpu {
    pub fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }

    pub fn run1(&mut self) {
        let instr = self.program[self.pc];
        match instr.op {
            Op::Nop => {
                self.pc += 1;
            }
            Op::Acc => {
                self.acc += instr.arg;
                self.pc += 1;
            }
            Op::Jmp => {
                self.pc = (self.pc as i32 + instr.arg) as usize;
            }
        }
    }

    pub fn terminates(&mut self) -> bool {
        let mut executed = BTreeSet::new();
        self.reset();
        loop {
            if self.pc == self.program.len() {
                return true;
            }
            if executed.contains(&self.pc) {
                return false;
            }
            executed.insert(self.pc);
            self.run1();
        }
    }

    pub fn fix(&mut self) {
        let num_instr = self.program.len();
        for i in 0..num_instr {
            if self.program[i].op == Op::Acc {
                continue;
            }
            let old = self.program[i];
            self.program[i] = if old.op == Op::Nop {
                Instr::new(Op::Jmp, old.arg)
            } else {
                Instr::new(Op::Nop, old.arg)
            };
            if self.terminates() {
                return;
            }
            self.program[i] = old;
        }
    }
}

fn process(bufin: impl BufRead) -> Result<i32> {
    let mut cpu = Cpu::default();
    for line_opt in bufin.lines() {
        let line = line_opt?;
        cpu.program.push(line.parse()?);
    }
    cpu.fix();
    Ok(cpu.acc)
}

#[test]
fn test() -> Result<()> {
    let input: &[u8] = b"nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6\n";
    assert_eq!(process(input)?, 8);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    for (s, instr) in vec![
        ("nop +0", Instr::new(Op::Nop, 0)),
        ("acc +1", Instr::new(Op::Acc, 1)),
        ("acc -5", Instr::new(Op::Acc, -5)),
        ("jmp +700", Instr::new(Op::Jmp, 700)),
        ("jmp -900", Instr::new(Op::Jmp, -900)),
    ] {
        assert_eq!(s.parse::<Instr>()?, instr);
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", process(stdin().lock())?);
    Ok(())
}
