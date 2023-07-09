use std::env::args;
use std::fs::File;
use std::io::Read;

const TILE: u8 = b'.';
const WALL: u8 = b'#';
const VOID: u8 = b' ';

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn read_file(filename: &String) -> (Vec<Vec<u8>>, String) {
    let mut file = File::open(filename).expect("Could not open file");

    let mut all = String::new();
    file.read_to_string(&mut all).expect("Could not read file");

    let (map, moves) = all.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<u8>> = map.lines().map(|l| l.bytes().collect()).collect();

    let max_row_len: usize = map.iter().map(|row| row.len()).max().unwrap();
    map.iter_mut().for_each(|row| row.extend((row.len()..max_row_len).map(|_| VOID)));
    (map, moves.to_string())
}

fn move_left(direction: &mut Direction) {
    match direction { // move counter-clockwise
        Direction::North => *direction = Direction::West,
        Direction::East  => *direction = Direction::North,
        Direction::South => *direction = Direction::East,
        Direction::West  => *direction = Direction::South,
    }
}

fn move_right(direction: &mut Direction) {
    match direction { // move clockwise
        Direction::North => *direction = Direction::East,
        Direction::East  => *direction = Direction::South,
        Direction::South => *direction = Direction::West,
        Direction::West  => *direction = Direction::North,
    }
}

fn apply_diff((x, y): (i64, i64), transpose: (i64, i64), map: &[Vec<u8>]) -> (i64, i64) {
    (
        (x + transpose.0).rem_euclid(map[0].len() as i64),
        (y + transpose.1).rem_euclid(map.len() as i64)
    )
}

fn walk(map: &[Vec<u8>], pos: &mut (i64, i64), count: i32, direction: &mut Direction, face_size: i64) {
    for _ in 0..count {
        let diff = match direction {
            Direction::North => (0, -1),
            Direction::East  => (1, 0),
            Direction::South => (0, 1),
            Direction::West  => (-1, 0)
        };
        let prev = *pos;
        let prev_tile = map[pos.1 as usize][pos.0 as usize];
        let x = pos.0 % face_size;
        let y = pos.1 % face_size;
        let (new_dir, mut new) = match (prev_tile, *direction) {
            (b'A', Direction::North) if y == 0 => {
                (Direction::South, (3 * face_size - 1 - x, 0))
            }
            (b'A', Direction::South) if y == face_size - 1 => {
                (Direction::North, (3 * face_size - 1 - x, 3 * face_size - 1))
            }
            (b'B', Direction::North) if y == 0 => {
                (Direction::East,  (2 * face_size, x))
            }
            (b'B', Direction::South) if y == face_size - 1 => {
                (Direction::East,  (2 * face_size, 3 * face_size - 1 - x))
            }
            (b'E', Direction::North) if y == 0 => {
                (Direction::West,  (3 * face_size - 1, face_size - 1 - x))
            }
            (b'E', Direction::South) if y == face_size - 1 => {
                (Direction::West,  (3 * face_size - 1, 2 * face_size + x))
            }
            (b'C', Direction::West) if x == 0 => {
                (Direction::South, (face_size + y, face_size))
            }
            (b'C', Direction::North) if y == 0 => {
                (Direction::South, (face_size - 1 - x, face_size))
            }
            (b'C', Direction::East) if x == face_size - 1 => {
                (Direction::South, (4 * face_size - 1 - y, face_size))
            }
            (b'D', Direction::West) if x == 0 => {
                (Direction::North, (2 * face_size - 1 - y, 2 * face_size - 1))
            }
            (b'D', Direction::South) if y == face_size - 1 => {
                (Direction::North, (face_size - 1 - x, 2 * face_size - 1))
            }
            (b'D', Direction::East) if x == face_size - 1 => {
                (Direction::North, (3 * face_size + y, 2 * face_size - 1))
            }
            _ => (*direction, apply_diff(*pos, diff, map))
        };

        while map[new.1 as usize][new.0 as usize] == VOID {
            match map[new.1 as usize][new.0 as usize] {
                VOID => { new = apply_diff(new, diff, map) }
                WALL | TILE | b'A'..=b'E' => break,
                _ => unreachable!("Invalid map tile @ {new:?}")
            }
        }

        if map[new.1 as usize][new.0 as usize] == WALL {
            *pos = prev;
            break
        } else {
            *direction = new_dir;
            *pos = new;
        }
    }
}

fn facing(direction: Direction) -> i64 {
    match direction {
        Direction::East  => 0,
        Direction::South => 1,
        Direction::West  => 2,
        Direction::North => 3
    }
}

fn solve(map: &[Vec<u8>], moves: &str) -> i64 {
    let face_size = map.iter().map(|row| row.len()).max().unwrap() as i64 / 4;
    let mut pos = (map[0].iter().enumerate().find(|(_, t)| **t == TILE || **t == b'C').unwrap().0 as i64, 0);
    let mut direction = Direction::East;

    for m in moves.trim_end().split_inclusive(&['L', 'R']) {
        let steps = m.trim_end_matches(&['L', 'R']).parse().expect("Could not parse move number");
        walk(map, &mut pos, steps, &mut direction, face_size);
        match m.chars().last().unwrap() {
            'L' => move_left(&mut direction),
            'R' => move_right(&mut direction),
            _ => {}
        }
    }
    (pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + facing(direction)
}

fn main() {
    let filename = args().nth(1).unwrap();
    let (map, moves) = read_file(&filename);

    println!("Answer: {}", solve(&map, &moves));
}
