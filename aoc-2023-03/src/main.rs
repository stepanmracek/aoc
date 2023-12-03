use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Symbol {
    Gear,
    Other,
}

#[derive(Debug, Clone)]
struct Number {
    value: i32,
    row: i32,
    col: i32,
    width: i32,
}

impl Number {
    fn neighbourhood(&self) -> Vec<(i32, i32)> {
        let mut result = vec![];
        for col in self.col - 1..=self.col + self.width {
            result.push((self.row - 1, col));
            result.push((self.row + 1, col));
        }
        result.push((self.row, self.col - 1));
        result.push((self.row, self.col + self.width));

        result
    }

    fn get_gears(&self, symbols: &HashMap<(i32, i32), Symbol>) -> Vec<(i32, i32)> {
        let mut result = vec![];
        for coord in self.neighbourhood() {
            if let Some(Symbol::Gear) = symbols.get(&coord) {
                result.push(coord);
            }
        }
        result
    }
}

struct World {
    symbols: HashMap<(i32, i32), Symbol>,
    numbers: Vec<Number>,
}

#[derive(Debug)]
struct WorldParseError;

impl FromStr for World {
    type Err = WorldParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split('\n').filter(|line| !line.is_empty());
        let mut symbols = HashMap::new();
        let mut numbers = Vec::new();
        for (row, line) in lines.enumerate() {
            let mut line_content = line.chars().enumerate();
            while let Some((col, c)) = line_content.next() {
                if c == '.' {
                    // pass
                } else if c.is_ascii_digit() {
                    let digits: String = line[col..]
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect();
                    numbers.push(Number {
                        row: row as i32,
                        col: col as i32,
                        value: digits.parse().map_err(|_| WorldParseError)?,
                        width: digits.len() as i32,
                    });
                    // skip remaining digits in line_content iterator
                    if digits.len() >= 2 {
                        let _ = line_content.nth(digits.len() - 2);
                    }
                } else {
                    let symbol = match c {
                        '*' => Symbol::Gear,
                        _ => Symbol::Other,
                    };
                    symbols.insert((row as i32, col as i32), symbol);
                }
            }
        }
        Ok(World { symbols, numbers })
    }
}

fn any_symbol(coords: &Vec<(i32, i32)>, symbols: &HashMap<(i32, i32), Symbol>) -> bool {
    for coord in coords {
        if symbols.get(coord).is_some() {
            return true;
        }
    }
    false
}

fn main() {
    let world: World = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .parse()
        .unwrap();

    let sum: i32 = world
        .numbers
        .iter()
        .filter(|number| any_symbol(&number.neighbourhood(), &world.symbols))
        .map(|number| number.value)
        .sum();
    println!("{}", sum);

    let mut gears: HashMap<(i32, i32), Vec<Number>> = HashMap::new();
    for number in world.numbers {
        for gear in number.get_gears(&world.symbols) {
            if let Some(v) = gears.get_mut(&gear) {
                v.push(number.clone())
            } else {
                gears.insert(gear, vec![number.clone()]);
            }
        }
    }
    let sum: i32 = gears
        .values()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0].value * numbers[1].value)
        .sum();
    println!("{}", sum);
}
