type Pattern = Vec<Vec<char>>;

#[derive(Debug)]
struct Symmetry {
    vertical: Option<usize>,
    horizontal: Option<usize>,
}

fn parse_pattern(s: &str) -> Pattern {
    let mut result = vec![];
    for line in s.lines().filter(|l| !l.is_empty()) {
        let row = line.chars().collect::<Vec<char>>();
        result.push(row);
    }
    result
}

fn print_pattern(p: &Pattern) {
    for row in p {
        println!("{}", row.iter().collect::<String>());
    }
}

fn check_vertical_symmetry(p: &Pattern, row: usize) -> bool {
    for shift in 0.. {
        let upper_row_index: i32 = row as i32 - shift as i32;
        let lower_row_index = row + shift + 1;

        if upper_row_index < 0 || lower_row_index >= p.len() {
            break;
        }

        if p[upper_row_index as usize] != p[lower_row_index] {
            return false;
        }
    }
    true
}

fn vertical_symmetry(p: &Pattern) -> Vec<usize> {
    let mut result = vec![];
    for row in 0..p.len() - 1 {
        if check_vertical_symmetry(p, row) {
            result.push(row + 1);
        }
    }
    result
}

fn get_col(p: &Pattern, col: usize) -> Vec<char> {
    p.iter().map(|row| row[col]).collect()
}

fn check_horizontal_symmetry(p: &Pattern, col: usize) -> bool {
    for shift in 0.. {
        let left_col_index: i32 = col as i32 - shift as i32;
        let right_col_index = col + shift + 1;

        if left_col_index < 0 || right_col_index >= p[0].len() {
            break;
        }

        if get_col(p, left_col_index as usize) != get_col(p, right_col_index) {
            return false;
        }
    }
    true
}

fn horizontal_symmetry(p: &Pattern) -> Vec<usize> {
    let mut result = vec![];
    for col in 0..p[0].len() - 1 {
        if check_horizontal_symmetry(p, col) {
            result.push(col + 1);
        }
    }
    result
}

fn get_symmetry(p: &Pattern) -> Symmetry {
    let old_h = horizontal_symmetry(p);
    let old_v = vertical_symmetry(p);
    for r in 0..p.len() {
        for c in 0..p[0].len() {
            let mut candidate = p.clone();

            let original = p[r][c];
            if original == '.' {
                candidate[r][c] = '#';
            } else if original == '#' {
                candidate[r][c] = '.';
            } else {
                panic!()
            }

            let new_h = horizontal_symmetry(&candidate);
            let new_v = vertical_symmetry(&candidate);

            for h in new_h {
                if !old_h.contains(&h) {
                    return Symmetry {
                        horizontal: Some(h),
                        vertical: None,
                    };
                }
            }
            for v in new_v {
                if !old_v.contains(&v) {
                    return Symmetry {
                        horizontal: None,
                        vertical: Some(v),
                    };
                }
            }
        }
    }
    panic!("No symmetry found!");
}

fn main() {
    let path = std::env::args().nth(1).expect("Missing file path argument");
    let file_content = std::fs::read_to_string(path).expect("Can't read file");

    let world = file_content
        .split("\n\n")
        .map(parse_pattern)
        .collect::<Vec<Pattern>>();

    let mut result = 0;
    for p in world.iter() {
        print_pattern(p);
        let symmetry = get_symmetry(p);
        println!("{:?}", symmetry);

        if let Some(vertical) = symmetry.vertical {
            result += 100 * vertical;
        }
        if let Some(horizontal) = symmetry.horizontal {
            result += horizontal;
        }
    }
    println!("result: {}", result);
}
