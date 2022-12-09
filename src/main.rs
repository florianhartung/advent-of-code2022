use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use std::process::{ExitCode, Termination};

use crate::day1::day1_solution;

mod day1;

pub(crate) enum Error {
    DayInput,
    InvalidDay,
    FileInput,
    InputParsing,
    #[allow(dead_code)]
    Solution(String),
}

type Result<T> = std::result::Result<T, Error>;

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DayInput => write!(f, "Could not read input"),
            Error::InvalidDay => write!(f, "Expected a day from 1 to 24"),
            Error::FileInput => write!(f, "Could not read file for requested day"),
            Error::InputParsing => write!(f, "Invalid input file contents"),
            Error::Solution(msg) => write!(f, "Error whilst trying to execute solution for current day: {}", msg),
        }
    }
}

fn main() -> ExitCode {
    let f = unsafe { File::open("./res/day1.txt").unwrap_unchecked() };
    let reader = BufReader::new(f);
    return ExitCode::from(unsafe { day1_solution(reader) } as u8);


    /*let day = read_day_from_stdin("Day: ")?;

    let input_file = open_input_file(day)
        .map_err(|_| Error::FileInput)?;

    match day {
        1 => day1_solution(input_file),
        _ => {}
    };

    Ok(())*/
}

fn read_day_from_stdin(prompt: &str) -> Result<u32> {
    let mut day: String = String::new();

    print!("{prompt}");
    io::stdout()
        .flush()
        .map_err(|_| Error::DayInput)?;

    io::stdin()
        .read_line(&mut day)
        .map_err(|_| Error::DayInput)?;

    let day: u32 = day.trim()
        .parse::<u32>()
        .map_err(|_| Error::InputParsing)?;

    match day {
        1..=24 => Ok(day),
        _ => Err(Error::InvalidDay),
    }
}

unsafe fn open_input_file(day: u32) -> BufReader<File> {
    let input_file = File::open(format!("./res/day{}.txt", day)).unwrap_unchecked();

    BufReader::new(input_file)
}