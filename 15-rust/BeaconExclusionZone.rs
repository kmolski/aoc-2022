use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position(isize, isize);

impl Position {
    fn manhattan_dist(self, Position(x, y): &Position) -> isize {
        (self.0 - x).abs() + (self.1 - y).abs()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Sensor {
    self_pos: Position,
    peer_pos: Position,
    distance: isize
}

fn read_file(filename: &String) -> Vec<Sensor> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let regex = Regex::new(r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();

    reader.lines()
        .map(|line| read_line(&line.expect("Could not read line"), &regex))
        .collect()
}

fn read_line(line: &str, regex: &Regex) -> Sensor {
    let captures = regex.captures(line).expect(&format!("Line does not match: {line}"));
    let self_pos = Position(captures[1].parse().unwrap(), captures[2].parse().unwrap());
    let peer_pos = Position(captures[3].parse().unwrap(), captures[4].parse().unwrap());

    Sensor { self_pos, peer_pos, distance: self_pos.manhattan_dist(&peer_pos) }
}

fn solve_part_1(sensors: &Vec<Sensor>, row_index: isize) -> usize {
    let mut unavailable_positions = HashSet::new();

    for sensor in sensors {
        let y_distance = (sensor.self_pos.1 - row_index).abs();

        if y_distance <= sensor.distance {
            let x_offset = sensor.distance - y_distance;
            for x in (sensor.self_pos.0 - x_offset)..=(sensor.self_pos.0 + x_offset) {
                unavailable_positions.insert(Position(x, row_index));
            }
        }
    }
    for sensor in sensors {
        unavailable_positions.remove(&Position(sensor.peer_pos.0, sensor.peer_pos.1));
    }

    unavailable_positions.len()
}

fn check_pos(sensors: &Vec<Sensor>, skip_id: usize, max_dimension: isize, pos: Position) -> Option<Position> {
    if pos.0 < 0 || pos.0 > max_dimension || pos.1 < 0 || pos.1 > max_dimension {
        return None;
    }

    for (_, other) in sensors.iter().enumerate().filter(|(other_id, _)| *other_id != skip_id) {
        if pos.manhattan_dist(&other.self_pos) <= other.distance {
            return None;
        }
    }
    return Some(pos)
}

fn find_empty_pos(sensors: &Vec<Sensor>, max_dimension: isize) -> Position {
    for (id, sensor) in sensors.iter().enumerate() {
        let dist = sensor.distance;
        for i in 0..dist {
            let top = Position(sensor.self_pos.0 - i, sensor.self_pos.1 - dist - 1 + i);
            let left = Position(sensor.self_pos.0 - dist - 1 + i, sensor.self_pos.1 + i);
            let right = Position(sensor.self_pos.0 + dist + 1 - i, sensor.self_pos.1 - i);
            let bottom = Position(sensor.self_pos.0 + i, sensor.self_pos.1 + dist + 1 - i);

            if let Some(pos) = check_pos(sensors, id, max_dimension, top)
                    .or(check_pos(sensors, id, max_dimension, left))
                    .or(check_pos(sensors, id, max_dimension, right))
                    .or(check_pos(sensors, id, max_dimension, bottom)) {
                return pos;
            }
        }
    }
    unreachable!()
}

fn solve_part_2(sensors: &Vec<Sensor>, max_dimension: isize) -> isize {
    let empty_pos = find_empty_pos(sensors, max_dimension);
    empty_pos.0 * 4_000_000 + empty_pos.1
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let beacons = read_file(&filename);

    println!("Part 1: {}", solve_part_1(&beacons, 2_000_000));
    println!("Part 2: {}", solve_part_2(&beacons, 4_000_000));
}
