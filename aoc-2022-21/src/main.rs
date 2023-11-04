use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct Job {
    op: Op,
    left: String,
    right: String,
}

#[derive(Debug)]
enum Monkey {
    Value(f64),
    Job(Job),
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Monkey {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<_> = s.split(' ').collect();
        if items.len() == 1 {
            Ok(Monkey::Value(items[0].parse().map_err(|_| ParseError)?))
        } else {
            let op = match items[1] {
                "*" => Op::Mul,
                "/" => Op::Div,
                "+" => Op::Add,
                "-" => Op::Sub,
                &_ => return Err(ParseError),
            };
            Ok(Monkey::Job(Job {
                op,
                left: items[0].into(),
                right: items[2].into(),
            }))
        }
    }
}

fn eval(monkey: &Monkey, monkeys: &HashMap<String, Monkey>) -> f64 {
    match monkey {
        Monkey::Value(value) => *value,
        Monkey::Job(job) => {
            let left = eval(monkeys.get(&job.left).unwrap(), monkeys);
            let right = eval(monkeys.get(&job.right).unwrap(), monkeys);
            match job.op {
                Op::Add => left + right,
                Op::Sub => left - right,
                Op::Mul => left * right,
                Op::Div => left / right,
            }
        }
    }
}

fn get_root_children(monkeys: &HashMap<String, Monkey>) -> Option<(String, String)> {
    if let Monkey::Job(root) = &monkeys["root"] {
        return Some((root.left.clone(), root.right.clone()));
    }
    None
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let mut monkeys: HashMap<String, Monkey> = std::fs::read_to_string(arg)
        .unwrap()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let items: Vec<_> = line.split(": ").collect();
            (items[0].into(), items[1].parse().unwrap())
        })
        .collect();

    println!("part 1 result: {}", eval(&monkeys["root"], &monkeys));

    let (root_left, root_right) = get_root_children(&monkeys).unwrap();

    let incr_factor = 500_000_000.0;
    if let Monkey::Value(init_human) = monkeys["humn"] {
        println!("initial human value: {}", init_human);
        let init_left = eval(&monkeys[&root_left], &monkeys);
        let init_right = eval(&monkeys[&root_right], &monkeys);
        println!("left: {}", init_left);
        println!("right: {}", init_right);

        println!("increasing human value to: {}", init_human + incr_factor);
        monkeys.insert("humn".into(), Monkey::Value(init_human + incr_factor));

        let after_left = eval(&monkeys[&root_left], &monkeys);
        let after_right = eval(&monkeys[&root_right], &monkeys);
        println!("left: {}", after_left);
        println!("right: {}", after_right);

        let left_delta = after_left - init_left;
        let right_delta = after_right - init_right;

        if left_delta != 0.0 && right_delta == 0.0 {
            let target_delta = init_right - init_left;
            println!("delta achieved by increase of one: {}", left_delta);
            println!("target delta: {}", target_delta);
            let increase = incr_factor * target_delta / left_delta;
            println!("required increase of initial human value: {}", increase);
            let result = (init_human + increase).round();
            println!("part 2 result: {}", result);

            monkeys.insert("humn".into(), Monkey::Value(result));
            let result_left = eval(&monkeys[&root_left], &monkeys);
            let result_right = eval(&monkeys[&root_right], &monkeys);
            println!("left: {}", result_left);
            println!("right: {}", result_right);
        }
    }
}
