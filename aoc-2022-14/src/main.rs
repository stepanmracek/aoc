use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Coord {
    col: usize,
    row: usize,
}

#[derive(Debug)]
struct Bounds {
    min_col: usize,
    max_col: usize,
    max_row: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum WorldItem {
    Air,
    Stone,
    Sand,
}

type World = HashMap<usize, Vec<WorldItem>>;

fn read_input(s: &String) -> Vec<Vec<Coord>> {
    let lines = s.split('\n').filter(|line| !line.is_empty());
    let instructions: Vec<Vec<_>> = lines
        .map(|line| {
            line.split(" -> ")
                .filter_map(|item| {
                    item.split_once(',').map(|(col, row)| Coord {
                        col: col.parse().unwrap(),
                        row: row.parse().unwrap(),
                    })
                })
                .collect()
        })
        .collect();
    instructions
}

fn get_bounds(instructions: &Vec<Vec<Coord>>) -> Bounds {
    let mut min_col = usize::MAX;
    let mut max_col = 0;
    let mut max_row = 0;
    for i in instructions.iter().flat_map(|line| line.iter()) {
        if i.row > max_row {
            max_row = i.row;
        }
        if i.col > max_col {
            max_col = i.col;
        }
        if i.col < min_col {
            min_col = i.col;
        }
    }

    Bounds {
        min_col,
        max_col,
        max_row,
    }
}

fn draw_line(world: &mut World, from: &Coord, to: &Coord) {
    if from.col == to.col {
        let col = world.get_mut(&from.col).unwrap();
        let start = from.row.min(to.row);
        let end = from.row.max(to.row);
        for row in start..=end {
            col[row] = WorldItem::Stone;
        }
    } else if from.row == to.row {
        let start = from.col.min(to.col);
        let end = from.col.max(to.col);
        for col in start..=end {
            world.get_mut(&col).unwrap()[from.row] = WorldItem::Stone;
        }
    }
}

fn create_world(instructions: &Vec<Vec<Coord>>) -> World {
    let bounds = get_bounds(instructions);
    let mut world: World = HashMap::new();
    let col_padding = 2 * (bounds.max_col - bounds.min_col);
    for col in (bounds.min_col - col_padding)..=(bounds.max_col + col_padding) {
        world.insert(col, vec![WorldItem::Air; bounds.max_row + 3]);
    }

    for line in instructions {
        for (from, to) in line.iter().tuple_windows() {
            draw_line(&mut world, from, to);
        }
    }
    draw_line(
        &mut world,
        &Coord {
            col: bounds.min_col - col_padding,
            row: bounds.max_row + 2,
        },
        &Coord {
            col: bounds.max_col + col_padding,
            row: bounds.max_row + 2,
        },
    );
    world
}

fn draw_world(world: &World) {
    let min_col = world.keys().min().unwrap();
    let max_col = world.keys().max().unwrap();
    let rows = world.get(min_col).unwrap().len();

    for row in 0..rows {
        for col in *min_col..=*max_col {
            let symbol = match world.get(&col).unwrap()[row] {
                WorldItem::Air => '·',
                WorldItem::Sand => 'o',
                WorldItem::Stone => '█',
            };
            print!("{}", symbol);
        }
        println!()
    }
}

fn fall(world: &World, start: Coord) -> Option<Coord> {
    let col = world.get(&start.col).unwrap();

    if col[start.row] != WorldItem::Air {
        return None;
    }

    let air = WorldItem::Air;
    let crash = col
        .iter()
        .enumerate()
        .find(|(index, &ref item)| index >= &start.row && item != &air);

    if let Some(crash) = crash {
        let crash_coord = Coord {
            col: start.col,
            row: crash.0 - 1,
        };

        // check lower left
        let lower_left = Coord {
            col: crash_coord.col - 1,
            row: crash_coord.row + 1,
        };
        if world.get(&lower_left.col).unwrap()[lower_left.row] == air {
            return fall(world, lower_left);
        }

        // check lower right
        let lower_right = Coord {
            col: crash_coord.col + 1,
            row: crash_coord.row + 1,
        };
        if world.get(&lower_right.col).unwrap()[lower_right.row] == air {
            return fall(world, lower_right);
        }

        return Some(crash_coord);
    }

    None
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();
    let instructions = read_input(&file_content);
    let mut world = create_world(&instructions);

    for counter in 0.. {
        let crash = fall(&world, Coord { col: 500, row: 0 });
        if let Some(crash) = crash {
            world.get_mut(&crash.col).unwrap()[crash.row] = WorldItem::Sand;
        } else {
            draw_world(&world);
            println!("{}", counter);
            break;
        }
    }
}
