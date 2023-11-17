use std::{collections::HashMap, collections::HashSet, mem::swap, str::FromStr};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Blizzard {
    dx: i32,
    dy: i32,
}

type Blizzards = HashMap<Coord, Vec<Blizzard>>;

struct World {
    blizzards: Blizzards,
    players: HashSet<Coord>,
    start: Coord,
    finish: Coord,
    size: Coord,
}

impl Coord {
    fn neighbours(&self) -> [Coord; 5] {
        [
            self.clone(),
            Coord {
                x: self.x + 1,
                y: self.y,
            },
            Coord {
                x: self.x - 1,
                y: self.y,
            },
            Coord {
                x: self.x,
                y: self.y + 1,
            },
            Coord {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}

impl World {
    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..=self.size.y {
            for x in 0..=self.size.x {
                let coord = Coord { x, y };
                if self.players.contains(&coord) {
                    print!("P");
                } else if coord == self.start || coord == self.finish {
                    print!(".");
                } else if self.blizzards.contains_key(&coord) {
                    let blizzards = &self.blizzards[&coord];
                    if blizzards.len() > 1 {
                        print!("{}", blizzards.len());
                    } else {
                        match blizzards[0] {
                            Blizzard { dx: -1, dy: 0 } => print!("<"),
                            Blizzard { dx: 1, dy: 0 } => print!(">"),
                            Blizzard { dx: 0, dy: -1 } => print!("^"),
                            Blizzard { dx: 0, dy: 1 } => print!("v"),
                            _ => panic!(),
                        }
                    }
                } else if x == 0 || y == 0 || coord.x == self.size.x || coord.y == self.size.y {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn move_blizzards(&self) -> Blizzards {
        let mut result: Blizzards = HashMap::new();

        for (coord, blizzards) in self.blizzards.iter() {
            for blizzard in blizzards {
                // move
                let mut new_coord = Coord {
                    x: coord.x + blizzard.dx,
                    y: coord.y + blizzard.dy,
                };

                // wrap
                if new_coord.x == 0 {
                    new_coord.x = self.size.x - 1;
                }
                if new_coord.x == self.size.x {
                    new_coord.x = 1;
                }
                if new_coord.y == 0 {
                    new_coord.y = self.size.y - 1;
                }
                if new_coord.y == self.size.y {
                    new_coord.y = 1;
                }

                // append to result
                if !result.contains_key(&new_coord) {
                    result.insert(new_coord.clone(), vec![]);
                }
                result.get_mut(&new_coord).unwrap().push(blizzard.clone());
            }
        }

        result
    }

    fn step(self) -> (World, bool) {
        // move all blizzards
        let new_blizzards = self.move_blizzards();
        let mut new_players = HashSet::new();
        let mut finished = false;

        // move each player
        for old_player in self.players {
            for candidate in old_player.neighbours() {
                // skip the candidate position if it is not start or finish
                // ..or if it is outside of the world boundaries
                // ..or if it collides with some blizzard
                if candidate != self.start
                    && candidate != self.finish
                    && (candidate.x <= 0
                        || candidate.y <= 0
                        || candidate.x >= self.size.x
                        || candidate.y >= self.size.y
                        || new_blizzards.contains_key(&candidate))
                {
                    continue;
                }

                if candidate == self.finish {
                    finished = true;
                }

                new_players.insert(candidate);
            }
        }

        (
            World {
                blizzards: new_blizzards,
                players: new_players,
                start: self.start.clone(),
                finish: self.finish.clone(),
                size: self.size.clone(),
            },
            finished,
        )
    }
}

#[derive(Debug)]
struct ParseWorldError {}

impl FromStr for World {
    type Err = ParseWorldError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blizzards = Blizzards::new();
        let blizzard_chars = ['>', '<', '^', 'v'];
        let lines = s.split('\n').filter(|line| !line.is_empty());
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in lines.enumerate() {
            for (x, char) in line.chars().enumerate() {
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
                if !blizzard_chars.contains(&char) {
                    continue;
                }
                let coord = Coord {
                    x: x as i32,
                    y: y as i32,
                };
                if !blizzards.contains_key(&coord) {
                    blizzards.insert(coord.clone(), vec![]);
                }
                let blizzard = match char {
                    '<' => Blizzard { dx: -1, dy: 0 },
                    '>' => Blizzard { dx: 1, dy: 0 },
                    '^' => Blizzard { dx: 0, dy: -1 },
                    'v' => Blizzard { dx: 0, dy: 1 },
                    _ => return Err(ParseWorldError {}),
                };
                blizzards.get_mut(&coord).unwrap().push(blizzard);
            }
        }

        Ok(World {
            blizzards,
            players: HashSet::from([Coord { x: 1, y: 0 }]),
            start: Coord { x: 1, y: 0 },
            finish: Coord {
                x: max_x as i32 - 1,
                y: max_y as i32,
            },
            size: Coord {
                x: max_x as i32,
                y: max_y as i32,
            },
        })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut world: World = std::fs::read_to_string(&args[1]).unwrap().parse().unwrap();
    let mut total = 0;

    // to finish
    for step in 1.. {
        let finished;
        (world, finished) = world.step();
        if finished {
            println!("Finished in {} steps", step);
            total += step;
            break;
        }
    }

    // back to start
    world.players.clear();
    swap(&mut world.start, &mut world.finish);
    world.players = HashSet::from([world.start.clone()]);
    for step in 1.. {
        let finished;
        (world, finished) = world.step();
        if finished {
            println!("Finished in {} steps", step);
            total += step;
            break;
        }
    }

    // to finish again
    world.players.clear();
    swap(&mut world.start, &mut world.finish);
    world.players = HashSet::from([world.start.clone()]);
    for step in 1.. {
        let finished;
        (world, finished) = world.step();
        if finished {
            println!("Finished in {} steps", step);
            total += step;
            break;
        }
    }

    println!("Total steps: {}", total);
}
