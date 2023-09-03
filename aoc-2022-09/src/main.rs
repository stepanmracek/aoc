use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Direction(i32, i32);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position(i32, i32);

impl Default for Position {
    fn default() -> Self {
        Position(0, 0)
    }
}

impl Position {
    fn apply(&self, direction: &Direction) -> Position {
        Position(self.0 + direction.0, self.1 + direction.1)
    }
}

fn move_tail(head: &Position, tail: &Position) -> Position {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    if dx.abs() <= 1 && dy.abs() <= 1 {
        // no need to adjust anything
        tail.clone()
    } else {
        // move tail towards head
        Position(tail.0 + dx.signum(), tail.1 + dy.signum())
    }
}

fn parse_line(line: &str) -> Vec<Direction> {
    let items: Vec<_> = line.split(' ').collect();
    let count = items[1].parse::<usize>().unwrap();
    let direction = match items[0] {
        "U" => Direction(0, 1),
        "D" => Direction(0, -1),
        "L" => Direction(-1, 0),
        "R" => Direction(1, 0),
        _ => panic!("Unknown symbol {}", items[0]),
    };
    vec![direction; count]
}

fn main() {
    let arg = std::env::args().skip(1).next().unwrap();
    let file_content = std::fs::read_to_string(&arg).unwrap();
    let instructions = file_content
        .split('\n')
        .filter(|s| !s.is_empty())
        .flat_map(|s| parse_line(s));

    let head_positions = instructions.scan(Position::default(), |position, direction| {
        *position = position.apply(&direction);
        Some(position.clone())
    });

    /*let tail_positions =
    head_positions.scan(Position::default(), |tail_position, head_position| {
        *tail_position = move_tail(&head_position, tail_position);
        Some(tail_position.clone())
    });*/

    let tail_positions = head_positions
        .scan(vec![Position::default(); 9], |rope, head_position| {
            let mut new_state: Vec<Position> = Vec::new();
            new_state.push(move_tail(&head_position, &rope[0]));
            rope.iter().skip(1).for_each(|tail| {
                let head = new_state.last().unwrap();
                new_state.push(move_tail(&head, tail));
            });
            *rope = new_state.clone();
            Some(new_state)
        })
        .map(|rope| rope.last().unwrap().clone());

    let tail_positions_count = tail_positions.collect::<HashSet<_>>().len();
    print!("{}", tail_positions_count);
}
