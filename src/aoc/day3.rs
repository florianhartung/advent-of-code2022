use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::aoc::AocOutput;

pub fn solve(input: BufReader<File>) -> AocOutput {
    let total_priorities: u32 = input.lines()
        .map_while(Result::ok)
        .map(get_double_item)
        .map(get_item_priority)
        .sum();

    AocOutput::first(total_priorities)
}

fn get_double_item(rucksack: String) -> u8 {
    let bytes = rucksack.into_bytes();

    let (left_compartment, right_compartment) = bytes.split_at(bytes.len() / 2);

    // Bits: MSB->   0(x12) Z..A z..a   <-LSB
    let mut items: u64 = 0;

    for &item in left_compartment {
        let item_index = get_item_priority(item) - 1;
        items |= 1 << item_index;
    }

    for &item in right_compartment {
        let item_index = get_item_priority(item) - 1;

        if items & (1 << item_index) > 0 {
            return item;
        }
    }

    unreachable!("Could not find an item that exists in both compartments")
}

fn item_index_in_alphabet(item: u8) -> u32 {
    (item.to_ascii_lowercase() - 'a' as u8) as u32
}

fn get_item_priority(item: u8) -> u32 {
    if item.is_ascii_lowercase() {
        return item_index_in_alphabet(item) + 1;
    }
    return item_index_in_alphabet(item) + 26 + 1;
}