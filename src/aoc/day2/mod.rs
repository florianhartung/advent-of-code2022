use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::aoc::AocOutput;
use crate::aoc::day2::rock_paper_scissors::{RpsRound, RpsShape};

mod rock_paper_scissors;

trait FirstChar {
    fn first_char(self) -> Option<char>;
}

impl FirstChar for &str {
    fn first_char(self) -> Option<char> {
        self.chars().next()
    }
}

pub fn solve(input: BufReader<File>) -> AocOutput {
    let score: u32 = input.lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut line = line.chars().filter(|c| c.is_alphabetic());
            let (opp, player) = (line.next().unwrap(), line.next().unwrap());
            line_to_rps_round(opp, player)
        })
        .map(RpsRound::into_score)
        .sum();

    AocOutput::from(score, "Not yet implemented")
}

fn line_to_rps_round(opponent: char, player: char) -> RpsRound {
    RpsRound {
        opponent: char_to_rps_shape(opponent),
        player: char_to_rps_shape(player),
    }
}

fn char_to_rps_shape(c: char) -> RpsShape {
    match c {
        'A' | 'X' => RpsShape::ROCK,
        'B' | 'Y' => RpsShape::PAPER,
        'C' | 'Z' => RpsShape::SCISSORS,
        x => panic!("Invalid character {}", x),
    }
}