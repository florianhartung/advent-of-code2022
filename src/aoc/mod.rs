use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use anyhow::{bail, Result};

use crate::aoc::day1::Day1;

pub mod day1;

pub struct AocOutput {
    pub first: Box<dyn Display>,
    pub second: Box<dyn Display>,
}

impl AocOutput {
    pub fn from(first: impl Display + 'static, second: impl Display + 'static) -> Self {
        AocOutput {
            first: Box::new(first),
            second: Box::new(second),
        }
    }
}

pub trait AocDaySolver {
    fn solve(input: BufReader<File>) -> AocOutput;
}

pub fn solve(day: u32, input: BufReader<File>) -> Result<AocOutput> {
    match day {
        1 => Ok(Day1::solve(input)),
        _ => bail!("Invalid or unimplemented day"),
    }
}

