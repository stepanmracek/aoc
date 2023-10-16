use std::collections::HashMap;

enum Jet {
    Left,
    Right,
}

struct Coord {
    x: i64,
    y: i64,
}

struct Chamber {
    rocks: Vec<[bool; 7]>,
    jets: Vec<Jet>,
    next_rock: usize,
    next_jet: usize,
    states: HashMap<StateKey, StateValue>,
    curent_rocks_count: usize,
    target_rocks_count: usize,
    top_offset: usize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct StateKey {
    pattern: [usize; 7],
    next_rock: usize,
    next_jet: usize,
}

#[derive(Debug)]
struct StateValue {
    top: usize,
    rocks: usize,
}

impl Chamber {
    fn new(jets: Vec<Jet>, target_rocks_count: usize) -> Self {
        Self {
            rocks: Vec::new(),
            jets,
            next_rock: 0,
            next_jet: 0,
            states: HashMap::new(),
            curent_rocks_count: 0,
            target_rocks_count,
            top_offset: 0,
        }
    }

    fn top(&self) -> usize {
        for y in (0..self.rocks.len()).rev() {
            if self.rocks[y].iter().any(|&x| x) {
                return y + 1;
            }
        }
        return 0;
    }

    fn top_pattern(&self) -> [usize; 7] {
        let mut pattern = [usize::MAX; 7];
        for x in 0..7 {
            for (i, y) in (0..self.rocks.len()).rev().enumerate() {
                if self.rocks[y][x] {
                    pattern[x] = i;
                    break;
                }
            }
        }
        pattern
    }

    fn simulate(&mut self) {
        loop {
            let top = self.top() as i64;
            //println!("Top is now {}", top);
            let mut rock: Vec<Coord> = ROCKS[self.next_rock]
                .iter()
                .map(|c| Coord {
                    x: c.x + 2,
                    y: c.y + top + 3,
                })
                .collect();
            self.next_rock = (self.next_rock + 1) % ROCKS.len();

            loop {
                let push = &self.jets[self.next_jet];
                self.next_jet = (self.next_jet + 1) % self.jets.len();
                let candidate: Vec<Coord> = rock
                    .iter()
                    .map(|c| Coord {
                        x: match push {
                            Jet::Left => c.x - 1,
                            Jet::Right => c.x + 1,
                        },
                        y: c.y,
                    })
                    .collect();
                //println!("About to move {:?}: {:?}", push, candidate);
                if !self.is_collision(&candidate) {
                    rock = candidate;
                }

                let candidate: Vec<Coord> =
                    rock.iter().map(|c| Coord { x: c.x, y: c.y - 1 }).collect();
                //println!("About to move down: {:?}", candidate);
                if self.is_collision(&candidate) {
                    for c in rock {
                        let y = c.y as usize;
                        if y >= self.rocks.len() {
                            self.rocks.resize((c.y + 1) as usize, [false; 7]);
                        }
                        self.rocks[y][c.x as usize] = true;
                    }
                    self.curent_rocks_count += 1;
                    break;
                } else {
                    rock = candidate;
                }
            }

            let new_state = StateKey {
                pattern: self.top_pattern(),
                next_jet: self.next_jet,
                next_rock: self.next_rock,
            };
            let new_state_value = StateValue {
                top: self.top(),
                rocks: self.curent_rocks_count,
            };
            //println!("{:?}", state);

            if self.top_offset == 0 {
                if let Some(old_state_value) = self.states.get(&new_state) {
                    let grow = new_state_value.top - old_state_value.top;
                    let rocks = new_state_value.rocks - old_state_value.rocks;
                    let remaining_rocks = self.target_rocks_count - self.curent_rocks_count;
                    let shortcut_cycle_count = remaining_rocks / rocks;
                    if shortcut_cycle_count > 0 && grow > 0 {
                        self.curent_rocks_count += shortcut_cycle_count * rocks;
                        self.top_offset = shortcut_cycle_count * grow;
                    }
                } else {
                    self.states.insert(new_state, new_state_value);
                }
            }

            if self.curent_rocks_count == self.target_rocks_count {
                break;
            }
        }
    }

    fn is_collision(&self, candidate: &Vec<Coord>) -> bool {
        for c in candidate {
            if c.x >= 7 || c.x < 0 || c.y < 0 {
                return true;
            }
            if (c.y as usize) < self.rocks.len() && self.rocks[c.y as usize][c.x as usize] {
                return true;
            }
        }
        false
    }

    fn print(&self) {
        for y in (0..self.rocks.len()).rev() {
            print!("|");
            for x in 0..7 {
                print!("{}", if self.rocks[y][x] { '#' } else { ' ' })
            }
            println!("| {}", y);
        }
        println!("+-------+");
    }
}

fn parse_jet_pattern(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .map(|c| if c == '<' { Jet::Left } else { Jet::Right })
        .collect()
}

const ROCKS: &[&[Coord]] = &[
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 3, y: 0 },
    ],
    &[
        Coord { x: 1, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 1 },
        Coord { x: 2, y: 1 },
        Coord { x: 1, y: 2 },
    ],
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 2, y: 1 },
        Coord { x: 2, y: 2 },
    ],
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 0, y: 2 },
        Coord { x: 0, y: 3 },
    ],
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 1 },
    ],
];

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let target_rocks_count: usize = std::env::args()
        .nth(2)
        .or(Some("1000000000000".into()))
        .unwrap()
        .parse()
        .unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();
    let jet_pattern = parse_jet_pattern(&file_content);
    let mut chamber = Chamber::new(jet_pattern, target_rocks_count);

    chamber.simulate();
    //chamber.print();
    println!("Chamber height: {:?}", chamber.top() + chamber.top_offset);
}
