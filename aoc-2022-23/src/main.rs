use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    r: i32,
    c: i32,
}

#[derive(Debug)]
struct Boundary {
    min_r: i32,
    min_c: i32,
    max_r: i32,
    max_c: i32,
}

type World = HashSet<Coord>;

impl Coord {
    fn n(&self) -> Self {
        Self {
            r: self.r - 1,
            c: self.c,
        }
    }

    fn ne(&self) -> Self {
        Self {
            r: self.r - 1,
            c: self.c + 1,
        }
    }

    fn nw(&self) -> Self {
        Self {
            r: self.r - 1,
            c: self.c - 1,
        }
    }

    fn s(&self) -> Self {
        Self {
            r: self.r + 1,
            c: self.c,
        }
    }

    fn se(&self) -> Self {
        Self {
            r: self.r + 1,
            c: self.c + 1,
        }
    }

    fn sw(&self) -> Self {
        Self {
            r: self.r + 1,
            c: self.c - 1,
        }
    }

    fn e(&self) -> Self {
        Self {
            r: self.r,
            c: self.c + 1,
        }
    }

    fn w(&self) -> Self {
        Self {
            r: self.r,
            c: self.c - 1,
        }
    }
}

fn parse_world(input: &str) -> World {
    let mut world = World::new();

    for (r, row) in input
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        for (c, ch) in row.chars().enumerate() {
            if ch == '#' {
                world.insert(Coord {
                    r: r as i32,
                    c: c as i32,
                });
            }
        }
    }
    world
}

fn get_world_boundary(world: &World) -> Boundary {
    let mut b = Boundary {
        min_r: i32::MAX,
        min_c: i32::MAX,
        max_r: i32::MIN,
        max_c: i32::MIN,
    };
    for coord in world.iter() {
        if coord.r > b.max_r {
            b.max_r = coord.r;
        }
        if coord.c > b.max_c {
            b.max_c = coord.c;
        }
        if coord.r < b.min_r {
            b.min_r = coord.r;
        }
        if coord.c < b.min_c {
            b.min_c = coord.c;
        }
    }
    b
}

fn print_world(world: &World) {
    let b = get_world_boundary(world);
    //println!("{:?}", b);
    for r in b.min_r..=b.max_r {
        for c in b.min_c..=b.max_c {
            if world.contains(&Coord { r, c }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn count_empty(world: &World) -> usize {
    let b = get_world_boundary(world);
    let mut count = 0;
    for r in b.min_r..=b.max_r {
        for c in b.min_c..=b.max_c {
            if !world.contains(&Coord { r, c }) {
                count += 1;
            }
        }
    }
    count
}

fn step(world: &World, round: usize) -> (World, bool) {
    let mut new_world = World::new();

    // destination coord -> vector of source coordinates
    let mut proposals: HashMap<Coord, Vec<Coord>> = HashMap::new();

    // first half of each round
    for source in world {
        let destination = propose_move(world, source, round);

        if !proposals.contains_key(&destination) {
            proposals.insert(destination.clone(), vec![]);
        }

        proposals
            .get_mut(&destination)
            .unwrap()
            .push(source.clone());
    }

    // second half of the round:
    // Each Elf moves to their proposed destination tile if they were the only Elf to propose moving to that position
    // If two or more Elves propose moving to the same position, none of those Elves move.

    let mut moved = false;
    for (destination, sources) in proposals {
        if sources.len() == 1 {
            if sources[0] != destination {
                //println!("Moving from {:?} to {:?}", sources[0], destination);
                moved = true
            }
            new_world.insert(destination);
        } else {
            for src in sources {
                new_world.insert(src);
            }
        }
    }

    (new_world, moved)
}

fn check_north(world: &World, src: &Coord) -> Option<Coord> {
    // If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
    if !world.contains(&src.n()) && !world.contains(&src.ne()) && !world.contains(&src.nw()) {
        Some(src.n())
    } else {
        None
    }
}

fn check_south(world: &World, src: &Coord) -> Option<Coord> {
    // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
    if !world.contains(&src.s()) && !world.contains(&src.se()) && !world.contains(&src.sw()) {
        Some(src.s())
    } else {
        None
    }
}

fn check_west(world: &World, src: &Coord) -> Option<Coord> {
    // If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
    if !world.contains(&src.w()) && !world.contains(&src.nw()) && !world.contains(&src.sw()) {
        Some(src.w())
    } else {
        None
    }
}

fn check_east(world: &World, src: &Coord) -> Option<Coord> {
    // If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
    if !world.contains(&src.e()) && !world.contains(&src.ne()) && !world.contains(&src.se()) {
        Some(src.e())
    } else {
        None
    }
}

fn propose_move(world: &World, src: &Coord, round: usize) -> Coord {
    let checks = [check_north, check_south, check_west, check_east];

    // If no other Elves are in one of eight adjacent positions, the Elf does not do anything during this round
    if !world.contains(&src.n())
        && !world.contains(&src.ne())
        && !world.contains(&src.nw())
        && !world.contains(&src.s())
        && !world.contains(&src.se())
        && !world.contains(&src.sw())
        && !world.contains(&src.e())
        && !world.contains(&src.w())
    {
        return src.clone();
    }

    // Elf looks in each of four directions and proposes moving one step in the first valid direction
    for check_index in round..=round + 3 {
        if let Some(check_result) = checks[check_index % 4](world, src) {
            return check_result;
        }
    }

    src.clone()
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let mut world = parse_world(std::fs::read_to_string(arg).unwrap().as_str());
    let mut moved;
    for round in 0.. {
        (world, moved) = step(&world, round);
        println!("== End of Round {} ==; someone moved: {}", round + 1, moved);
        if !moved {
            break;
        }
    }
    print_world(&world);
    println!("{}", count_empty(&world));
}
