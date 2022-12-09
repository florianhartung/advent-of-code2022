extern crate core;

use std::borrow::Borrow;
use std::env;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{File, read};
use std::io::{BufReader, Write};
use std::process::Termination;
use crate::aoc::AocDaySolver;

mod aoc;

pub enum Error {
    NoDayGiven,
    InputFileReading,
    InvalidDay,
}

type Result<T> = std::result::Result<T, Error>;

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoDayGiven => write!(f, "Expected a day as the first argument!"),
            Error::InputFileReading => write!(f, "Unable to read input file for requested day!"),
            Error::InvalidDay => write!(f, "Invalid day!"),
        }
    }
}

fn main() -> Result<()> {
    let day = read_day_from_args()?;

    let input_file = open_input_file(day)?;

    let output = aoc::solve(day, input_file)?;

    println!("[Day {day}] {}, {}", output.first, output.second);

    Ok(())
}

fn read_day_from_args() -> Result<u32> {
    env::args()
        .skip(1)
        .next()
        .ok_or(Error::NoDayGiven)?
        .parse::<u32>()
        .map_err(|_| Error::InvalidDay)
}

fn open_input_file(day: u32) -> Result<BufReader<File>> {
    let input_file = File::open(format!("./res/day{}.txt", day))
        .map_err(|_| Error::InputFileReading)?;

    Ok(BufReader::new(input_file))
}