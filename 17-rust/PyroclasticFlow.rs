use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::io::Read;

use itertools::sorted;

const WIDTH: isize = 7;
const SHAPES: [&str; 5] = [
"####",
".#.
###
.#.",
"###
..#
..#",
"#
#
#
#",
"##
##",
];

#[derive(Copy, Clone, Eq, Ord, Hash, PartialEq, PartialOrd)]
struct Point(isize, isize);

type PointCloud = HashSet<Point>;

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Could not open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read file contents");

    String::from(content.trim())
}

fn convert_shape(shape: &str) -> PointCloud {
    let mut cloud = HashSet::new();
    for (y, row) in shape.split('\n').enumerate() {
        for (x, char) in row.chars().enumerate() {
            if char == '#' {
                cloud.insert(Point(x as isize, y as isize));
            }
        }
    }
    cloud
}

fn increment_coords(current: &PointCloud, offset: (isize, isize)) -> HashSet<Point> {
    current.iter()
        .map(|p| Point(p.0 + offset.0, p.1 + offset.1))
        .collect()
}

fn shift_overlaps(current: &PointCloud, offset: (isize, isize), settled: &PointCloud) -> bool {
    current.iter()
        .map(|p| Point(p.0 + offset.0, p.1 + offset.1))
        .any(|p| p.0 < 0 || p.0 >= WIDTH || settled.contains(&p) || p.1 <= 0)
}

fn get_top_surface(settled: &PointCloud, max_y: isize) -> Vec<Point> {
    sorted(settled.iter().filter(|p| max_y - p.1 <= 30))
        .cloned()
        .collect()
}

fn make_cache_key(jet_i: usize, max_y: isize, settled: &PointCloud, new: &PointCloud) -> String {
    let path = get_top_surface(settled, max_y).iter()
        .map(|Point(x, y)| format!("{},{}", x, max_y - y))
        .collect::<Vec<_>>().join(":");
    let new = sorted(new.iter())
        .map(|Point(x, y)| format!("{},{}", x, max_y - y))
        .collect::<Vec<_>>().join(":");
    format!("{jet_i}#{path}#{new}")
}

fn update_cache(cache: &mut HashMap<String, (usize, isize)>, shape_i: usize, jet_i: usize, max_y: isize, settled: &PointCloud, new: &PointCloud) {
    cache.insert(make_cache_key(jet_i, max_y, settled, new), (shape_i, max_y));
}

fn solve(limit: usize, jets: &str) -> isize {
    let mut jet_i = 0;
    let mut shape_i = 0;
    let mut settled = HashSet::new();
    let mut cache = HashMap::new();
    loop {
        for shape in SHAPES {
            let max_y = settled.iter().map(|p: &Point| p.1).max().unwrap_or(0);
            if settled.len() > 10000 {
                settled.retain(|p| max_y - p.1 <= 5000);
            }

            let current = convert_shape(shape);
            // rocks appear two units away from the left wall, four units above the floor or highest rock
            let mut shift = (2, max_y + 4);

            loop {
                let x_shift = if jets.chars().nth(jet_i).unwrap() == '>' { 1 } else { -1 };
                if !shift_overlaps(&current, (shift.0 + x_shift, shift.1), &settled) {
                    shift.0 += x_shift;
                }
                jet_i = (jet_i + 1) % jets.len();

                if !shift_overlaps(&current, (shift.0, shift.1 - 1), &settled) {
                    shift.1 -= 1;
                } else {
                    break;
                }
            }

            let mut new = increment_coords(&current, shift);
            if let Some((prev_i, prev_y)) = cache.get(&make_cache_key(jet_i, max_y, &settled, &new)) {
                let cycle_i = shape_i - prev_i;
                if shape_i + cycle_i <= limit {
                    settled = increment_coords(&settled, (0, max_y - prev_y));
                    new = increment_coords(&new, (0, max_y - prev_y));
                    shape_i += cycle_i;
                }
            }
            update_cache(&mut cache, shape_i, jet_i, max_y, &settled, &new);
            settled.extend(new);

            shape_i += 1;
            if shape_i == limit {
                return settled.iter().map(|p| p.1).max().unwrap()
            }
        }
    }
}

fn main() {
    let filename = args().nth(1).unwrap();
    let jets = read_file(&filename);

    println!("Part 1: {}", solve(2022, &jets));
    println!("Part 2: {}", solve(1_000_000_000_000, &jets));
}
