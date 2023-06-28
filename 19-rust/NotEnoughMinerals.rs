use std::collections::{HashMap, HashSet, VecDeque};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

struct Recipe {
    ingredients: HashMap<usize, isize>,
    product: usize,
}

struct Blueprint {
    index: isize,
    recipes: Vec<Recipe>,
}

type Robots = [isize; 4];
type Resources = [isize; 4];

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Node {
    robots: Robots,
    resources: Resources,
    time_left: isize,
}

fn read_file(filename: &String) -> Vec<Blueprint> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let blueprint = Regex::new(r"Blueprint (?<index>\d+)").unwrap();
    let recipe =
        Regex::new(r"Each (?P<product>\w+) robot costs(?P<ing>(?:(?: and)? \d+ \w+)+)").unwrap();
    let ingredient = Regex::new(r"(?P<amt>\d+) (?P<id>\w+)").unwrap();

    reader.lines()
        .map(|line| read_line(&line.expect("Could not read line"), &blueprint, &recipe, &ingredient))
        .collect()
}

fn material_to_index(material: &str) -> usize {
    match material {
        "ore" => 0,
        "clay" => 1,
        "obsidian" => 2,
        "geode" => 3,
        _ => unreachable!("Unknown material {}", material),
    }
}

fn read_line(line: &str, blueprint: &Regex, recipe: &Regex, ingredient: &Regex) -> Blueprint {
    let index_cap = blueprint.captures(line).unwrap();
    let index = index_cap["index"].parse().unwrap();

    let mut recipes = Vec::new();
    for r in recipe.captures_iter(line) {
        let product = material_to_index(&r["product"]);
        let ing = r["ing"].to_owned();

        let mut ingredients = HashMap::new();
        for i in ingredient.captures_iter(&ing) {
            let amount = i["amt"].parse().unwrap();
            let identifier = material_to_index(&i["id"]);
            ingredients.insert(identifier, amount);
        }
        recipes.push(Recipe { product, ingredients });
    }

    Blueprint { index, recipes }
}

fn can_build_robot(current: Node, recipe: &Recipe) -> Option<isize> {
    let needed: Vec<_> = current.resources.iter().enumerate()
        .map(|(i, have)| recipe.ingredients.get(&i).unwrap_or(&0) - have)
        .collect();

    let mut max_cost = 0;
    for i in 0..4 {
        if needed[i] > 0 {
            if current.robots[i] <= 0 { return None; }
            let cost = needed[i] / current.robots[i] + (needed[i] % current.robots[i] != 0) as isize;
            if cost > max_cost {
                max_cost = cost;
            }
        }
    }

    if current.time_left > max_cost {
        Some(max_cost)
    } else {
        None
    }
}

fn is_robot_redundant(max_needed: &[isize], current: Node, recipe: &Recipe) -> bool {
    recipe.product != 3 &&
        (current.robots[recipe.product] >= max_needed[recipe.product] ||
            current.resources[recipe.product] - current.robots[recipe.product] >= max_needed[recipe.product])
}

fn max_geodes_opened(blueprint: &Blueprint, time_left: isize) -> isize {
    let max_needed: Vec<_> = (0..3).map(|i| blueprint.recipes.iter()
        .map(|r| r.ingredients.get(&i).cloned().unwrap_or(0))
        .max().unwrap())
        .collect();

    let start = Node {
        robots: [1, 0, 0, 0],
        resources: [0, 0, 0, 0],
        time_left
    };

    let mut max_reward = 0;
    let mut visited: HashSet<Node> = HashSet::new();
    let mut queue: VecDeque<Node> = VecDeque::from([start]);

    while let Some(current) = queue.pop_front() {
        for recipe in &blueprint.recipes {
            if is_robot_redundant(&max_needed, current, recipe) {
                // prune this branch
                continue;
            } else if let Some(time_cost) = can_build_robot(current, recipe) {
                let new_time_left = current.time_left - time_cost - 1;

                let mut new_resources = current.resources;
                for i in 0..4 {
                    new_resources[i] += current.robots[i] * (time_cost + 1);
                    new_resources[i] -= recipe.ingredients.get(&i).unwrap_or(&0);
                }

                let mut new_robots = current.robots;
                new_robots[recipe.product] += 1;

                let new_node = Node {
                    robots: new_robots,
                    resources: new_resources,
                    time_left: new_time_left
                };

                let new_reward = new_resources[3] + new_robots[3] * new_time_left;
                if new_reward > max_reward {
                    max_reward = new_reward;
                }

                if visited.insert(new_node) {
                    queue.push_back(new_node);
                }
            }
        }
    }
    max_reward
}

fn solve_part_1(blueprints: &[Blueprint]) -> isize {
    blueprints
        .iter()
        .map(|b| b.index * max_geodes_opened(b, 24))
        .sum()
}

fn solve_part_2(blueprints: &[Blueprint]) -> isize {
    blueprints
        .iter()
        .take(3)
        .map(|b| max_geodes_opened(b, 32))
        .product()
}

fn main() {
    let filename = args().nth(1).expect("No input file given");
    let blueprints = read_file(&filename);

    println!("Part 1: {}", solve_part_1(&blueprints));
    println!("Part 2: {}", solve_part_2(&blueprints));
}
