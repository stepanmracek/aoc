use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

impl Cube {
    fn neighbours(&self) -> Vec<Cube> {
        let mut result = vec![
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ];
        if self.x > 1 {
            result.push(Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            });
        }
        if self.y > 1 {
            result.push(Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            });
        }
        if self.x > 1 {
            result.push(Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            });
        }
        result
    }
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

fn count_shared_faces(cubes: &[Cube]) -> usize {
    shared_faces_x(cubes) + shared_faces_y(cubes) + shared_faces_z(cubes)
}

fn calc_total_area(cubes: &[Cube]) -> usize {
    let shared_faces = count_shared_faces(&cubes[..]);
    cubes.len() * 6 - 2 * shared_faces
}

fn get_neighbours(
    cube: &Cube,
    other_cubes: &HashSet<Cube>,
    range_x: &RangeInclusive<usize>,
    range_y: &RangeInclusive<usize>,
    range_z: &RangeInclusive<usize>,
) -> Vec<Cube> {
    cube.neighbours()
        .iter()
        .filter(|c| {
            !other_cubes.contains(c)
                && range_x.contains(&c.x)
                && range_y.contains(&c.y)
                && range_z.contains(&c.z)
        })
        .cloned()
        .collect()
}

fn flood_fill_recursive(
    seed: Cube,
    other_cubes: &mut HashSet<Cube>,
    range_x: &RangeInclusive<usize>,
    range_y: &RangeInclusive<usize>,
    range_z: &RangeInclusive<usize>,
) {
    let neighbours = get_neighbours(&seed, other_cubes, range_x, range_y, range_z);
    println!("{:?} -> {:?}", seed, neighbours);
    other_cubes.insert(seed);
    for neighbour in neighbours {
        flood_fill_recursive(neighbour, other_cubes, range_x, range_y, range_z)
    }
}

fn flood_fill(cubes: &[Cube]) -> HashSet<Cube> {
    let range_x = cubes.iter().min_by_key(|cube| cube.x).unwrap().x
        ..=cubes.iter().max_by_key(|cube| cube.x).unwrap().x;
    let range_y = cubes.iter().min_by_key(|cube| cube.y).unwrap().y
        ..=cubes.iter().max_by_key(|cube| cube.y).unwrap().y;
    let range_z: RangeInclusive<usize> = cubes.iter().min_by_key(|cube| cube.z).unwrap().z
        ..=cubes.iter().max_by_key(|cube| cube.z).unwrap().z;
    println!("x: {:?}; y: {:?}; z: {:?}", range_x, range_y, range_z);

    let mut other_cubes: HashSet<Cube> = cubes.iter().cloned().collect();
    println!("before: {}", other_cubes.len());
    flood_fill_recursive(
        Cube { x: 0, y: 0, z: 0 },
        &mut other_cubes,
        &range_x,
        &range_y,
        &range_z,
    );
    other_cubes
}

fn calc_outer_area(cubes: &[Cube]) -> usize {
    println!("{}", flood_fill(cubes).len());
    0
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let cubes: Vec<Cube> = std::fs::read_to_string(arg)
        .unwrap()
        .trim()
        .split('\n')
        .filter_map(|line| line.parse().ok())
        .collect();

    println!("total area: {}", calc_total_area(&cubes[..]));
    println!("outer area: {}", calc_outer_area(&cubes[..]));
}
