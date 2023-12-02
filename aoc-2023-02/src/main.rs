use std::str::FromStr;

#[derive(Debug)]
struct GameParseError;

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, sets) = s.split_once(": ").ok_or(GameParseError)?;
        let game_id = game
            .split_once(' ')
            .ok_or(GameParseError)?
            .1
            .parse()
            .map_err(|_| GameParseError)?;

        let sets: Result<Vec<Set>, GameParseError> = sets
            .split(';')
            .map(|s| s.trim())
            .map(|s| s.parse())
            .collect();

        Ok(Game {
            id: game_id,
            sets: sets?,
        })
    }
}

impl Game {
    fn is_plausible(&self, bag: &Bag) -> bool {
        self.sets.iter().all(|set| set.is_plausible(bag))
    }

    fn get_min(&self) -> Bag {
        let mut result = Bag {
            red: 0,
            green: 0,
            blue: 0,
        };
        for set in self.sets.iter() {
            if let Some(red) = set.red {
                if red > result.red {
                    result.red = red;
                }
            }
            if let Some(green) = set.green {
                if green > result.green {
                    result.green = green;
                }
            }
            if let Some(blue) = set.blue {
                if blue > result.blue {
                    result.blue = blue;
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct Set {
    red: Option<i32>,
    green: Option<i32>,
    blue: Option<i32>,
}

#[derive(Debug)]
struct Bag {
    red: i32,
    green: i32,
    blue: i32,
}

impl Bag {
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

impl Set {
    fn is_plausible(&self, bag: &Bag) -> bool {
        if let Some(red) = self.red {
            if red > bag.red {
                return false;
            }
        }
        if let Some(green) = self.green {
            if green > bag.green {
                return false;
            }
        }
        if let Some(blue) = self.blue {
            if blue > bag.blue {
                return false;
            }
        }

        true
    }
}

impl FromStr for Set {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = None;
        let mut green = None;
        let mut blue = None;
        s.split(", ")
            .filter_map(|s| s.trim().split_once(' '))
            .for_each(|(count, color)| {
                let count = count.parse().ok();
                match color {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => {}
                }
            });

        Ok(Set { red, green, blue })
    }
}

fn main() {
    let games: Result<Vec<Game>, GameParseError> =
        std::fs::read_to_string(std::env::args().nth(1).unwrap())
            .unwrap()
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse())
            .collect();
    let games = games.unwrap();

    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result: i32 = games
        .iter()
        .filter(|game| game.is_plausible(&bag))
        .map(|game| game.id)
        .sum();
    println!("{}", result);

    let result: i32 = games.iter().map(|game| game.get_min().power()).sum();
    println!("{}", result);
}
