use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(filename: &String) -> Vec<(usize, isize)> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.expect("Could not read line").parse().expect("Could not parse number"))
        .enumerate()
        .collect()
}

fn mix(coordinates: &[(usize, isize)], rounds: isize) -> Vec<(usize, isize)> {
    let mut coords = coordinates.to_vec();
    let coords_len = coords.len() as isize - 1;

    for _ in 0..rounds {
        for &value in coordinates {
            let old_index = coords.iter().position(|&v| v == value).expect("No index found");
            let new_index = (old_index as isize + value.1).rem_euclid(coords_len);
            let value = coords.remove(old_index);
            coords.insert(new_index as usize, value);
        }
    }

    coords
}

fn solve(coordinates: &[(usize, isize)], key: isize, rounds: isize) -> isize {
    let coordinates: Vec<_> = coordinates.iter().cloned().map(|(i, v)| (i, v * key)).collect();
    let mixed = mix(&coordinates, rounds);
    let zero_pos = mixed.iter().position(|&value| value.1 == 0).expect("No zero position found");

    [1000, 2000, 3000].iter().map(|offset| mixed[(zero_pos + offset) % mixed.len()].1).sum()
}

fn main() {
    let filename = args().nth(1).expect("No input file given");
    let coordinates = read_file(&filename);

    println!("Part 1: {}", solve(&coordinates, 1, 1));
    println!("Part 2: {}", solve(&coordinates, 811589153, 10));
}
