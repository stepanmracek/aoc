use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
enum Operand {
    Old,
    Value(i64),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    counter: i32,
    operation: Operation,
    operand: Operand,
    test_modulo: i64,
    on_true: usize,
    on_false: usize,
}

impl Monkey {
    fn process(&mut self, item: i64, modulo: i64) -> (usize, i64) {
        self.counter += 1;
        let operand = match self.operand {
            Operand::Old => item,
            Operand::Value(v) => v,
        };
        let mut worry_level = match self.operation {
            Operation::Multiply => item * operand,
            Operation::Add => item + operand,
        };
        worry_level = worry_level % modulo;
        if worry_level % self.test_modulo == 0 {
            (self.on_true, worry_level)
        } else {
            (self.on_false, worry_level)
        }
    }

    fn process_all(&mut self, modulo: i64) -> HashMap<usize, Vec<i64>> {
        let mut result: HashMap<usize, Vec<i64>> = HashMap::new();
        for item in self.items.clone() {
            let (to, processed_item) = self.process(item, modulo);
            if !result.contains_key(&to) {
                result.insert(to, vec![]);
            }
            result.get_mut(&to).unwrap().push(processed_item);
        }

        result
    }
}

struct MonkeyParseError;

impl FromStr for Monkey {
    type Err = MonkeyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.split('\n').collect();
        let items: Vec<_> = lines[1]["  Starting items: ".len()..]
            .split(", ")
            .filter_map(|i| i.parse::<i64>().ok())
            .collect();
        let op_items: Vec<_> = lines[2].split(' ').collect();
        let operand = if let Ok(v) = op_items[op_items.len() - 1].parse::<i64>() {
            Operand::Value(v)
        } else {
            Operand::Old
        };
        let operation = if op_items[op_items.len() - 2] == "*" {
            Operation::Multiply
        } else {
            Operation::Add
        };
        let test_modulo = lines[3]
            .split(' ')
            .last()
            .and_then(|s| s.parse::<i64>().ok())
            .ok_or(MonkeyParseError)?;
        let on_true = lines[4]
            .split(' ')
            .last()
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or(MonkeyParseError)?;
        let on_false = lines[5]
            .split(' ')
            .last()
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or(MonkeyParseError)?;

        Ok(Monkey {
            items,
            counter: 0,
            operation,
            operand,
            test_modulo,
            on_true,
            on_false,
        })
    }
}

fn main() {
    let arg = std::env::args().skip(1).next().unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();

    let mut monkeys: Vec<_> = file_content
        .split("\n\n")
        .filter_map(|s| s.parse::<Monkey>().ok())
        .collect();

    let modulo: i64 = monkeys.iter().map(|m| m.test_modulo).product();

    for round in 1..=10_000 {
        println!("{}", round);
        for m in 0..monkeys.len() {
            let process_result = monkeys[m].process_all(modulo);

            monkeys[m].items.clear();
            for (to_monkey, items) in process_result {
                monkeys[to_monkey].items.extend(items.iter());
            }
        }

        for m in 0..monkeys.len() {
            println!("{}: {:?} {}", m, monkeys[m].items, monkeys[m].counter);
        }
    }

    let mut counters: Vec<i32> = monkeys.iter().map(|m| m.counter).collect();
    counters.sort();
    let monkey_business: i64 = counters.iter().rev().take(2).map(|v| *v as i64).product();
    println!("{}", monkey_business);
}
