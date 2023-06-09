use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

type ValveEntry = (String, isize, Vec<String>);
type Valves = Vec<(String, isize)>;
type DistMap = Vec<Vec<isize>>;

fn read_file(filename: &String) -> Vec<ValveEntry> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let regex =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]+)")
            .unwrap();

    reader
        .lines()
        .map(|line| read_line(&line.expect("Could not read line"), &regex))
        .collect()
}

fn read_line(line: &str, regex: &Regex) -> (String, isize, Vec<String>) {
    let captures = regex
        .captures(line)
        .unwrap_or_else(|| panic!("Line does not match: {line}"));
    let self_tag = captures[1].to_owned();
    let flow = captures[2].parse().unwrap();
    let path_tags = captures[3].split(", ").map(str::to_owned).collect();

    (self_tag, flow, path_tags)
}

fn build_dist_map(name_map: &Valves, entries: Vec<ValveEntry>) -> (DistMap, usize) {
    let tag_to_index: HashMap<_, _> = name_map
        .iter()
        .map(|(tag, _)| tag)
        .enumerate()
        .map(|(i, tag)| (tag, i))
        .collect();

    let n = name_map.len();
    let mut dist_map = vec![vec![isize::MAX; n]; n];
    for (tag, _, paths) in entries {
        for other_tag in paths {
            let (this_index, other_index) = (tag_to_index[&tag], tag_to_index[&other_tag]);
            dist_map[this_index][other_index] = 1;
        }
    }

    for i in 0..n {
        // find shortest paths using the Floyd-Warshall algorithm
        for j in 0..n {
            for k in 0..n {
                dist_map[j][k] = dist_map[j][i]
                    .saturating_add(dist_map[i][k])
                    .min(dist_map[j][k]);
            }
        }
    }

    (dist_map, tag_to_index[&"AA".to_owned()])
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Node {
    index: usize,
    time_left: isize,
    priority: isize,
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

fn solve_part_1(name_map: &Valves, dist_map: &DistMap, aa_index: usize) -> isize {
    let start = Node {
        index: aa_index,
        time_left: 30,
        priority: 0,
    };

    let mut frontier = BinaryHeap::new();
    let mut visited: HashMap<Node, Vec<usize>> = HashMap::new();
    let mut reward_so_far: HashMap<Node, isize> = HashMap::new();

    frontier.push(start);
    visited.insert(start, Vec::from([aa_index]));
    reward_so_far.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        for (next, &time_cost) in dist_map[current.index]
            .iter()
            .enumerate()
            .filter(|(i, _)| name_map[*i].1 > 0)
        // positive-flow valves only
        {
            let flow = name_map[next].1;
            let time_left = current.time_left - (time_cost + 1);
            let relief = flow * time_left;
            let new_reward = reward_so_far[&current] + relief;
            let new_node = Node {
                index: next,
                time_left,
                priority: relief,
            };

            if time_left > 0
                && !visited[&current].contains(&next)
                && (!reward_so_far.contains_key(&new_node) || new_reward > reward_so_far[&new_node])
            {
                reward_so_far.insert(new_node, new_reward);
                frontier.push(new_node);

                let mut new_visited = visited[&current].clone();
                new_visited.push(new_node.index);
                visited.insert(new_node, new_visited);
            }
        }
    }

    *reward_so_far.values().max().unwrap()
}

fn solve_part_2(name_map: &Valves, dist_map: &DistMap, aa_index: usize) -> isize {
    let start = Node {
        index: aa_index,
        time_left: 26,
        priority: 0,
    };

    let mut frontier = BinaryHeap::new();
    let mut visited: HashMap<Node, Vec<usize>> = HashMap::new();
    let mut reward_so_far: HashMap<Node, isize> = HashMap::new();

    frontier.push(start);
    visited.insert(start, Vec::from([aa_index]));
    reward_so_far.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        for (next, &time_cost) in dist_map[current.index]
            .iter()
            .enumerate()
            .filter(|(i, _)| name_map[*i].1 > 0)
        // positive-flow valves only
        {
            let flow = name_map[next].1;
            let time_left = current.time_left - (time_cost + 1);
            let relief = flow * time_left;
            let new_reward = reward_so_far[&current] + relief;
            let new_node = Node {
                index: next,
                time_left,
                priority: relief,
            };

            if time_left > 0
                && !visited[&current].contains(&next)
                && (!reward_so_far.contains_key(&new_node) || new_reward > reward_so_far[&new_node])
            {
                reward_so_far.insert(new_node, new_reward);
                frontier.push(new_node);

                let mut new_visited = visited[&current].clone();
                new_visited.push(new_node.index);
                visited.insert(new_node, new_visited);
            }
        }
    }

    *reward_so_far.values().max().unwrap()
}

fn main() {
    let filename = args().nth(1).expect("No input file given");
    let entries = read_file(&filename);
    let name_map = entries
        .iter()
        .map(|entry| (entry.0.clone(), entry.1))
        .collect();
    let (dist_map, aa_index) = build_dist_map(&name_map, entries);

    println!("Part 1: {}", solve_part_1(&name_map, &dist_map, aa_index));
    println!("Part 2: {}", solve_part_2(&name_map, &dist_map, aa_index));
}
