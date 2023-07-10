use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_snafu(s: &str) -> i64 {
    let mut accum = 0;

    for (power, c) in s.chars().rev().enumerate() {
        let digit = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!("Invalid SNAFU digit: {c}")
        };
        accum += digit * 5_i64.pow(power as u32);
    }
    accum
}

fn read_file(filename: &String) -> Vec<i64> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| parse_snafu(&line.expect("Could not read line")))
        .collect()
}

fn show_snafu(mut number: i64) -> String {
    let mut carry = 0;
    let mut digits = Vec::new();
    while number > 0 || carry != 0 {
        number += carry;
        carry = 0;
        match number % 5 {
            0 => digits.push('0'),
            1 => digits.push('1'),
            2 => digits.push('2'),
            3 => { digits.push('='); carry = 1; }
            4 => { digits.push('-'); carry = 1; }
            d => unreachable!("No SNAFU digit for remainder {d}")
        }
        number /= 5;
    }

    digits.iter().rev().collect()
}

fn main() {
    let filename = args().nth(1).expect("No input file given");
    let numbers = read_file(&filename);
    let sum = numbers.iter().sum::<i64>();

    println!("Part 1: {}", show_snafu(sum));
}
