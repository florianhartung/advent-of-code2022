use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;

use anyhow::{bail, Result};

pub mod day1;
pub mod day2;
pub mod day3;

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

    pub fn first(first: impl Display + 'static) -> Self {
        Self::from(first, "---")
    }

    pub fn second(second: impl Display + 'static) -> Self {
        Self::from("---", second)
    }

    #[allow(dead_code)]
    pub fn todo() -> Self {
        Self {
            first: Box::new("TODO"),
            second: Box::new("TODO"),
        }
    }
}

pub fn solve(day: u32, input: BufReader<File>) -> Result<AocOutput> {
    let solve_fn = match day {
        1 => day1::solve,
        2 => day2::solve,
        3 => day3::solve,
        _ => bail!("Invalid or unimplemented day"),
    };

    Ok(solve_fn(input))
}

