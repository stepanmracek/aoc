use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Debug)]
struct Round {
    oponent: Symbol,
    outcome: Outcome,
}

#[derive(Debug)]
struct ParseRoundError;

fn letter_to_symbol(letter: &str) -> Option<Symbol> {
    match letter {
        "A" => Some(Symbol::Rock),
        "B" => Some(Symbol::Paper),
        "C" => Some(Symbol::Scissors),
        _ => None,
    }
}

fn letter_to_outcome(letter: &str) -> Option<Outcome> {
    match letter {
        "X" => Some(Outcome::Lose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None,
    }
}

impl FromStr for Round {
    type Err = ParseRoundError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (oponent, outcome) = s.split_once(' ').ok_or(ParseRoundError)?;
        let oponent = letter_to_symbol(oponent).ok_or(ParseRoundError)?;
        let outcome = letter_to_outcome(outcome).ok_or(ParseRoundError)?;
        Ok(Round { oponent, outcome })
    }
}

impl Round {
    fn get_my_symbol(&self) -> Symbol {
        if self.outcome == Outcome::Draw {
            self.oponent.clone()
        } else if self.outcome == Outcome::Lose {
            if self.oponent == Symbol::Rock {
                Symbol::Scissors
            } else if self.oponent == Symbol::Paper {
                Symbol::Rock
            } else {
                Symbol::Paper
            }
        } else {
            if self.oponent == Symbol::Rock {
                Symbol::Paper
            } else if self.oponent == Symbol::Paper {
                Symbol::Scissors
            } else {
                Symbol::Rock
            }
        }
    }
    fn score(&self) -> i32 {
        let my = self.get_my_symbol();
        let mut s = 0;
        match my {
            Symbol::Rock => s += 1,
            Symbol::Paper => s += 2,
            Symbol::Scissors => s += 3,
        }
        if (my == Symbol::Rock && self.oponent == Symbol::Scissors)
            || (my == Symbol::Scissors && self.oponent == Symbol::Paper)
            || (my == Symbol::Paper && self.oponent == Symbol::Rock)
        {
            s += 6;
        } else if my == self.oponent {
            s += 3;
        }
        s
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_content = std::fs::read_to_string(&args[1]).unwrap();

    let rounds = file_content
        .split('\n')
        .filter_map(|r| r.parse::<Round>().ok())
        .map(|r| r.score());
    let score: i32 = rounds.sum();
    print!("{:?}", score)
}
