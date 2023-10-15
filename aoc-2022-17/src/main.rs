use kdam::tqdm;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Jet {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

struct Chamber {
    rocks: HashSet<Coord>,
}

impl Chamber {
    fn new() -> Self {
        Self {
            rocks: HashSet::new(),
        }
    }

    fn top(&self) -> i64 {
        self.rocks.iter().map(|c| c.y).max().unwrap_or(0)
    }

    fn add_rock(&mut self, jet_pattern: &mut impl Iterator<Item = Jet>, rock: &[Coord]) {
        let top = self.top();
        //println!("Top is now {}", top);
        let mut rock: HashSet<Coord> = HashSet::from_iter(rock.iter().map(|c| Coord {
            x: c.x + 2,
            y: c.y + top + 3 + 1,
        }));

        let mut downs = 0;
        loop {
            let push = jet_pattern.next().unwrap();
            let candidate = HashSet::from_iter(rock.iter().map(|c| Coord {
                x: match push {
                    Jet::Left => c.x - 1,
                    Jet::Right => c.x + 1,
                },
                y: c.y,
            }));
            //println!("About to move {:?}: {:?}", push, candidate);
            if !self.is_collision(&candidate, downs >= 3) {
                rock = candidate;
            }

            let candidate = HashSet::from_iter(rock.iter().map(|c| Coord { x: c.x, y: c.y - 1 }));
            //println!("About to move down: {:?}", candidate);
            if self.is_collision(&candidate, downs >= 3) {
                self.rocks.extend(rock.iter().cloned());
                break;
            } else {
                rock = candidate;
                downs += 1;
            }
        }
    }

    fn is_collision(&self, candidate: &HashSet<Coord>, with_other_rocks: bool) -> bool {
        for c in candidate {
            if c.x >= 7 || c.x < 0 || c.y <= 0 {
                return true;
            }
        }
        if with_other_rocks {
            if self.rocks.intersection(candidate).count() > 0 {
                return true;
            }
        }
        false
    }

    fn print(&self) {
        let mut y = self.top() + 3;
        loop {
            print!("|");
            for x in 0..7 {
                print!(
                    "{}",
                    if self.rocks.contains(&Coord { x, y }) {
                        '#'
                    } else {
                        ' '
                    }
                )
            }
            println!("| {}", y);

            y -= 1;
            if y == 0 {
                println!("+-------+");
                break;
            }
        }
    }
}

fn parse_jet_pattern(input: &str) -> impl Iterator<Item = Jet> + '_ {
    input
        .trim()
        .chars()
        .map(|c| if c == '<' { Jet::Left } else { Jet::Right })
        .cycle()
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
    let rocks_count = std::env::args().nth(2).or(Some("2022".into())).unwrap().parse::<usize>().unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();
    let mut jet_pattern = parse_jet_pattern(&file_content);
    let mut chamber = Chamber::new();
    let rocks = ROCKS.iter().cycle().take(rocks_count); //1_000_000_000_000);
    for &rock in tqdm!(rocks) {
        chamber.add_rock(&mut jet_pattern, rock);
    }
    chamber.print();
    println!("{}", chamber.top());
}
