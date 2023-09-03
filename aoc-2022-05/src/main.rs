use std::str::FromStr;

fn print_stacks(stacks: &mut [Vec<char>; 9]) {
    for stack in stacks.iter_mut() {
        println!("{:?}", stack)
    }
}

#[derive(Debug)]
struct Instruction {
    repeat: usize,
    from: usize,
    to: usize,
}

struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<_> = s
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        if vals.len() < 6 {
            return Err(InstructionParseError);
        }

        let repeat = vals[1]
            .parse::<usize>()
            .map_err(|_| InstructionParseError)?;
        let from = vals[3]
            .parse::<usize>()
            .map_err(|_| InstructionParseError)?
            - 1;
        let to = vals[5]
            .parse::<usize>()
            .map_err(|_| InstructionParseError)?
            - 1;
        Ok(Instruction { repeat, from, to })
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_content = std::fs::read_to_string(&args[1]).unwrap();

    let mut stacks: [Vec<char>; 9] = Default::default();

    let header = file_content
        .split('\n')
        .take_while(|&line| line != "")
        .map(|line| {
            line.replace("    ", " ")
                .replace("[", "")
                .replace("] ", "")
                .replace("]", "")
        })
        .take_while(|line| line.chars().filter(|c| c.is_digit(10)).count() == 0);

    let mut skip = 2;
    for row in header {
        skip += 1;
        let crates = row.chars().enumerate().filter(|(_, c)| !c.is_whitespace());
        for (i, c) in crates {
            stacks[i].push(c);
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    print_stacks(&mut stacks);

    let instructions = file_content
        .split('\n')
        .skip(skip)
        .filter_map(|i| i.parse::<Instruction>().ok());

    for i in instructions {
        println!("{:?}", i);
        let mut batch: Vec<_> = vec![];
        for _ in 0..i.repeat {
            let c = stacks[i.from].pop().unwrap();
            batch.push(c)
        }
        stacks[i.to].extend(batch.iter().rev());
        print_stacks(&mut stacks);
    }

    let result: String = stacks.iter().filter_map(|s| s.last()).collect();
    println!("{}", result);
}
