type Point = (i64, i64);

struct Instruction {
    direction: Point,
    count: i64,
}

fn parse_instructions_part_1(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(|line| {
            let items: Vec<&str> = line.split(' ').collect();
            let direction = match items[0] {
                "U" => (-1, 0),
                "D" => (1, 0),
                "L" => (0, -1),
                "R" => (0, 1),
                _ => panic!("Unknown direction {}", items[0]),
            };
            let count = items[1].parse().expect("Can't parse count");
            Instruction { direction, count }
        })
        .collect()
}

fn parse_instructions_part_2(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(|line| {
            let (_, line) = line.split_once('#').expect("Can't parse line");
            let count = i64::from_str_radix(&line[0..5], 16).expect("Can't parse count");
            let direction = match &line.chars().nth(5) {
                Some('0') => (0, 1),
                Some('1') => (1, 0),
                Some('2') => (0, -1),
                Some('3') => (-1, 0),
                _ => panic!("Unknown direction"),
            };
            Instruction { direction, count }
        })
        .collect()
}

fn get_points(instructions: &[Instruction]) -> Vec<Point> {
    let mut result = vec![];

    let mut current = (0, 0);
    result.push(current);

    for instruction in instructions {
        current.0 += instruction.direction.0 * instruction.count;
        current.1 += instruction.direction.1 * instruction.count;
        result.push(current);
    }

    result
}

fn get_outer_points_count(instructions: &[Instruction]) -> i64 {
    instructions.iter().map(|i| i.count).sum()
}

fn inner_area(points: &[Point]) -> i64 {
    let n = points.len();
    let mut result: i64 = 0;
    for i in 0..points.len() {
        result += (points[i].0 * points[(i + 1) % n].1) - (points[i].1 * points[(i + 1) % n].0);
    }
    (result / 2).abs()
}

fn outer_area(instructions: &[Instruction]) -> i64 {
    get_outer_points_count(instructions) / 2 + 1
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(path).unwrap();
    for parse_func in [parse_instructions_part_1, parse_instructions_part_2] {
        let instructions = parse_func(&file_content);
        let points = get_points(&instructions);
        println!("{}", inner_area(&points) + outer_area(&instructions));
    }
}
