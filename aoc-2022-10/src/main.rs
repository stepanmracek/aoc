use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

#[derive(Debug)]
struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instruction::Noop)
        } else {
            let v = s.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            Ok(Instruction::Add(v))
        }
    }
}

fn main() {
    let arg = std::env::args().skip(1).next().unwrap();
    let file_content = std::fs::read_to_string(&arg).unwrap();

    let instructions = file_content
        .split('\n')
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<Instruction>().ok());

    let result = instructions
        .flat_map(|i| match i {
            Instruction::Noop => vec![i],
            Instruction::Add(_) => vec![Instruction::Noop, i],
        })
        .scan(1, |x, instruction| {
            let before = x.clone();
            match instruction {
                Instruction::Noop => {}
                Instruction::Add(v) => *x = *x + v,
            }
            Some(before)
        })
        .enumerate()
        .map(|(i, x)| (i + 1, x));
    //.filter(|(i, _)| HashSet::from([20, 60, 100, 140, 180, 220]).contains(i))
    //.map(|(i, x)| (i as i32) * x)
    //.sum();

    for (i, x) in result {
        let pos = i as i32 % 40;
        print!(
            "{}",
            if pos == x || pos == x + 1 || pos == x + 2 {
                "█"
            } else {
                " "
            }
        );
        if i % 40 == 0 {
            println!()
        }
    }
}
