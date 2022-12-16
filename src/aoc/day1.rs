use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

use crate::aoc::AocOutput;

pub fn solve(input: BufReader<File>) -> AocOutput {
    let top_3_elves = input.lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<u32>().ok())
        .batching(|it| it.while_some().sum1::<u32>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .collect::<Vec<u32>>();

    let elf_with_max_calories = *top_3_elves.first().unwrap_or(&0);
    let top3_elves_with_max_calories: u32 = top_3_elves.into_iter().sum();

    AocOutput::from(elf_with_max_calories, top3_elves_with_max_calories)
}