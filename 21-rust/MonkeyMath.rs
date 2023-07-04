use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
enum Expression {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Eq(String, String),
    Num(i64)
}

fn read_file(filename: &String) -> HashMap<String, Expression> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| {
            let line = line.expect("Could not read line");
            let (name, expr) = line.split_once(':').expect("Could not split line");
            (name.to_string(), read_expr(expr))
        })
        .collect()
}

fn read_expr(line: &str) -> Expression {
    let tokens = line.trim().split(' ').collect::<Vec<_>>();
    match &tokens[..] {
        [a, "+", b] => Expression::Add(a.to_string(), b.to_string()),
        [a, "-", b] => Expression::Sub(a.to_string(), b.to_string()),
        [a, "*", b] => Expression::Mul(a.to_string(), b.to_string()),
        [a, "/", b] => Expression::Div(a.to_string(), b.to_string()),
        [num] => Expression::Num(num.parse().expect("Could not parse number")),
        _ => unreachable!("Unknown expression {}", line),
    }
}

fn eval(exprs: &HashMap<String, Expression>, name: &str) -> Option<i64> {
    match &exprs.get(name) {
        Some(Expression::Add(a, b)) => eval(exprs, a).and_then(|a| eval(exprs, b).map(|b| a + b)),
        Some(Expression::Sub(a, b)) => eval(exprs, a).and_then(|a| eval(exprs, b).map(|b| a - b)),
        Some(Expression::Mul(a, b)) => eval(exprs, a).and_then(|a| eval(exprs, b).map(|b| a * b)),
        Some(Expression::Div(a, b)) => eval(exprs, a).and_then(|a| eval(exprs, b).map(|b| a / b)),
        Some(Expression::Num(num))  => Some(*num),
        None => None,
        _ => unreachable!("Unknown expression {}", name),
    }
}

fn match_humn(exprs: &HashMap<String, Expression>, name: &str, value: i64) -> i64 {
    match (exprs.get(name), value) {
        (Some(Expression::Eq(a, b)), 1) => {
            match (eval(exprs, a), eval(exprs, b)) {
                (Some(a_val), _) => match_humn(exprs, b, a_val),
                (_, Some(b_val)) => match_humn(exprs, a, b_val),
                _ => unreachable!("Both sides of equality are incomplete")
            }
        },
        (Some(Expression::Add(a, b)), val) => {
            match (eval(exprs, a), eval(exprs, b)) {
                (Some(a_val), _) => match_humn(exprs, b, val - a_val),
                (_, Some(b_val)) => match_humn(exprs, a, val - b_val),
                _ => unreachable!("Both sides of equality are incomplete")
            }
        },
        (Some(Expression::Sub(a, b)), val) => {
            match (eval(exprs, a), eval(exprs, b)) {
                (Some(a_val), _) => match_humn(exprs, b, a_val - val),
                (_, Some(b_val)) => match_humn(exprs, a, b_val + val),
                _ => unreachable!("Both sides of equality are incomplete")
            }
        },
        (Some(Expression::Mul(a, b)), val) => {
            match (eval(exprs, a), eval(exprs, b)) {
                (Some(a_val), _) => match_humn(exprs, b, val / a_val),
                (_, Some(b_val)) => match_humn(exprs, a, val / b_val),
                _ => unreachable!("Both sides of equality are incomplete")
            }
        },
        (Some(Expression::Div(a, b)), val) => {
            match (eval(exprs, a), eval(exprs, b)) {
                (Some(a_val), _) => match_humn(exprs, b, a_val / val),
                (_, Some(b_val)) => match_humn(exprs, a, b_val * val),
                _ => unreachable!("Both sides of equality are incomplete")
            }
        },
        _ => value
    }
}

fn main() {
    let filename = args().nth(1).expect("No input file given");
    let mut expressions = read_file(&filename);

    println!("Part 1: {}", eval(&expressions, "root").unwrap());

    let root = match expressions.remove("root") {
        Some(Expression::Add(a, b)) => Expression::Eq(a, b),
        _ => unreachable!("Root expression is not an addition")
    };
    expressions.insert("root".to_string(), root);
    expressions.remove("humn");
    println!("Part 2: {}", match_humn(&expressions, "root", 1));
}
