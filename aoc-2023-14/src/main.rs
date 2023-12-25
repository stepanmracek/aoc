use std::{collections::hash_map::DefaultHasher, collections::HashMap, hash::Hasher};

type World = Vec<Vec<Rock>>;

#[derive(Clone, PartialEq)]
enum Rock {
    Rounded,
    CubeShaped,
    Empty,
}

struct Hole {
    start: usize,
    len: usize,
    round_count: usize,
}

fn parse_world(s: &str) -> World {
    let mut result = vec![];

    for line in s.lines() {
        let row = line
            .chars()
            .filter_map(|c| match c {
                'O' => Some(Rock::Rounded),
                '#' => Some(Rock::CubeShaped),
                '.' => Some(Rock::Empty),
                _ => None,
            })
            .collect();
        result.push(row);
    }

    result
}

fn print_world(w: &World) {
    for row in w {
        let row = row
            .iter()
            .map(|r| match r {
                Rock::Rounded => 'O',
                Rock::CubeShaped => '#',
                Rock::Empty => '.',
            })
            .collect::<String>();
        println!("{}", row);
    }
}

fn get_vertical_holes(w: &World, col: usize) -> Vec<Hole> {
    let mut holes = vec![];

    let mut in_hole = false;
    let mut start = 0;
    let mut round_count = 0;
    for r in 0..w.len() {
        let rock = &w[r][col];
        if rock == &Rock::CubeShaped {
            if in_hole {
                holes.push(Hole {
                    start,
                    len: r - start,
                    round_count,
                });
            }
            in_hole = false;
        } else {
            if !in_hole {
                in_hole = true;
                start = r;
                round_count = 0
            }
            if rock == &Rock::Rounded {
                round_count += 1;
            }
        }
    }
    if in_hole {
        holes.push(Hole {
            start,
            len: w.len() - start,
            round_count,
        });
    }

    holes
}

fn get_horizontal_holes(w: &World, row: usize) -> Vec<Hole> {
    let mut holes = vec![];

    let mut in_hole = false;
    let mut start = 0;
    let mut round_count = 0;
    for c in 0..w[0].len() {
        let rock = &w[row][c];
        if rock == &Rock::CubeShaped {
            if in_hole {
                holes.push(Hole {
                    start,
                    len: c - start,
                    round_count,
                });
            }
            in_hole = false;
        } else {
            if !in_hole {
                in_hole = true;
                start = c;
                round_count = 0
            }
            if rock == &Rock::Rounded {
                round_count += 1;
            }
        }
    }
    if in_hole {
        holes.push(Hole {
            start,
            len: w[0].len() - start,
            round_count,
        });
    }

    holes
}

fn tilt_north(w: &mut World) {
    for c in 0..w[0].len() {
        let holes = get_vertical_holes(w, c);
        for hole in holes {
            for r in 0..hole.len {
                let rock = if r < hole.round_count {
                    Rock::Rounded
                } else {
                    Rock::Empty
                };
                w[hole.start + r][c] = rock;
            }
        }
    }
}

fn tilt_east(w: &mut World) {
    for r in 0..w.len() {
        let holes = get_horizontal_holes(w, r);
        for hole in holes {
            for (i, c) in (0..hole.len).rev().enumerate() {
                let rock = if i < hole.round_count {
                    Rock::Rounded
                } else {
                    Rock::Empty
                };
                w[r][hole.start + c] = rock;
            }
        }
    }
}

fn tilt_south(w: &mut World) {
    for c in 0..w[0].len() {
        let holes = get_vertical_holes(w, c);
        for hole in holes {
            for (i, r) in (0..hole.len).rev().enumerate() {
                let rock = if i < hole.round_count {
                    Rock::Rounded
                } else {
                    Rock::Empty
                };
                w[hole.start + r][c] = rock;
            }
        }
    }
}

fn tilt_west(w: &mut World) {
    for r in 0..w.len() {
        let holes = get_horizontal_holes(w, r);
        for hole in holes {
            for c in 0..hole.len {
                let rock = if c < hole.round_count {
                    Rock::Rounded
                } else {
                    Rock::Empty
                };
                w[r][hole.start + c] = rock;
            }
        }
    }
}

fn compute_load(w: &World) -> usize {
    let mut load = 0;
    for (i, row) in w.iter().enumerate() {
        let round_count = row.iter().filter(|r| r == &&Rock::Rounded).count();
        load += (w.len() - i) * round_count;
    }
    load
}

fn hash(w: &World) -> u64 {
    let mut hasher = DefaultHasher::new();
    for r in 0..w.len() {
        for c in 0..w[0].len() {
            if w[r][c] == Rock::Rounded {
                hasher.write_usize(r);
                hasher.write_usize(c);
            }
        }
    }
    hasher.finish()
}

fn main() {
    let path = std::env::args().nth(1).expect("Missing file path argument");
    let file_content = std::fs::read_to_string(path).expect("Unable to read file");
    let mut world = parse_world(&file_content);

    let mut known_hashes = HashMap::new();
    let mut shortcut_found = false;
    let cycles = 1_000_000_000;
    let mut cycle = 1;
    loop {
        tilt_north(&mut world);
        tilt_west(&mut world);
        tilt_south(&mut world);
        tilt_east(&mut world);

        if !shortcut_found {
            let h = hash(&world);
            println!("cycle:{} hash:{} load:{}", cycle, h, compute_load(&world));
            if let Some(last_cycle) = known_hashes.get(&h) {
                println!(
                    "Known hash spotted; last cycle: {}, current cycle: {}",
                    last_cycle, cycle
                );
                shortcut_found = true;

                let remaining = cycles - cycle;
                let skipped_cycles = cycle - last_cycle;
                cycle += (remaining / skipped_cycles) * skipped_cycles;
                println!("Skipping to cycle: {}", cycle);
            } else {
                known_hashes.insert(h, cycle);
            }
        }

        if cycle == cycles {
            break;
        }
        cycle += 1
    }
    print_world(&world);
    println!("load:{}", compute_load(&world));
}
