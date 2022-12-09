use std::fs::File;
use std::io::{BufRead, BufReader};


pub(crate) fn day1_solution(input: BufReader<File>) -> u32 {
    input.lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .split(String::is_empty)
        .map(|slice| slice.iter().flat_map(|line| line.trim().parse::<u32>()).sum())
        .max()
        .unwrap_or_else(|| 0)
}