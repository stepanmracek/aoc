use std::collections::HashSet;

enum Mirror {
    Slash,
    Backslash,
}

enum Splitter {
    Horizontal,
    Vertical,
}

enum Item {
    Mirror(Mirror),
    Splitter(Splitter),
    Empty,
}

type World = Vec<Vec<Item>>;

fn parse_world(s: &str) -> World {
    let mut world = vec![];
    for line in s.lines() {
        let row = line
            .chars()
            .map(|c| match c {
                '/' => Item::Mirror(Mirror::Slash),
                '\\' => Item::Mirror(Mirror::Backslash),
                '|' => Item::Splitter(Splitter::Vertical),
                '-' => Item::Splitter(Splitter::Horizontal),
                '.' => Item::Empty,
                _ => panic!("Unknown world item"),
            })
            .collect::<Vec<Item>>();
        world.push(row);
    }
    world
}

fn print_world(world: &World) {
    for row in world.iter() {
        let row_str: String = row
            .iter()
            .map(|item| match item {
                Item::Mirror(Mirror::Slash) => '/',
                Item::Mirror(Mirror::Backslash) => '\\',
                Item::Splitter(Splitter::Vertical) => '|',
                Item::Splitter(Splitter::Horizontal) => '-',
                Item::Empty => '.',
            })
            .collect();
        println!("{}", row_str);
    }
}

fn is_in(world: &World, r: i32, c: i32) -> bool {
    r >= 0 && c >= 0 && r < world.len() as i32 && c < world[0].len() as i32
}

fn ray_trace(
    world: &World,
    visited: &mut HashSet<(usize, usize, i32, i32)>,
    r: usize,
    c: usize,
    dr: i32,
    dc: i32,
) {
    let key = (r, c, dr, dc);
    if visited.contains(&key) {
        return;
    }
    visited.insert(key);

    let item = &world[r][c];
    match item {
        Item::Empty => {
            let new_r = (r as i32) + dr;
            let new_c = (c as i32) + dc;
            if is_in(world, new_r, new_c) {
                ray_trace(world, visited, new_r as usize, new_c as usize, dr, dc);
            }
        }
        Item::Splitter(Splitter::Vertical) => {
            if dc != 0 {
                // Horizontal move
                let new_upper_row = (r as i32) - 1;
                if is_in(world, new_upper_row, c as i32) {
                    ray_trace(world, visited, new_upper_row as usize, c, -1, 0);
                }

                let new_lower_row = (r as i32) + 1;
                if is_in(world, new_lower_row, c as i32) {
                    ray_trace(world, visited, new_lower_row as usize, c, 1, 0);
                }
            } else {
                // vertical move
                let new_r = (r as i32) + dr;
                let new_c = (c as i32) + dc;
                if is_in(world, new_r, new_c) {
                    ray_trace(world, visited, new_r as usize, new_c as usize, dr, dc);
                }
            }
        }
        Item::Splitter(Splitter::Horizontal) => {
            if dc != 0 {
                // Horizontal move
                let new_r = (r as i32) + dr;
                let new_c = (c as i32) + dc;
                if is_in(world, new_r, new_c) {
                    ray_trace(world, visited, new_r as usize, new_c as usize, dr, dc);
                }
            } else {
                // vertical move
                let new_left_col = (c as i32) - 1;
                if is_in(world, r as i32, new_left_col) {
                    ray_trace(world, visited, r, new_left_col as usize, 0, -1);
                }

                let new_right_col = (c as i32) + 1;
                if is_in(world, r as i32, new_right_col) {
                    ray_trace(world, visited, r, new_right_col as usize, 0, 1);
                }
            }
        }
        Item::Mirror(Mirror::Slash) => {
            let new_dr;
            let new_dc;
            if dc > 0 && dr == 0 {
                // right -> up
                new_dr = -1;
                new_dc = 0;
            } else if dc < 0 && dr == 0 {
                // left -> down
                new_dr = 1;
                new_dc = 0;
            } else if dc == 0 && dr < 0 {
                // up -> right
                new_dr = 0;
                new_dc = 1;
            } else {
                // down -> left
                new_dr = 0;
                new_dc = -1;
            }
            let new_r = (r as i32) + new_dr;
            let new_c = (c as i32) + new_dc;
            if is_in(world, new_r, new_c) {
                ray_trace(
                    world,
                    visited,
                    new_r as usize,
                    new_c as usize,
                    new_dr,
                    new_dc,
                );
            }
        }
        Item::Mirror(Mirror::Backslash) => {
            let new_dr;
            let new_dc;
            if dc > 0 && dr == 0 {
                // right -> down
                new_dr = 1;
                new_dc = 0;
            } else if dc < 0 && dr == 0 {
                // left -> up
                new_dr = -1;
                new_dc = 0;
            } else if dc == 0 && dr < 0 {
                // up -> left
                new_dr = 0;
                new_dc = -1;
            } else {
                // down -> right
                new_dr = 0;
                new_dc = 1;
            }
            let new_r = (r as i32) + new_dr;
            let new_c = (c as i32) + new_dc;
            if is_in(world, new_r, new_c) {
                ray_trace(
                    world,
                    visited,
                    new_r as usize,
                    new_c as usize,
                    new_dr,
                    new_dc,
                );
            }
        }
    }
}

fn evaluate(world: &World, r: usize, c: usize, dr: i32, dc: i32) -> usize {
    let mut visited = HashSet::new();
    ray_trace(world, &mut visited, r, c, dr, dc);
    let energized: HashSet<(usize, usize)> = visited.iter().map(|(r, c, _, _)| (*r, *c)).collect();
    energized.len()
}

fn evaluate_all(world: &World) -> Vec<usize> {
    let mut result = vec![];
    for r in 0..world.len() {
        result.push(evaluate(world, r, 0, 0, 1));
        result.push(evaluate(world, r, world[0].len() - 1, 0, -1));
    }
    for c in 0..world[0].len() {
        result.push(evaluate(world, 0, c, 1, 0));
        result.push(evaluate(world, world.len() - 1, c, -1, 0));
    }

    result
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Missing input file argument");
    let file_content = std::fs::read_to_string(path).expect("Can't read input file");
    let world = parse_world(&file_content);
    print_world(&world);

    let energized_vals = evaluate_all(&world);
    let energized = energized_vals.iter().max();
    println!("{:?}", energized);
}
