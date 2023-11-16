use std::mem::swap;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone)]
struct State {
    row: usize,
    col: usize,
    orientation: Orientation,
}

/// used as a result of `get_adjacent_side()`. Describes which side is adjacent to some other
/// side given and how rows or column properties of `State` should change
#[derive(Debug)]
struct AdjacentSide {
    /// side row within unfolded cube map
    r: usize,
    /// side column within unfolded cube map
    c: usize,
    /// new orientation
    dir: Orientation,
    /// if row or column should be inverted (e.g. from column 40 to column 10)
    inv: bool,
    // if row and column properties of the state should be swapped
    swap_rc: bool,
}

impl State {
    /// Finds first free coordinate (Tile::Open) on the unfolded map
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

    /// Applies single instruction.
    /// The `get_next` parameter should be a function that for given state returns where the player
    /// should move on the map. Function should return `None` if there is wall in the way.
    /// The function should also handle the correct wrapping - either simple from the first part
    /// of the submission or more complex movement on folded cube.
    fn apply_instruction(
        &self,
        world: &World,
        instruction: &Instruction,
        get_next: fn(&World, &State) -> Option<State>,
    ) -> Self {
        match instruction {
            Instruction::Rotate(rotate) => {
                let old_orientation = self.orientation as i32;
                let new_orientation = ((old_orientation + *rotate as i32) + 4) % 4;
                State {
                    row: self.row,
                    col: self.col,
                    orientation: new_orientation.into(),
                }
            }
            Instruction::Move(steps) => {
                let mut state = self.clone();
                for _ in 0..*steps {
                    let next = get_next(world, &state);
                    match next {
                        Some(next) => state = next,
                        None => break,
                    }
                }
                state
            }
        }
    }
}

/// Simple implementation of obtaining an adjacent tile when the movement is happening strictly
/// in 2D world and when player reaches boundaries of the world, the movement continues on the
/// opposide side. Also it skips empty tiles.
fn get_next(world: &World, state: &State) -> Option<State> {
    let mut next_row = state.row;
    let mut next_col = state.col;
    let row_len = world[state.row].len();
    match state.orientation {
        Orientation::Up => next_row = (state.row + world.len() - 1) % world.len(),
        Orientation::Down => next_row = (state.row + 1) % world.len(),
        Orientation::Right => next_col = (state.col + 1) % row_len,
        Orientation::Left => next_col = (state.col + row_len - 1) % row_len,
    };
    let next = &world[next_row][next_col];
    let next_state = State {
        row: next_row,
        col: next_col,
        orientation: state.orientation,
    };
    match next {
        None => get_next(world, &next_state),
        Some(Tile::Open) => Some(next_state),
        Some(Tile::Wall) => None,
    }
}

/// Fixies initial (side relative) coordinates of the player after movement to the adjacent side of
/// the cube.
fn fix_initial_rc(new_state: &mut State, orientation: Orientation) {
    match orientation {
        Orientation::Up => new_state.row = 50 - 1,
        Orientation::Down => new_state.row = 0,
        Orientation::Left => new_state.col = 50 - 1,
        Orientation::Right => new_state.col = 0,
    }
}

/// Inverse side-relatie row or column coordinates based on the result of `get_adjacent_side()`
///
/// see `AdjacentSide::inv`
fn invese_rc(new_state: &mut State, orientation: Orientation) {
    match orientation {
        Orientation::Left | Orientation::Right => new_state.row = 50 - 1 - new_state.row,
        Orientation::Up | Orientation::Down => new_state.col = 50 - 1 - new_state.col,
    };
}

