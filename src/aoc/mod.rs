use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;

use crate::{Error, Result};
use crate::aoc::day1::Day1;

pub mod day1;

pub type AocInput = BufReader<File>;

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
    let output = match day {
        1 => Day1::solve(input),
        _ => return Err(Error::InvalidDay),
    };

    Ok(output)
}

