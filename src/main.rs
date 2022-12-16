extern crate core;

use std::fs::File;
use std::io::{BufReader};

use anyhow::{Context, Result};

mod aoc;

const INPUT_PATH_FOR_DAY: fn(u32) -> String = |day: u32| format!("./inputs/day{}.txt", day);

fn main() -> Result<()> {
    let day: u32 = std::env::args()
        .skip(1)
        .next()
        .with_context(|| "No day given as argument")?
        .parse()
        .with_context(|| "Invalid day given as argument")?;

    println!("Selected day {}", day);

    let input_for_day = read_input_file_for_day(day)?;

    let answer = aoc::solve(day, input_for_day)?;

    println!("[Day {}] {}, {}", day, answer.first, answer.second);

    Ok(())
}

fn read_input_file_for_day(day: u32) -> Result<BufReader<File>> {
    let file_path = INPUT_PATH_FOR_DAY(day);

    let input_file = File::open(&file_path)
        .with_context(|| format!("Could not open file `{}`", file_path))?;

    Ok(BufReader::new(input_file))
}