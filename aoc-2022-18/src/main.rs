use std::str::FromStr;

#[derive(Debug)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

struct ParseCubeError;

impl FromStr for Cube {
    type Err = ParseCubeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<usize> = s.trim().split(',').filter_map(|i| i.parse().ok()).collect();
        if items.len() != 3 {
            Err(ParseCubeError)
        } else {
            Ok(Self {
                x: items[0],
                y: items[1],
                z: items[2],
            })
        }
    }
}

fn shared_faces_x(cubes: &[Cube]) -> usize {
    let mut count = 0;
    let min = cubes.iter().min_by_key(|cube| cube.x).unwrap().x;
    let max = cubes.iter().max_by_key(|cube| cube.x).unwrap().x;
    for x in min..max {
        // all cubes having given x coordinate
        for cube in cubes.iter().filter(|cube| cube.x == x) {
            // all cubes having x one greater and same y and z
            count += cubes
                .iter()
                .filter(|other| other.x == cube.x + 1 && other.y == cube.y && other.z == cube.z)
                .count()
        }
    }
    count
}

fn shared_faces_y(cubes: &[Cube]) -> usize {
    let mut count = 0;
    let min = cubes.iter().min_by_key(|cube| cube.y).unwrap().y;
    let max = cubes.iter().max_by_key(|cube| cube.y).unwrap().y;
    for y in min..max {
        // all cubes having given y coordinate
        for cube in cubes.iter().filter(|cube| cube.y == y) {
            // all cubes having y one greater and same x and z
            count += cubes
                .iter()
                .filter(|other| other.y == cube.y + 1 && other.x == cube.x && other.z == cube.z)
                .count()
        }
    }
    count
}

fn shared_faces_z(cubes: &[Cube]) -> usize {
    let mut count = 0;
    let min = cubes.iter().min_by_key(|cube| cube.z).unwrap().z;
    let max = cubes.iter().max_by_key(|cube| cube.z).unwrap().z;
    for z in min..max {
        // all cubes having given z coordinate
        for cube in cubes.iter().filter(|cube| cube.z == z) {
            // all cubes having z one greater and same x and y
            count += cubes
                .iter()
                .filter(|other| other.z == cube.z + 1 && other.x == cube.x && other.y == cube.y)
                .count()
        }
    }
    count
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let cubes: Vec<Cube> = std::fs::read_to_string(arg)
        .unwrap()
        .trim()
        .split('\n')
        .filter_map(|line| line.parse().ok())
        .collect();

    let shared_faces = shared_faces_x(&cubes[..])
        + shared_faces_y(&cubes[..])
        + shared_faces_z(&cubes[..]);

    println!("{}", cubes.len() * 6 - 2 * shared_faces);
}
