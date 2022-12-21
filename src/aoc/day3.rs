use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

use crate::aoc::AocOutput;

pub fn solve(input: BufReader<File>) -> AocOutput {
    let total_priorities: u32 = input.lines()
        .map_while(Result::ok)
        .batching(|it| {
            let next3 = it.take(3).collect_vec();
            Option::from(next3)
                .filter(|vec| !vec.is_empty())
        })
        .map(get_common_item_priority)
        .sum();

    AocOutput::second(total_priorities)
}

fn get_common_item_priority(group: Vec<String>) -> u32 {
    let mut item_mask = group.into_iter()
        .map(|rucksack| items_in_rucksack(rucksack.into_bytes()))
        .fold(0xFFFF_FFFF_FFFF_FFFF, |prev, next| prev & next);

    assert_eq!(item_mask.count_ones(), 1, "More than 1 item shared in a group");

    let mut item_priority = 1;
    while item_mask & 1 == 0 || item_priority == 53 {
        item_mask >>= 1;
        item_priority += 1;
    }
    return item_priority;
}

fn items_in_rucksack(rucksack: Vec<u8>) -> u64 {
    let mut items: u64 = 0;

    for item in rucksack {
        let item_index = get_item_priority(item) - 1;
        items |= 1 << item_index;
    }

    items
}

fn _get_double_item(rucksack: String) -> u8 {
    let bytes = rucksack.into_bytes();

    let (left_compartment, right_compartment) = bytes.split_at(bytes.len() / 2);

    // Bits: MSB->   0(x12) Z..A z..a   <-LSB
    let items: u64 = items_in_rucksack(left_compartment.to_vec());

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