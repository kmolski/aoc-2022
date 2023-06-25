use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::File;
use std::ops::Add;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point(isize, isize, isize);

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

type PointCloud = HashSet<Point>;

fn read_file(filename: &String) -> PointCloud {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| read_line(&line.expect("Could not read line")))
        .collect()
}

fn read_line(line: &str) -> Point {
    let coords: Vec<_> = line.split(',').flat_map(|coord| coord.parse().ok()).collect();
    Point(coords[0], coords[1], coords[2])
}

fn is_pocket(p: Point, points: &PointCloud, cache: &mut HashMap<Point, bool>,
             min_coord: (isize, isize, isize), max_coord: (isize, isize, isize)) -> bool {
    if let Some(is_pocket) = cache.get(&p) {
        *is_pocket
    } else { // BFS
        let mut visited = HashSet::new();
        let mut queue = vec![p];

        let mut is_pocket = true;
        while !queue.is_empty() {
            let p = queue.pop().unwrap();
            if p.0 < min_coord.0 || p.0 > max_coord.0 || p.1 < min_coord.1
                || p.1 > max_coord.1 || p.2 < min_coord.2 || p.2 > max_coord.2 {
                is_pocket = false;
                break;
            }

            for offset in [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)] {
                let neighbor = p + Point(offset.0, offset.1, offset.2);
                if !visited.contains(&neighbor) && !points.contains(&neighbor) {
                    queue.push(neighbor);
                }
            }
            visited.insert(p);
        }
        cache.insert(p, is_pocket);
        is_pocket
    }
}

fn solve(points: &PointCloud, exclude_pockets: bool) -> isize {
    let min = (
        points.iter().map(|p| p.0).min().unwrap(),
        points.iter().map(|p| p.1).min().unwrap(),
        points.iter().map(|p| p.2).min().unwrap()
    );
    let max = (
        points.iter().map(|p| p.0).max().unwrap(),
        points.iter().map(|p| p.1).max().unwrap(),
        points.iter().map(|p| p.2).max().unwrap()
    );
    let mut cache = HashMap::new();

    let mut visible_sides = 0;
    for p in points {
        for offset in [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)] {
            let neighbor = *p + Point(offset.0, offset.1, offset.2);
            if !points.contains(&neighbor) {
                if exclude_pockets {
                    if !is_pocket(neighbor, points, &mut cache, min, max) {
                        visible_sides += 1;
                    }
                } else {
                    visible_sides += 1;
                }
            }
        }
    }
    visible_sides
}

fn main() {
    let filename = args().nth(1).unwrap();
    let cubes = read_file(&filename);

    println!("Part 1: {}", solve(&cubes, false));
    println!("Part 2: {}", solve(&cubes, true));
}
