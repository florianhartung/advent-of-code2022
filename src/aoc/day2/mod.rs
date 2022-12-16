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
        .map(line_to_rps_round)
        .map(RpsRound::into_score)
        .sum();

    AocOutput::from("Not anymore calculated", score)
}

fn line_to_rps_round(line: String) -> RpsRound {
    let mut line = line.chars().filter(|c| c.is_alphabetic());
    let opponent = char_to_rps_shape(line.next().unwrap());

    let player = get_player_shape(opponent, line.next().unwrap());

    RpsRound {
        opponent,
        player,
    }
}

fn char_to_rps_shape(c: char) -> RpsShape {
    match c {
        'A' => RpsShape::Rock,
        'B' => RpsShape::Paper,
        'C' => RpsShape::Scissors,
        x => panic!("Invalid character {}", x),
    }
}

fn get_player_shape(opponent_shape: RpsShape, c: char) -> RpsShape {
    match c {
        'X' => opponent_shape.get_wins_against(),
        'Y' => opponent_shape,
        'Z' => opponent_shape.get_looses_against(),
        x => panic!("Invalid character {}", x),
    }
}