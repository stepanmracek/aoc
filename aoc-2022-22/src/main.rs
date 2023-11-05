#[derive(Debug, Clone)]
enum Rotate {
    Left = -1,
    Right = 1,
}

#[derive(Debug)]
enum Instruction {
    Rotate(Rotate),
    Move(usize),
}

#[derive(Debug, PartialEq)]
enum Tile {
    Open,
    Wall,
}

type World = Vec<Vec<Option<Tile>>>;

#[derive(Debug, Clone)]
enum Orientation {
    Right = 0,
    Down,
    Left,
    Up,
}

impl From<i32> for Orientation {
    fn from(value: i32) -> Self {
        match value {
            0 => Orientation::Right,
            1 => Orientation::Down,
            2 => Orientation::Left,
            3 => Orientation::Up,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct State {
    row: usize,
    col: usize,
    orientation: Orientation,
}

impl State {
    fn init(world: &World) -> Self {
        let mut row = 0;
        let mut col = 0;

        'outer: for r in world {
            col = 0;
            for tile in r {
                if let Some(tile) = tile {
                    if tile == &Tile::Open {
                        break 'outer;
                    }
                }
                col += 1;
            }
            row += 1;
        }

        State {
            row,
            col,
            orientation: Orientation::Right,
        }
    }

    fn get_next(&self, world: &World, cur_row: usize, cur_col: usize) -> Option<(usize, usize)> {
        let mut next_row = cur_row;
        let mut next_col = cur_col;
        match self.orientation {
            Orientation::Up => next_row = (cur_row + world.len() - 1) % world.len(),
            Orientation::Down => next_row = (cur_row + 1) % world.len(),
            Orientation::Right => next_col = (cur_col + 1) % world[cur_row].len(),
            Orientation::Left => {
                next_col = (cur_col + world[cur_row].len() - 1) % world[cur_row].len()
            }
        };
        let next = &world[next_row][next_col];
        match next {
            None => self.get_next(world, next_row, next_col),
            Some(Tile::Open) => Some((next_row, next_col)),
            Some(Tile::Wall) => None,
        }
    }

    fn apply_instruction(&self, world: &World, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Rotate(rotate) => {
                let old_orientation = self.orientation.clone() as i32;
                let new_orientation = ((old_orientation + rotate.clone() as i32) + 4) % 4;
                return State {
                    row: self.row,
                    col: self.col,
                    orientation: new_orientation.into(),
                };
            }
            Instruction::Move(steps) => {
                let mut row = self.row;
                let mut col = self.col;
                for _ in 0..*steps {
                    let next = self.get_next(world, row, col);
                    match next {
                        Some((new_row, new_col)) => {
                            row = new_row;
                            col = new_col;
                        }
                        None => break,
                    }
                }
                return State {
                    row,
                    col,
                    orientation: self.orientation.clone(),
                };
            }
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut move_buffer: String = "".into();
    let mut result = vec![];
    for c in input.trim().chars() {
        match c {
            'L' => {
                if !move_buffer.is_empty() {
                    result.push(Instruction::Move(move_buffer.parse().unwrap()));
                    move_buffer.clear();
                }
                result.push(Instruction::Rotate(Rotate::Left));
            }
            'R' => {
                if !move_buffer.is_empty() {
                    result.push(Instruction::Move(move_buffer.parse().unwrap()));
                    move_buffer.clear();
                }
                result.push(Instruction::Rotate(Rotate::Right));
            }
            _ => move_buffer.push(c),
        }
    }
    if !move_buffer.is_empty() {
        result.push(Instruction::Move(move_buffer.parse().unwrap()));
        move_buffer.clear();
    }
    result
}

fn parse_world(input: &str) -> World {
    let mut world = vec![];
    let mut max_cols = 0;

    for line in input.split('\n') {
        let row: Vec<Option<Tile>> = line
            .chars()
            .map(|c| match c {
                ' ' => None,
                '.' => Some(Tile::Open),
                '#' => Some(Tile::Wall),
                _ => panic!(),
            })
            .collect();
        let cols = row.len();
        world.push(row);
        if cols > max_cols {
            max_cols = cols;
        }
    }

    for row in world.iter_mut() {
        row.resize_with(max_cols, || None);
    }

    world
}

fn print_world(world: &World) {
    for row in world {
        for tile in row {
            let c = match tile {
                None => 'x',
                Some(tile) => match tile {
                    Tile::Open => '.',
                    Tile::Wall => '#',
                },
            };
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let input_file: Vec<_> = std::fs::read_to_string(arg)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let instructions = parse_instructions(input_file[1].as_str());
    println!("{:?}", instructions);

    let world = parse_world(input_file[0].as_str());
    print_world(&world);

    let mut state = State::init(&world);
    println!("{:?}", state);

    for instruction in instructions {
        state = state.apply_instruction(&world, &instruction);
        println!("{:?}", state);
    }

    let result = (state.row + 1) * 1000 + 4 * (state.col + 1) + state.orientation as usize;
    println!("Result: {}", result)
}
