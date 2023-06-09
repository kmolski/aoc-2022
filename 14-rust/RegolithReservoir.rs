use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Block::Sand;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Block { Rock, Sand }

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position(usize, usize);

fn read_file(filename: &String) -> HashMap<Position, Block> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    reader.lines()
        .flat_map(|line| read_line(&line.expect("Could not read line")))
        .map(|pos| (pos, Block::Rock))
        .collect()
}

fn read_line(line: &str) -> impl Iterator<Item=Position> {
    let mut rocks: Vec<_> = line.split(" -> ")
        .flat_map(|pos| pos.split_once(',').and_then(|(x, y)| Some(Position(x.parse().ok()?, y.parse().ok()?))))
        .collect();

    let mut rock_surface = Vec::new();
    for pair in rocks.windows(2) {
        match *pair {
            [Position(x1, y1), Position(x2, y2)] if x1 == x2 => {
                for y in (y1.min(y2) + 1)..y1.max(y2) {
                    rock_surface.push(Position(x1, y));
                }
            },
            [Position(x1, y1), Position(x2, y2)] if y1 == y2 => {
                for x in (x1.min(x2) + 1)..x1.max(x2) {
                    rock_surface.push(Position(x, y1));
                }
            },
            _ => unreachable!()
        }
    }

    rocks.extend(&rock_surface);
    rocks.into_iter()
}

fn try_move(blocks: &HashMap<Position, Block>, x: &mut usize, y: &mut usize) -> bool {
    if !blocks.contains_key(&Position(*x, *y + 1)) { // falls down
        *y += 1;
        true
    } else if !blocks.contains_key(&Position(*x - 1, *y + 1)) { // falls left-down
        *x -= 1;
        *y += 1;
        true
    } else if !blocks.contains_key(&Position(*x + 1, *y + 1)) { // falls right-down
        *x += 1;
        *y += 1;
        true
    } else { // comes to rest
        false
    }
}

fn solve_part_1(rocks: &HashMap<Position, Block>) -> usize {
    let mut blocks = rocks.clone();
    let max_y = blocks.iter().map(|(&Position(_, y), _)| y).max().unwrap();

    let (mut sand_x, mut sand_y) = (500, 0);
    while sand_y <= max_y {
        if !try_move(&blocks, &mut sand_x, &mut sand_y) { // sand block came to rest
            blocks.insert(Position(sand_x, sand_y), Sand);
            (sand_x, sand_y) = (500, 0);
            continue;
        }
    }
    return blocks.iter().filter(|(_, &block)| block == Sand).count();
}

fn solve_part_2(rocks: &HashMap<Position, Block>) -> usize {
    let mut blocks = rocks.clone();
    let max_y = blocks.iter().map(|(&Position(_, y), _)| y).max().unwrap();

    let (mut sand_x, mut sand_y) = (500, 0);
    loop {
        if sand_y > max_y || !try_move(&blocks, &mut sand_x, &mut sand_y) {
            blocks.insert(Position(sand_x, sand_y), Sand);
            (sand_x, sand_y) = (500, 0);
            if !try_move(&blocks, &mut sand_x, &mut sand_y) { // sand block is blocked
                return blocks.iter().filter(|(_, &block)| block == Sand).count() + 1;
            }
            continue;
        }
    }
}

fn main() {
    let filename = args().nth(1).unwrap();
    let rocks = read_file(&filename);

    println!("Part 1: {}", solve_part_1(&rocks));
    println!("Part 2: {}", solve_part_2(&rocks));
}
