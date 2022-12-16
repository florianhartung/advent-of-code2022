use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::aoc::{AocDaySolver, AocOutput};

pub struct Day1;
impl AocDaySolver for Day1 {

    fn solve(input: BufReader<File>) -> AocOutput {
        let max_calories = input.lines()
            .map_while(Result::ok)
            .collect::<Vec<String>>()
            .split(String::is_empty)
            .map(|slice| slice.iter().flat_map(|line| line.trim().parse::<u32>()).sum())
            .max();

        AocOutput::from(max_calories.unwrap_or(0), 0)
    }
}