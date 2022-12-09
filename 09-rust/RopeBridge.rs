use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Move {
    dir: Direction,
    length: u32,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position(i32, i32);

impl Position {
    fn nudge(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.0 -= 1,
            Direction::Right => self.1 += 1,
            Direction::Down => self.0 += 1,
            Direction::Left => self.1 -= 1,
        }
    }

    fn max_axis_diff(&self, Position(y, x): Position) -> i32 {
        i32::max((y - self.0).abs(), (x - self.1).abs())
    }

    fn follow(&mut self, other @ Position(y, x): Position) {
        if self.max_axis_diff(other) >= 2 {
            let y_diff = (y - self.0).signum();
            let x_diff = (x - self.1).signum();
            self.0 += y_diff;
            self.1 += x_diff;
        }
    }
}

fn read_direction(token: &str) -> Direction {
    match token {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        _ => panic!("Unknown value: {token}"),
    }
}

fn read_moves(filename: String) -> Vec<Move> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            let line = l.expect("Could not read line");
            let tokens = line.trim().split_once(' ').expect("Could not split line");
            Move {
                dir: read_direction(tokens.0),
                length: tokens.1.parse().expect("Could not parse length"),
            }
        })
        .collect()
}

fn solve<const KNOTS: usize>(moves: &Vec<Move>) -> usize {
    let mut knots = [Position(0, 0); KNOTS];
    let mut visited: HashSet<Position> = HashSet::from([knots[0]]);

    for m in moves {
        for _ in 0..m.length {
            knots[0].nudge(m.dir); // move head knot
            for i in 1..KNOTS {
                knots[i].follow(knots[i - 1]);
            }
            visited.insert(knots[KNOTS - 1]); // record tail knot
        }
    }

    visited.len()
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let moves = read_moves(filename);

    println!("Part 1: {}", solve::<2>(&moves));
    println!("Part 2: {}", solve::<10>(&moves));
}
