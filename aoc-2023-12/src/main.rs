use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    checksum: Vec<usize>,
}

#[derive(Debug)]
struct RowParseError {}

impl FromStr for Row {
    type Err = RowParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, checksum) = s.split_once(' ').ok_or(RowParseError {})?;
        let springs: Vec<Spring> = springs
            .chars()
            .filter_map(|c| match c {
                '.' => Some(Spring::Operational),
                '#' => Some(Spring::Damaged),
                '?' => Some(Spring::Unknown),
                _ => None,
            })
            .collect();
        let checksum: Vec<usize> = checksum.split(',').filter_map(|s| s.parse().ok()).collect();
        Ok(Row { springs, checksum })
    }
}

fn springs_to_string(springs: &[Spring]) -> String {
    springs
        .iter()
        .map(|v| match v {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        })
        .collect()
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:?}",
            springs_to_string(&self.springs),
            self.checksum
        )
    }
}

impl Row {
    fn count_arrangements(&self) -> usize {
        let mut cache: HashMap<(usize, usize, usize), usize> = HashMap::new();
        self.count_arrangements_recursive(&mut cache, 0, 0, 0)
    }

    fn count_arrangements_recursive(
        &self,
        cache: &mut HashMap<(usize, usize, usize), usize>,
        pos: usize,
        block_index: usize,
        block_pos: usize,
    ) -> usize {
        let key = (pos, block_index, block_pos);
        if let Some(cached) = cache.get(&key) {
            return *cached;
        }
        if pos == self.springs.len() {
            let valid_over_last_block = block_index == self.checksum.len() && block_pos == 0;
            let valid =
                block_index == self.checksum.len() - 1 && self.checksum[block_index] == block_pos;

            if valid || valid_over_last_block {
                return 1;
            } else {
                return 0;
            }
        }

        let mut result = 0;
        for substitute in [Spring::Operational, Spring::Damaged] {
            if self.springs[pos] == substitute || self.springs[pos] == Spring::Unknown {
                if substitute == Spring::Operational && block_pos == 0 {
                    // just move to the next spring
                    result += self.count_arrangements_recursive(cache, pos + 1, block_index, 0);
                } else if substitute == Spring::Operational
                    && block_pos > 0
                    && block_index < self.checksum.len()
                    && self.checksum[block_index] == block_pos
                {
                    // block of damaged springs ended -> increment block_index
                    result += self.count_arrangements_recursive(cache, pos + 1, block_index + 1, 0);
                } else if substitute == Spring::Damaged {
                    // next damaged spring in a sequence of damaged springs -> increment block_pos
                    result += self.count_arrangements_recursive(
                        cache,
                        pos + 1,
                        block_index,
                        block_pos + 1,
                    );
                }
            }
        }
        cache.insert(key, result);
        result
    }

    fn extend(&self) -> Self {
        let mut springs = self.springs.clone();
        for _ in 1..5 {
            springs.push(Spring::Unknown);
            springs.extend(self.springs.iter().cloned());
        }

        let mut checksum = vec![];
        for _ in 0..5 {
            checksum.extend(self.checksum.iter().cloned());
        }

        Row { springs, checksum }
    }
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Missing input file argument");
    let file_content = std::fs::read_to_string(path).expect("Can't read input file");
    let rows: Vec<Row> = file_content
        .lines()
        .filter_map(|s| s.parse::<Row>().ok())
        .collect();

    let iterator = rows.iter().map(|row| row.extend().count_arrangements());
    let result: usize = iterator.sum();
    println!("{}", result);
}
