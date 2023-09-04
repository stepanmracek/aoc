use itertools::Itertools;
use kdam::tqdm;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type Coord = (usize, usize);

#[derive(Debug)]
struct Map {
    heights: Vec<Vec<u8>>,
    start: Coord,
    end: Coord,
    connections: HashSet<(Coord, Coord)>,
}

#[derive(Debug)]
struct MapParseError;

impl FromStr for Map {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let heights: Vec<Vec<u8>> = s
            .split('\n')
            .filter(|l| !l.is_empty())
            .enumerate()
            .map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .map(|(col, c)| {
                        if c == 'S' {
                            start = (row, col);
                            return 0;
                        } else if c == 'E' {
                            end = (row, col);
                            return ('z' as u8) - ('a' as u8);
                        }
                        (c as u8) - ('a' as u8)
                    })
                    .collect()
            })
            .collect();

        let mut connections: HashSet<(Coord, Coord)> = HashSet::new();

        for row in 0..heights.len() {
            for col in 0..heights[0].len() {
                if row > 0 && ((heights[row - 1][col] as i32 - heights[row][col] as i32) <= 1) {
                    connections.insert(((row, col), (row - 1, col)));
                }
                if row < heights.len() - 1
                    && ((heights[row + 1][col] as i32 - heights[row][col] as i32) <= 1)
                {
                    connections.insert(((row, col), (row + 1, col)));
                }
                if col > 0 && ((heights[row][col - 1] as i32 - heights[row][col] as i32) <= 1) {
                    connections.insert(((row, col), (row, col - 1)));
                }
                if col < heights[0].len() - 1
                    && ((heights[row][col + 1] as i32 - heights[row][col] as i32) <= 1)
                {
                    connections.insert(((row, col), (row, col + 1)));
                }
            }
        }

        Ok(Map {
            heights,
            start,
            end,
            connections,
        })
    }
}

impl Map {
    fn get_min(&self, n: &mut HashSet<Coord>, d: &HashMap<Coord, usize>) -> Option<Coord> {
        let mut min_d: Option<usize> = None;
        let mut min_v: Option<Coord> = None;

        for v in n.iter() {
            if d.contains_key(v) && (min_d.is_none() || d.get(v).unwrap() < &min_d.unwrap()) {
                min_d = Some(*d.get(v).unwrap());
                min_v = Some(*v);
            }
        }

        if min_v.is_some() {
            n.remove(&min_v.unwrap());
        }

        return min_v;
    }

    fn get_neighbours(&self, u: &Coord) -> Vec<Coord> {
        let mut candidates: Vec<Coord> = vec![(u.0 + 1, u.1), (u.0, u.1 + 1)];
        if u.0 > 0 {
            candidates.push((u.0 - 1, u.1))
        }
        if u.1 > 0 {
            candidates.push((u.0, u.1 - 1))
        }

        candidates
            .iter()
            .filter(|&c| self.connections.contains(&(*u, *c)))
            .cloned()
            .collect()
    }

    fn dijkstra(&self, start: &Coord) -> Result<usize, &str> {
        let rows = 0..self.heights.len();
        let cols = 0..self.heights[0].len();
        let mut n: HashSet<Coord> = rows.cartesian_product(cols).collect();
        let mut d: HashMap<Coord, usize> = HashMap::new();
        d.insert(*start, 0);

        loop {
            if n.is_empty() {
                return Err("Path not found!");
            }
            let u = self.get_min(&mut n, &d);
            if u.is_none() {
                return Err("Best node not found!");
            }

            if let Some(u) = u {
                if u == self.end {
                    return Ok(d.get(&u).unwrap().clone());
                }

                let neighbours = self.get_neighbours(&u);
                for v in neighbours {
                    let alt = d.get(&u).unwrap() + 1;
                    if !d.contains_key(&v) || alt < *d.get(&v).unwrap() {
                        d.insert(v, alt);
                    }
                }
            }
        }
    }
}

fn main() {
    let arg = std::env::args().skip(1).next().unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();
    let map = file_content.parse::<Map>().unwrap();

    let candidate_starts = (0..map.heights.len())
        .cartesian_product(0..map.heights[0].len())
        .filter(|(r, c)| map.heights[*r][*c] == 0)
        .collect_vec();

    let result = tqdm!(candidate_starts.iter())
        .filter_map(|candidate| map.dijkstra(&candidate).ok())
        .min();
    println!("{:?}", result);
}
