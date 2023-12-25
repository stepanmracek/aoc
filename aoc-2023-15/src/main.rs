#[derive(Clone)]
struct Lens {
    box_name: String,
    focal_length: usize,
}

struct Remove {
    box_name: String,
}

enum Instruction {
    Add(Lens),
    Remove(Remove),
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.trim()
        .split(',')
        .map(|s| {
            if let Some(stripped) = s.strip_suffix('-') {
                Instruction::Remove(Remove {
                    box_name: stripped.to_string(),
                })
            } else {
                let (box_name, focal_length) = s.split_once('=').expect("'=' not found");
                Instruction::Add(Lens {
                    box_name: box_name.to_string(),
                    focal_length: focal_length.parse().expect("Can't parse focal length"),
                })
            }
        })
        .collect()
}

fn hash(s: &str) -> usize {
    let mut result = 0;
    for char in s.chars() {
        result += char as usize;
        result *= 17;
        result %= 256;
    }
    result
}

fn print_boxes(boxes: &[Vec<Lens>]) {
    for (i, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            print!("Box {}:", i);
            for lens in b {
                print!(" [{} {}]", lens.box_name, lens.focal_length);
            }
            println!();
        }
    }
}

fn focusing_power(boxes: &[Vec<Lens>]) -> usize {
    let mut ans = 0;
    for (box_num, b) in boxes.iter().enumerate() {
        for (lens_num, lens) in b.iter().enumerate() {
            ans += (box_num + 1) * (lens_num + 1) * lens.focal_length;
        }
    }
    ans
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Missing input file argument");
    let file_content = std::fs::read_to_string(path).expect("Can't read input file");
    let instructions = parse_instructions(&file_content);
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for instruction in instructions {
        match instruction {
            Instruction::Remove(remove) => {
                let i = hash(&remove.box_name);
                let old_box = &boxes[i];
                boxes[i] = old_box
                    .iter()
                    .filter(|b| b.box_name != remove.box_name)
                    .cloned()
                    .collect();
            }
            Instruction::Add(new_lens) => {
                let index = hash(&new_lens.box_name);
                let old_box = boxes.get_mut(index).unwrap();
                let lens = old_box.iter_mut().find(|l| l.box_name == new_lens.box_name);
                if let Some(lens) = lens {
                    lens.focal_length = new_lens.focal_length;
                } else {
                    old_box.push(new_lens);
                }
            }
        }
    }

    print_boxes(&boxes);
    println!("{}", focusing_power(&boxes));
}
