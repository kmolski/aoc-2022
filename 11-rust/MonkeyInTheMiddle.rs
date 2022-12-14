use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Deref, Div, Mul, Rem};

trait ModularArithmetic<T>:
    Add<Output = T>
    + Mul<Output = T>
    + Div<u64, Output = T>
    + Rem<u64, Output = u64>
    + From<u64> + Clone + Sized {}

#[derive(Clone)]
struct Monkey<T: ModularArithmetic<T>> {
    id: usize,
    items: VecDeque<T>,
    op: Operation,
    test: Test,
    inspection_count: u64
}

#[derive(Copy, Clone)]
struct Operation {
    operator: Operator,
    left: Operand,
    right: Operand,
}

#[derive(Copy, Clone)]
enum Operator { Add, Multiply } // assume addition and multiplication only

#[derive(Copy, Clone)]
enum Operand { OldValue, Literal(u64) }

#[derive(Copy, Clone)]
struct Test {
    divisible_by: u64, // assume "divisible by" test only
    target_if_true: usize,
    target_if_false: usize,
}

#[derive(Clone)]
struct ModulusSet { modulus_to_rem: HashMap<u64, u64> }

impl ModularArithmetic<u64> for u64 {}
impl ModularArithmetic<ModulusSet> for ModulusSet {}

impl ModulusSet {
    fn get(&mut self, modulus: u64) -> u64 {
        if let None = self.modulus_to_rem.get(&modulus) {
            let rem = self.modulus_to_rem[&u64::MAX] % modulus;
            self.modulus_to_rem.insert(modulus, rem);
        }
        self.modulus_to_rem[&modulus]
    }

    fn set_moduli(mut self, moduli: &impl Deref<Target=[u64]>) -> ModulusSet {
        moduli.iter().for_each(|m| { self.get(*m); });
        self
    }
}

impl Add for ModulusSet {
    type Output = ModulusSet;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let mut copy = self.clone();
        for (modulus, rem) in &mut copy.modulus_to_rem {
            if *modulus != u64::MAX {
                *rem = (self.get(*modulus) + rhs.get(*modulus)) % modulus;
            }
        }
        copy
    }
}

impl Mul for ModulusSet {
    type Output = ModulusSet;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        let mut copy = self.clone();
        for (modulus, rem) in &mut copy.modulus_to_rem {
            if *modulus != u64::MAX {
                *rem = (self.get(*modulus) * rhs.get(*modulus)) % modulus;
            }
        }
        copy
    }
}

impl Div<u64> for ModulusSet {
    type Output = ModulusSet;

    fn div(self, rhs: u64) -> Self::Output {
        assert_eq!(rhs, 1);
        self // division by 1 = no-op
    }
}

impl Rem<u64> for ModulusSet {
    type Output = u64;

    fn rem(mut self, rhs: u64) -> Self::Output { self.get(rhs) }
}

impl From<u64> for ModulusSet {
    fn from(val: u64) -> Self { ModulusSet { modulus_to_rem: HashMap::from([(u64::MAX, val)]) } }
}

fn read_monkey(section: &str) -> Monkey<u64> {
    let lines: Vec<&str> = section.lines().collect();
    let id = lines[0].trim_matches(|c: char| !c.is_numeric()).parse().unwrap();
    let items = lines[1].split_once(": ").unwrap().1.split(", ").flat_map(str::parse).collect();

    let tokens: Vec<&str> = lines[2].split_once(" = ").unwrap().1.split(" ").collect();
    let operator = match tokens[1] {
        "+" => Operator::Add,
        "*" => Operator::Multiply,
        _ => unreachable!()
    };
    let left = tokens[0].parse().map(Operand::Literal).unwrap_or(Operand::OldValue);
    let right = tokens[2].parse().map(Operand::Literal).unwrap_or(Operand::OldValue);
    let op = Operation { operator, left, right };

    let divisible_by = lines[3].trim_matches(|c: char| !c.is_numeric()).parse().unwrap();
    let target_if_true = lines[4].trim_matches(|c: char| !c.is_numeric()).parse().unwrap();
    let target_if_false = lines[5].trim_matches(|c: char| !c.is_numeric()).parse().unwrap();
    let test = Test { divisible_by, target_if_true, target_if_false };

    Monkey { id, items, op, test, inspection_count: 0 }
}

fn read_file(filename: &String) -> Vec<Monkey<u64>> {
    let mut file = File::open(filename).expect("Could not open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read file contents");

    content.split("\n\n").map(read_monkey).collect()
}

fn get_monkeys_with_modulus(monkeys: &Vec<Monkey<u64>>) -> Vec<Monkey<ModulusSet>> {
    let moduli: Vec<u64> = monkeys.iter().map(|m| m.test.divisible_by).collect();
    monkeys.iter().map(|m|
        Monkey {
            id: m.id,
            items: m.items.iter().map(|i| ModulusSet::from(*i).set_moduli(&moduli)).collect(),
            op: m.op,
            test: m.test,
            inspection_count: 0
        }).collect()
}

fn solve_part<T: ModularArithmetic<T>>(mut monkeys: Vec<Monkey<T>>, rounds: u32, divisor: u64) -> u64 {
    for _ in  0..rounds {
        let mut next_monkeys: Vec<Monkey<T>> = Vec::new();
        while !monkeys.is_empty() {
            let mut m = monkeys.remove(0);
            while !m.items.is_empty() {
                m.inspection_count += 1;
                let old = m.items.pop_front().unwrap();

                let left = if let Operand::Literal(lit) = m.op.left { lit.into() } else { old.clone() };
                let right = if let Operand::Literal(lit) = m.op.right { lit.into() } else { old.clone() };
                let operator: fn(T, T) -> T = match m.op.operator {
                    Operator::Add => T::add,
                    Operator::Multiply => T::mul
                };
                let new = operator(left, right) / divisor;

                let target = if new.clone() % m.test.divisible_by == 0 { m.test.target_if_true } else { m.test.target_if_false };
                monkeys.iter_mut().find(|m| m.id == target)
                    .or(next_monkeys.iter_mut().find(|m| m.id == target))
                    .unwrap_or(&mut m).items.push_back(new.clone());
            }
            next_monkeys.push(m);
        }
        monkeys = next_monkeys;
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkeys.iter().take(2).map(|m| m.inspection_count).product()
}

fn solve(filename: &String) {
    let monkeys = read_file(filename);

    println!("Part 1: {}", solve_part(monkeys.clone(), 20, 3));
    println!("Part 2: {}", solve_part(get_monkeys_with_modulus(&monkeys), 10000, 1));
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    solve(&filename);
}
