use kdam::tqdm;

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
}

impl Chamber {
    fn new() -> Self {
        Self { rocks: Vec::new() }
    }

    fn top(&self) -> usize {
        for y in (0..self.rocks.len()).rev() {
            if self.rocks[y].iter().any(|&x| x) {
                return y + 1;
            }
        }
        return 0;
    }

    fn add_rock(&mut self, jet_pattern: &mut impl Iterator<Item = Jet>, rock: &[Coord]) {
        let top = self.top() as i64;
        //println!("Top is now {}", top);
        let mut rock: Vec<Coord> = rock
            .iter()
            .map(|c| Coord {
                x: c.x + 2,
                y: c.y + top + 3,
            })
            .collect();

        let mut downs = 0;
        loop {
            let push = jet_pattern.next().unwrap();
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
            if !self.is_collision(&candidate, downs >= 3) {
                rock = candidate;
            }

            let candidate: Vec<Coord> = rock.iter().map(|c| Coord { x: c.x, y: c.y - 1 }).collect();
            //println!("About to move down: {:?}", candidate);
            if self.is_collision(&candidate, downs >= 3) {
                for c in rock {
                    let y = c.y as usize;
                    if y >= self.rocks.len() {
                        self.rocks.resize((c.y + 1) as usize, [false; 7]);
                    }
                    self.rocks[y][c.x as usize] = true;
                }
                break;
            } else {
                rock = candidate;
                downs += 1;
            }
        }
    }

    fn is_collision(&self, candidate: &Vec<Coord>, with_other_rocks: bool) -> bool {
        for c in candidate {
            if c.x >= 7 || c.x < 0 || c.y < 0 {
                return true;
            }
            if with_other_rocks && (c.y as usize) < self.rocks.len() && self.rocks[c.y as usize][c.x as usize] {
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
    let rocks_count = std::env::args()
        .nth(2)
        .or(Some("1000000000000".into()))
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();
    let mut jet_pattern = parse_jet_pattern(&file_content);
    let mut chamber = Chamber::new();
    let rocks = ROCKS.iter().cycle().take(rocks_count);
    for &rock in tqdm!(rocks) {
        chamber.add_rock(&mut jet_pattern, rock);
    }
    chamber.print();
    println!("{}", chamber.top());
}