/// When there is a side wrap on a folded cube.
fn get_next_on_cube_wrap(world: &World, state: &State) -> Option<State> {
    // get adjacent side
    let adjacent_side = get_adjacent_side(state.row / 50, state.col / 50, state.orientation);

    // new state is initialized based on the result of `get_adjacent_side()` call with side-relative
    // coordinates (= relative row and column position within the given side)
    let mut new_state = State {
        row: state.row % 50,
        col: state.col % 50,
        orientation: adjacent_side.dir,
    };

    // If row or column should be inverted
    if adjacent_side.inv {
        invese_rc(&mut new_state, state.orientation)
    }

    // If row and column should be swapped
    if adjacent_side.swap_rc {
        swap(&mut new_state.row, &mut new_state.col);
    }

    fix_initial_rc(&mut new_state, adjacent_side.dir);

    // Convert side-relative coordinates to the absolute coordinates within unfolded 2D map
    new_state.row = new_state.row + 50 * adjacent_side.r;
    new_state.col = new_state.col + 50 * adjacent_side.c;

    let next = &world[new_state.row][new_state.col];
    match next {
        None => panic!("Somehow we got outside of the cube :("),
        Some(Tile::Open) => return Some(new_state),
        Some(Tile::Wall) => return None,
    }
}

/// When there is no side wrap
fn get_next_on_cube_no_wrap(
    world: &World,
    cur_state: &State,
    next_row: usize,
    next_col: usize,
) -> Option<State> {
    let next = &world[next_row][next_col];
    let next_state = State {
        row: next_row,
        col: next_col,
        orientation: cur_state.orientation,
    };
    match next {
        None => panic!("Somehow we got outside of the cube :("),
        Some(Tile::Open) => return Some(next_state),
        Some(Tile::Wall) => return None,
    }
}

/// Implementation of obtaining an adjacent tile when the movement is happening on folded cube in 3D.
/// When player reaches boundaries of the world, the movement continues on the adjacent cube side.
fn get_next_on_cube(world: &World, state: &State) -> Option<State> {
    match state.orientation {
        Orientation::Right => {
            if (state.col % 50) == 50 - 1 {
                get_next_on_cube_wrap(world, state)
            } else {
                get_next_on_cube_no_wrap(world, state, state.row, state.col + 1)
            }
        }
        Orientation::Left => {
            if (state.col % 50) == 0 {
                get_next_on_cube_wrap(world, state)
            } else {
                get_next_on_cube_no_wrap(world, state, state.row, state.col - 1)
            }
        }
        Orientation::Up => {
            if (state.row % 50) == 0 {
                get_next_on_cube_wrap(world, state)
            } else {
                get_next_on_cube_no_wrap(world, state, state.row - 1, state.col)
            }
        }
        Orientation::Down => {
            if (state.row % 50) == 50 - 1 {
                get_next_on_cube_wrap(world, state)
            } else {
                get_next_on_cube_no_wrap(world, state, state.row + 1, state.col)
            }
        }
    }
}

