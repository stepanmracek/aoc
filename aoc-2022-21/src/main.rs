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
    first: String,
    second: String,
}

#[derive(Debug)]
enum Monkey {
    Value(i64),
    Job(Job),
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Monkey {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<_> = s.split(" ").collect();
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
                first: items[0].into(),
                second: items[2].into(),
            }))
        }
    }
}

fn eval(monkey: &Monkey, monkeys: &HashMap<String, Monkey>) -> i64 {
    match monkey {
        Monkey::Value(value) => *value,
        Monkey::Job(job) => {
            let first = eval(monkeys.get(&job.first).unwrap(), monkeys);
            let second = eval(monkeys.get(&job.second).unwrap(), monkeys);
            match job.op {
                Op::Add => first + second,
                Op::Sub => first - second,
                Op::Mul => first * second,
                Op::Div => first / second,
            }
        }
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let monkeys: HashMap<String, Monkey> = std::fs::read_to_string(arg)
        .unwrap()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let items: Vec<_> = line.split(": ").collect();
            (items[0].into(), items[1].parse().unwrap())
        })
        .collect();

    println!("{:?}", eval(monkeys.get("root").unwrap(), &monkeys));
}
