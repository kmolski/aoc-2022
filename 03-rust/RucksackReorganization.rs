use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_rucksacks(filename: String) -> Vec<String> {
    let file = File::open(filename).expect("Could not open file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}

fn split_in_half(str: &String) -> (&str, &str) {
    assert_eq!(str.len() % 2, 0); // length must be even
    str.split_at(str.len() / 2)
}

fn find_repeated_items((first, second): (&str, &str)) -> HashSet<u8> {
    let first_set: HashSet<_> = first.bytes().collect();
    let second_set: HashSet<_> = second.bytes().collect();
    first_set.intersection(&second_set).map(|c| *c).collect()
}

fn item_to_priority(c: u8) -> u32 {
    match c {
        b'a'..=b'z' => (c - b'a' + 1) as u32,
        b'A'..=b'Z' => (c - b'A' + 27) as u32,
        _ => unreachable!("Value out of range"),
    }
}

fn part_1(rucksacks: &Vec<String>) -> u32 {
    rucksacks.iter()
        .map(split_in_half)
        .flat_map(find_repeated_items)
        .map(item_to_priority)
        .sum()
}

fn find_group_badge(rucksacks: &[String]) -> u8 {
    rucksacks.iter()
        .map(|sack| sack.bytes().collect::<HashSet<_>>())
        .reduce(|accum, sack| accum.intersection(&sack).map(|c| *c).collect())
        .unwrap().into_iter()
        .next().expect("No badge item found")
}

fn part_2(rucksacks: &Vec<String>) -> u32 {
    rucksacks.chunks(3)
        .map(find_group_badge)
        .map(item_to_priority)
        .sum()
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let rucksacks = read_rucksacks(filename);

    println!("Part 1: {}", part_1(&rucksacks));
    println!("Part 2: {}", part_2(&rucksacks));
}