/// Function returns for given cube side adjacent side and other indicators needed in the implementation
/// of `get_next_on_cube_wrap()`
///
/// This function is not generic for any arbitrary unfolded cube, but only to the one given in my
/// puzzle input:
///
/// ```txt
///       +-----+-----+
///       |(0,1)|(0,2)|
///       +-----+-----+
///       |(1,1)|
/// +-----+-----+
/// |(2,0)|(2,1)|
/// +-----+-----+
/// |(3,0)|
/// +-----+
/// ```
///
/// For example if we are moving from the side `(0,1)` to the left, the resuling structure is:
/// - side row: 2
/// - side col: 0
/// - we will continue on that side oriented to the right
/// - row property should be switched (e.g from row 40 we should continue on row 10)
/// - row and column should not be switched
fn get_adjacent_side(side_row: usize, side_col: usize, orientation: Orientation) -> AdjacentSide {
    if side_row == 0 && side_col == 1 {
        match orientation {
            Orientation::Right => AdjacentSide {
                r: 0,
                c: 2,
                dir: Orientation::Right,
                inv: false,
                swap_rc: false,
            },
            Orientation::Down => AdjacentSide {
                r: 1,
                c: 1,
                dir: Orientation::Down,
                inv: false,
                swap_rc: false,
            },
            Orientation::Left => AdjacentSide {
                r: 2,
                c: 0,
                dir: Orientation::Right,
                inv: true,
                swap_rc: false,
            },
            Orientation::Up => AdjacentSide {
                r: 3,
                c: 0,
                dir: Orientation::Right,
                inv: false,
                swap_rc: true,
            },
        }
    } else if side_row == 0 && side_col == 2 {
        match orientation {
            Orientation::Right => AdjacentSide {
                r: 2,
                c: 1,
                dir: Orientation::Left,
                inv: true,
                swap_rc: false,
            },
            Orientation::Down => AdjacentSide {
                r: 1,
                c: 1,
                dir: Orientation::Left,
                inv: false,
                swap_rc: false,
            },
            Orientation::Left => AdjacentSide {
                r: 0,
                c: 1,
                dir: Orientation::Left,
                inv: false,
                swap_rc: false,
            },
            Orientation::Up => AdjacentSide {
                r: 3,
                c: 0,
                dir: Orientation::Up,
                inv: false,
                swap_rc: false,
            },
        }
    } else if side_row == 1 && side_col == 1 {
        match orientation {
            Orientation::Right => AdjacentSide {
                r: 0,
                c: 2,
                dir: Orientation::Up,
                inv: false,
                swap_rc: true,
            },
            Orientation::Down => AdjacentSide {
                r: 2,
                c: 1,
                dir: Orientation::Down,
                inv: false,
                swap_rc: false,
            },
            Orientation::Left => AdjacentSide {
                r: 2,
                c: 0,
                dir: Orientation::Down,
                inv: false,
                swap_rc: true,
            },
            Orientation::Up => AdjacentSide {
                r: 0,
                c: 1,
                dir: Orientation::Up,
                inv: false,
                swap_rc: false,
            },
        }
    } else if side_row == 2 && side_col == 0 {
        match orientation {
            Orientation::Right => AdjacentSide {
                r: 2,
                c: 1,
                dir: Orientation::Right,
                inv: false,
                swap_rc: false,
            },
            Orientation::Down => AdjacentSide {
                r: 3,
                c: 0,
                dir: Orientation::Down,
                inv: false,
                swap_rc: false,
            },
            Orientation::Left => AdjacentSide {
                r: 0,
                c: 1,
                dir: Orientation::Right,
                inv: true,
                swap_rc: false,
            },
            Orientation::Up => AdjacentSide {
                r: 1,
                c: 1,
                dir: Orientation::Right,
                inv: false,
                swap_rc: true,
            },
        }
    } else if side_row == 2 && side_col == 1 {
        match orientation {
            Orientation::Right => AdjacentSide {
                r: 0,
                c: 2,
                dir: Orientation::Left,
                inv: true,
                swap_rc: false,
            },
            Orientation::Down => AdjacentSide {
                r: 3,
                c: 0,
                dir: Orientation::Left,
                inv: false,
                swap_rc: true,
            },
            Orientation::Left => AdjacentSide {
                r: 2,
                c: 0,
                dir: Orientation::Left,
                inv: false,
                swap_rc: false,
            },
            Orientation::Up => AdjacentSide {
                r: 1,
                c: 1,
                dir: Orientation::Up,
                inv: false,
                swap_rc: false,
            },
        }
    } else if side_row == 3 && side_col == 0 {
        match orientation {
            Orientation::Right => AdjacentSide {
                r: 2,
                c: 1,
                dir: Orientation::Up,
                inv: false,
                swap_rc: true,
            },
            Orientation::Down => AdjacentSide {
                r: 0,
                c: 2,
                dir: Orientation::Down,
                inv: false,
                swap_rc: false,
            },
            Orientation::Left => AdjacentSide {
                r: 0,
                c: 1,
                dir: Orientation::Down,
                inv: false,
                swap_rc: true,
            },
            Orientation::Up => AdjacentSide {
                r: 2,
                c: 0,
                dir: Orientation::Up,
                inv: false,
                swap_rc: false,
            },
        }
    } else {
        panic!(
            "Movement from r:{}, c:{}, {:?} not implemented!",
            side_row, side_col, orientation
        )
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
    for instruction in instructions {
        state = state.apply_instruction(&world, &instruction, get_next_on_cube);
    }

    let result = (state.row + 1) * 1000 + 4 * (state.col + 1) + state.orientation as usize;
    println!("Result: {}", result)
}
