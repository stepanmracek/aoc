use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

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

fn springs_to_string(springs: &Vec<Spring>) -> String {
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

fn append(candidate_springs: &mut Vec<Spring>, oper_count: usize, dmg_count: usize) {
    for _ in 0..oper_count {
        candidate_springs.push(Spring::Operational);
    }
    for _ in 0..dmg_count {
        candidate_springs.push(Spring::Damaged);
    }
}

fn append_many(
    candidate_springs: &Vec<Spring>,
    oper_range: RangeInclusive<usize>,
    dmg_count: usize,
) -> Vec<Vec<Spring>> {
    let mut result = vec![];
    for oper_count in oper_range {
        let mut c = candidate_springs.clone();
        append(&mut c, oper_count, dmg_count);
        result.push(c);
    }
    result
}

impl Row {
    fn generate_arrangements(&self) -> Vec<Vec<Spring>> {
        let mut result = vec![];

        let row_len = self.springs.len();
        let damaged_count: usize = self.checksum.iter().sum();
        let operational_count = row_len - damaged_count;
        let damaged_groups_count = self.checksum.len();
        //println!("operational_count {}", operational_count);

        let max_initial_operational = operational_count - (damaged_groups_count - 1);
        //println!("max_initial_operational {}", max_initial_operational);
        for initial_operational in 0..=max_initial_operational {
            let mut candidate: Vec<Spring> = vec![];
            append(&mut candidate, initial_operational, self.checksum[0]);
            //let remaining_operational = operational_count - initial_operational;
            let check = self.check(&candidate);
            /*println!(
                "initial_operational {}, remaining_operational {}, candidate {} => {}",
                initial_operational,
                remaining_operational,
                springs_to_string(&candidate),
                check
            );*/
            if !check {
                continue;
            }

            let remaining_damaged_groups = &self.checksum[1..];
            let arrangements = self.gen_recursive_arrangements(
                &candidate,
                operational_count,
                remaining_damaged_groups,
            );

            result.extend(
                arrangements
                    .iter()
                    .filter(|a| a.len() == self.springs.len())
                    .cloned(),
            );
        }

        result
    }

    fn gen_recursive_arrangements(
        &self,
        candidate: &Vec<Spring>,
        max_operational: usize,
        remaining_damaged_groups: &[usize],
    ) -> Vec<Vec<Spring>> {
        let mut result = vec![];

        let remaining_operational = max_operational
            - candidate
                .iter()
                .filter(|&v| v == &Spring::Operational)
                .count();
        if remaining_damaged_groups.len() > 0 {
            let max_oper_count = remaining_operational - (remaining_damaged_groups.len() - 1);
            let oper_range = 1..=max_oper_count;
            let candidates = append_many(candidate, oper_range, remaining_damaged_groups[0]);
            /*println!(
                "gen_recursive_arrangements {}",
                springs_to_string(candidate)
            );
            println!("gen_recursive_arrangements candidates: {}; remaining_operational: {}; remaining_damaged_groups.len(): {}", candidates.len(), remaining_operational, remaining_damaged_groups.len());*/
            for c in candidates.iter() {
                let check = self.check(c);
                //println!("  {} => {}", springs_to_string(c), check);
                if check {
                    // && remaining_damaged_groups[1..].len() > 0
                    result.extend(
                        self.gen_recursive_arrangements(
                            c,
                            max_operational,
                            &remaining_damaged_groups[1..],
                        )
                        .iter()
                        .cloned(),
                    )
                }
            }
        } else {
            //println!("No more damaged groups left, appending just remaining operational springs");
            let mut c = candidate.clone();
            for _ in 0..remaining_operational {
                c.push(Spring::Operational);
            }
            if self.check(&c) {
                result.push(c);
            }
        }

        result
    }

    #[allow(dead_code)]
    fn check(&self, candidate: &Vec<Spring>) -> bool {
        for (template, candidate) in std::iter::zip(self.springs.iter(), candidate) {
            if template != &Spring::Unknown && template != candidate {
                return false;
            }
        }
        return true;
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
    let file_content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let rows: Vec<Row> = file_content
        .split('\n')
        .filter_map(|s| s.parse::<Row>().ok())
        .collect();

    /*let result: usize = rows.iter().map(|r| r.generate_arrangements().len()).sum();
    println!("{}", result);*/

    for row in rows {
        println!("{}", row);
        println!("{}", row.generate_arrangements().len());
        let row = row.extend();
        println!("{}", row);
        println!("{}", row.generate_arrangements().len());
        println!();
    }
}
