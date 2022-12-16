use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;

use anyhow::{bail, Result};

pub mod day1;

pub struct AocOutput {
    pub first: Box<dyn Display>,
    pub second: Box<dyn Display>,
}

impl AocOutput {
    pub fn from(first: impl Display + 'static, second: impl Display + 'static) -> Self {
        Self {
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    pub fn todo() -> Self {
        Self {
            first: Box::new("TODO"),
            second: Box::new("TODO"),
        }
    }
}

pub fn solve(day: u32, input: BufReader<File>) -> Result<AocOutput> {
    match day {
        1 => Ok(day1::solve(input)),
        _ => bail!("Invalid or unimplemented day"),
    }
}

