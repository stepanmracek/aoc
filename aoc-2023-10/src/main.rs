use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    r: usize,
    c: usize,
}

#[derive(Debug)]
struct World {
    pipes: Vec<Vec<HashSet<Direction>>>,
    start: Coord,
}

#[derive(Debug)]
struct ParseWorldError {}

impl FromStr for World {
    type Err = ParseWorldError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.split('\n').filter(|line| !line.is_empty()).enumerate();
        let mut pipes = vec![];
        let mut start = Coord { r: 0, c: 0 };
        for (r, row_str) in rows {
            let mut row = vec![];
            for (c, col_char) in row_str.chars().enumerate() {
                let pipe = match col_char {
                    '|' => HashSet::from([Direction::North, Direction::South]),
                    '-' => HashSet::from([Direction::East, Direction::West]),
                    'L' => HashSet::from([Direction::North, Direction::East]),
                    'J' => HashSet::from([Direction::North, Direction::West]),
                    '7' => HashSet::from([Direction::South, Direction::West]),
                    'F' => HashSet::from([Direction::South, Direction::East]),
                    'S' => {
                        start.r = r;
                        start.c = c;
                        HashSet::from([])
                    }
                    _ => HashSet::from([]),
                };
                row.push(pipe);
            }
            pipes.push(row)
        }

        Ok(World { pipes, start })
    }
}

impl World {
    fn starting_directions(&self) -> Vec<Direction> {
        let mut result = vec![];
        if self.start.r > 0
            && self.pipes[self.start.r - 1][self.start.c].contains(&Direction::South)
        {
            result.push(Direction::North);
        }
        if self.pipes[self.start.r + 1][self.start.c].contains(&Direction::North) {
            result.push(Direction::South);
        }
        if self.pipes[self.start.r][self.start.c + 1].contains(&Direction::West) {
            result.push(Direction::East);
        }
        if self.start.c > 0 && self.pipes[self.start.r][self.start.c - 1].contains(&Direction::East)
        {
            result.push(Direction::West);
        }
        result
    }

    fn step(&self, from: &Coord, in_direction: &Direction) -> (Coord, Direction) {
        let coord = match in_direction {
            Direction::West => Coord {
                r: from.r,
                c: from.c - 1,
            },
            Direction::East => Coord {
                r: from.r,
                c: from.c + 1,
            },
            Direction::North => Coord {
                r: from.r - 1,
                c: from.c,
            },
            Direction::South => Coord {
                r: from.r + 1,
                c: from.c,
            },
        };

        let inv_in_direction = match in_direction {
            Direction::West => Direction::East,
            Direction::East => Direction::West,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
        };

        let out_direction = self.pipes[coord.r][coord.c]
            .iter()
            .find(|&d| d != &inv_in_direction)
            .unwrap();

        (coord, out_direction.clone())
    }

    fn triple_coord(&self, coord: &Coord, pipe_directions: &HashSet<Direction>) -> Vec<Coord> {
        let mut result = vec![Coord {
            r: coord.r * 3 + 1,
            c: coord.c * 3 + 1,
        }];
        if pipe_directions.contains(&Direction::North) {
            result.push(Coord {
                r: coord.r * 3,
                c: coord.c * 3 + 1,
            })
        }
        if pipe_directions.contains(&Direction::South) {
            result.push(Coord {
                r: coord.r * 3 + 2,
                c: coord.c * 3 + 1,
            })
        }
        if pipe_directions.contains(&Direction::East) {
            result.push(Coord {
                r: coord.r * 3 + 1,
                c: coord.c * 3 + 2,
            })
        }
        if pipe_directions.contains(&Direction::West) {
            result.push(Coord {
                r: coord.r * 3 + 1,
                c: coord.c * 3,
            })
        }
        result
    }

    fn compute_loop(&self) -> HashSet<Coord> {
        let start_directions = self.starting_directions();
        let mut loop_coords: HashSet<Coord> = self
            .triple_coord(
                &self.start,
                &HashSet::from_iter(start_directions.iter().cloned()),
            )
            .iter()
            .cloned()
            .collect();
        println!("starting directions: {:?}", start_directions);

        let mut first = (self.start.clone(), start_directions[0].clone());
        let mut second = (self.start.clone(), start_directions[1].clone());

        for step in 1.. {
            first = self.step(&first.0, &first.1);
            second = self.step(&second.0, &second.1);

            loop_coords.extend(self.triple_coord(&first.0, &self.pipes[first.0.r][first.0.c]));
            loop_coords.extend(self.triple_coord(&second.0, &self.pipes[second.0.r][second.0.c]));

            println!("step: {}", step);
            println!("first: {:?}", self.step(&self.start, &start_directions[0]));
            println!("first: {:?}", self.step(&self.start, &start_directions[1]));

            if first.0 == second.0 {
                break;
            }
        }

        loop_coords
    }
}

fn flood_fill(coord: &Coord, pipes: &HashSet<Coord>, max: &Coord, outer: &mut HashSet<Coord>) {
    if pipes.contains(coord) {
        return;
    }
    if outer.contains(coord) {
        return;
    }
    outer.insert(coord.clone());
    if coord.r > 0 {
        flood_fill(
            &Coord {
                r: coord.r - 1,
                c: coord.c,
            },
            pipes,
            max,
            outer,
        );
    }
    if coord.c > 0 {
        flood_fill(
            &Coord {
                r: coord.r,
                c: coord.c - 1,
            },
            pipes,
            max,
            outer,
        );
    }
    if coord.r < max.r {
        flood_fill(
            &Coord {
                r: coord.r + 1,
                c: coord.c,
            },
            pipes,
            max,
            outer,
        );
    }
    if coord.c < max.c {
        flood_fill(
            &Coord {
                r: coord.r,
                c: coord.c + 1,
            },
            pipes,
            max,
            outer,
        );
    }
}

fn is_original_coord(tripple_coord: &Coord) -> bool {
    tripple_coord.r > 0
        && tripple_coord.c > 0
        && (tripple_coord.r - 1) % 3 == 0
        && (tripple_coord.c - 1) % 3 == 0
}

fn main() {
    let file_content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let world = file_content.parse::<World>().unwrap();
    let loop_cords = world.compute_loop();
    let max = Coord {
        r: world.pipes.len() * 3,
        c: world.pipes[0].len() * 3,
    };
    let mut outer = HashSet::new();
    flood_fill(&Coord { r: 0, c: 0 }, &loop_cords, &max, &mut outer);
    let mut inner_count = 0;
    for r in 0..world.pipes.len() * 3 {
        for c in 0..world.pipes[0].len() * 3 {
            let coord = Coord { r, c };
            if loop_cords.contains(&coord) {
                print!("X");
            } else if outer.contains(&coord) && is_original_coord(&coord) {
                print!("O");
            } else if is_original_coord(&coord) {
                print!("I");
                inner_count += 1;
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("Inner count: {}", inner_count);
}
