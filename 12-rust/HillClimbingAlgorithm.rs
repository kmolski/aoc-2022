use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(filename: &String) -> Vec<Vec<u8>> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.expect("Could not read line").bytes().collect()).collect()
}

fn find_around((x, y): (usize, usize), (max_x, max_y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (x, y) = (x as isize, y as isize);
    let (max_x, max_y) = (max_x as isize, max_y as isize);
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
        .filter(move |(x, y)| (0..max_x).contains(x) && (0..max_y).contains(y))
        .map(|(x, y)| (x as usize, y as usize))
}

fn visit_position(pos: (usize, usize), prev_dist: i64, coord_to_steps: &mut HashMap<(usize, usize), i64>) {
    let new_shortest = prev_dist + 1;
    if let None = coord_to_steps.get(&pos).filter(|&&dist| new_shortest >= dist) {
        coord_to_steps.insert(pos, new_shortest);
    }
}

fn is_reachable(height: u8, prev_height: u8) -> bool {
    height == b'S' || height + 1 >= prev_height
}

fn solve_part(map: &Vec<Vec<u8>>, from_square: u8) -> i64 {
    let dimensions = (map[0].len(), map.len());
    let finish_coord = map.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().enumerate().find(|(_, c)| **c == b'E').map(|(x, _)| (x, y)))
        .unwrap();

    let mut coord_to_steps = HashMap::new();
    let mut to_visit = VecDeque::from([(finish_coord, -1, b'z' + 1)]);
    while !to_visit.is_empty() {
        let position = to_visit.pop_front().unwrap();

        visit_position(position.0, position.1, &mut coord_to_steps);
        let neighborhood = find_around(position.0, dimensions);
        for pos @ (x, y) in neighborhood.filter(|&(x, y)| is_reachable(map[y][x], position.2)) {
            if let None = coord_to_steps.get(&pos) {
                let new = (pos, coord_to_steps[&position.0], map[y][x]);
                if !to_visit.contains(&new) {
                    to_visit.push_back(new);
                }
            }
        }
    }

    let start_coord = map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().filter(|(_, c)| **c == from_square).map(move |(x, _)| (x, y)))
        .filter(|pos| coord_to_steps.contains_key(&pos))
        .min_by_key(|pos| coord_to_steps[&pos])
        .unwrap();
    coord_to_steps[&start_coord]
}

fn solve(filename: &String) {
    let map = read_file(filename);

    println!("Part 1: {}", solve_part(&map, b'S'));
    println!("Part 2: {}", solve_part(&map, b'a'));
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    solve(&filename);
}
