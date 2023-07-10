use std::collections::{HashMap, HashSet, VecDeque};
use std::env::args;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::ops::Add;

use itertools::Itertools;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Position(i64, i64);

type MoveDiff = (i64, i64);
type ScanArea<'a> = &'a [MoveDiff; 3];
type MoveAction<'a> = (ScanArea<'a>, MoveDiff);

const MOVE_ACTIONS: [MoveAction; 4] = [
    (&[(-1, -1), (0, -1), (1, -1)], (0, -1)),
    (&[(-1,  1), (0,  1), (1,  1)], (0,  1)),
    (&[(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
    (&[(1,  -1), (1,  0), (1,  1)], (1,  0))
];

impl Add<(i64, i64)> for Position {
    type Output = Position;

    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn read_line(row_n: i64, row: String) -> Vec<Position> {
    row.chars()
        .enumerate()
        .filter(|&(_, c)| c == '#')
        .map(&|(col_n, _)| Position(col_n as i64, row_n))
        .collect()
}

fn read_file(filename: &String) -> HashSet<Position> {
    let file = File::open(filename).expect("Could not open file");

    let reader = BufReader::new(file);
    reader.lines()
        .enumerate()
        .flat_map(|row| read_line(row.0 as i64, row.1.expect("Could not read line")))
        .collect()
}

fn should_move(elves: &HashSet<Position>, node: Position) -> bool {
    (-1..=1).cartesian_product(-1..=1)
        .filter(|&d| d != (0, 0))
        .any(|diff| elves.contains(&(node + diff)))
}

fn is_free_space(elves: &HashSet<Position>, diffs: &[(i64, i64)], node: Position) -> bool {
    diffs.iter().all(|&diff| !elves.contains(&(node + diff)))
}

fn is_only_intention(intentions: &HashMap<Position, Vec<Position>>, node: Position, target: Position) -> bool {
    intentions[&target].len() == 1 && intentions[&target][0] == node
}

fn simulate_round(elves: &HashSet<Position>, directions: &VecDeque<MoveAction>,
                  moves: &mut HashMap<Position, Position>, intentions: &mut HashMap<Position, Vec<Position>>) -> HashSet<Position> {
    for &node in elves.iter().filter(|&&n| should_move(elves, n)) {
        let Some(diff) = directions.iter()
            .find(|&diffs| is_free_space(elves, diffs.0, node))
            .map(|&diff| diff.1)
            else { continue };

        moves.insert(node, node + diff);
        intentions.entry(node + diff).or_insert_with(Vec::new).push(node);
    }

    let mut new_elves = HashSet::new();
    for &node in elves {
        if let Some(target) = moves.get(&node).filter(|&&t| is_only_intention(intentions, node, t)) {
            new_elves.insert(*target);
        } else {
            new_elves.insert(node);
        }
    }
    new_elves
}

fn solve_part_1(elves: &HashSet<Position>, rounds: i32) -> i64 {
    let mut elves = elves.clone();
    let mut directions: VecDeque<MoveAction> = VecDeque::from(MOVE_ACTIONS);

    for _ in 0..rounds {
        let mut moves = HashMap::new();
        let mut intentions = HashMap::new();

        elves = simulate_round(&elves, &directions, &mut moves, &mut intentions);
        directions.rotate_left(1);
    }

    let max_x = elves.iter().map(|&pos| pos.0).max().unwrap();
    let min_x = elves.iter().map(|&pos| pos.0).min().unwrap();
    let max_y = elves.iter().map(|&pos| pos.1).max().unwrap();
    let min_y = elves.iter().map(|&pos| pos.1).min().unwrap();
    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i64
}

fn solve_part_2(elves: &HashSet<Position>) -> i64 {
    let mut elves = elves.clone();
    let mut directions: VecDeque<MoveAction> = VecDeque::from(MOVE_ACTIONS);

    let mut i = 0;
    loop {
        let mut moves = HashMap::new();
        let mut intentions = HashMap::new();

        let new_elves = simulate_round(&elves, &directions, &mut moves, &mut intentions);

        i += 1;
        if elves == new_elves { return i; }

        directions.rotate_left(1);
        elves = new_elves;
    }
}

fn main() {
    let filename = args().nth(1).unwrap();
    let elves = read_file(&filename);

    println!("Part 1: {}", solve_part_1(&elves, 10));
    println!("Part 2: {}", solve_part_2(&elves));
}
