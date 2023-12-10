use num;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

struct World {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>,
}

#[derive(Debug)]
struct ParseWorldError;

impl FromStr for Node {
    type Err = ParseWorldError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // AAA = (BBB, CCC)
        let (name, targets) = s.split_once(" = ").ok_or(ParseWorldError {})?;
        let targets = targets
            .to_string()
            .replace("(", "")
            .replace(")", "")
            .replace(",", "");
        let (left, right) = targets.split_once(" ").ok_or(ParseWorldError {})?;
        Ok(Node {
            name: name.into(),
            left: left.into(),
            right: right.into(),
        })
    }
}

impl FromStr for World {
    type Err = ParseWorldError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instructions, nodes) = s.split_once("\n\n").ok_or(ParseWorldError {})?;

        let instructions = instructions
            .trim()
            .chars()
            .filter_map(|i| match i {
                'L' => Some(Instruction::Left),
                'R' => Some(Instruction::Right),
                _ => None,
            })
            .collect();

        let nodes = nodes
            .split("\n")
            .filter_map(|line| line.parse::<Node>().ok())
            .map(|node| (node.name.clone(), node))
            .collect();

        Ok(World {
            instructions,
            nodes,
        })
    }
}

impl World {
    fn step(&self, current_node: &String, instruction: &Instruction) -> String {
        if instruction == &Instruction::Left {
            self.nodes[current_node].left.clone()
        } else {
            self.nodes[current_node].right.clone()
        }
    }

    fn path_len(&self, start: &str) -> usize {
        let mut current_node = start.to_string();
        let instructions = std::iter::zip(1usize.., self.instructions.iter().cycle());
        for (counter, instruction) in instructions {
            current_node = self.step(&current_node, instruction);
            if current_node.ends_with('Z') {
                return counter;
            }
        }
        return 0;
    }

    fn path_parallel(&self) -> usize {
        let start_nodes: Vec<_> = self
            .nodes
            .keys()
            .filter(|n| n.ends_with('A'))
            .cloned()
            .collect();

        let counters: Vec<_> = start_nodes
            .iter()
            .map(|start_node| self.path_len(&start_node.as_str()))
            .collect();

        counters.iter().fold(1, |acc, v| num::integer::lcm(acc, *v))
    }
}

fn main() {
    let file_content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let world = file_content.parse::<World>().unwrap();
    println!("{:?}", world.path_parallel());
}
