extern crate core;

use std::borrow::Borrow;
use std::env::Args;
use std::fs::File;
use std::future::{IntoFuture};
use std::io::{BufReader, BufWriter, Write};

use anyhow::{Context, Result};
use chrono::Datelike;
use clap::Parser;

use crate::config::Config;

mod aoc;
mod config;

const INPUT_PATH_FOR_DAY: fn(u32) -> String = |day: u32| format!("./inputs/day{}.txt", day);

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config: Config = args.into();

    println!("Selected day {}", config.day);

    let input = read_inputs_for_day(&config).await?;

    let answer = aoc::solve(config.day, input)?;

    println!("[Day {}] {}, {}", config.day, answer.first, answer.second);

    Ok(())
}

/// Returns the inputs for a day specified in the `Config`.
///
/// If the inputs for that day are not already locally stored,
/// they will be downloaded using the session from the `RemoteConfig` inside of the `Config`
/// and then read from the just downloaded file.
async fn read_inputs_for_day(config: &Config) -> Result<BufReader<File>> {
    let from_file = read_input_file_for_day(config.day);

    if from_file.is_ok() {
        return from_file;
    }


    /*let f = poll_fn(|_| {
        download_inputs_for_day(&config)
            .with_context()
            .map(read_input_file)
    });*/

    todo!()
}

async fn download_inputs_for_day(config: &Config) -> Result<File> {
    let session_token: &str = config.remote_config
        .as_ref()
        .with_context(|| "Could not download input because session token is missing")?
        .session
        .borrow();

    let mut response = reqwest::Client::new()
        .get(format!("https://adventofcode.com/{}/day/{}/input", config.year, config.day))
        .header("session", session_token)
        .send()
        .await?;

    let file_path = INPUT_PATH_FOR_DAY(config.day);
    let file = File::open(file_path)?;
    let mut writer = BufWriter::new(file);

    while let Some(chunk) = response.chunk().await? {
        writer.write(chunk.as_ref())?;
    }

    let file = writer.into_inner()?;

    Ok(file)
}

fn read_input_file(file: File) -> BufReader<File> {
    BufReader::new(file)
}

fn read_input_file_for_day(day: u32) -> Result<BufReader<File>> {
    let file_path = INPUT_PATH_FOR_DAY(day);

    let input_file = File::open(&file_path)
        .with_context(|| format!("Could not open file `{}`", file_path))?;

    Ok(read_input_file(input_file))
}