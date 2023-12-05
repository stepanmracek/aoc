use kdam::tqdm;
use std::{ops::Range, str::FromStr};

#[derive(Debug)]
struct Map {
    source_range: Range<usize>,
    dest_range: Range<usize>,
}

impl Map {
    fn map(&self, value: usize) -> Option<usize> {
        if self.source_range.contains(&value) {
            return Some(self.dest_range.start + (value - self.source_range.start));
        }
        None
    }
}

#[derive(Debug)]
struct MapParseError {}

impl FromStr for Map {
    type Err = MapParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<_> = s.split(' ').collect();
        let dest_start = items[0].parse::<usize>().map_err(|_| MapParseError {})?;
        let src_start = items[1].parse::<usize>().map_err(|_| MapParseError {})?;
        let count = items[2].parse::<usize>().map_err(|_| MapParseError {})?;
        Ok(Map {
            source_range: src_start..src_start + count,
            dest_range: dest_start..dest_start + count,
        })
    }
}

#[derive(Debug)]
struct Maps {
    name: String,
    maps: Vec<Map>,
}

impl Maps {
    fn map(&self, value: usize) -> usize {
        for map in self.maps.iter() {
            if let Some(dest) = map.map(value) {
                return dest;
            }
        }
        value
    }
}

fn seed_to_location(seed: usize, maps: &Vec<Maps>) -> usize {
    let mut result = seed;
    for m in maps {
        result = m.map(result);
    }
    result
}

impl FromStr for Maps {
    type Err = MapParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, maps) = s.split_once(":\n").ok_or(MapParseError {})?;
        let maps: Result<Vec<Map>, MapParseError> = maps
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse())
            .collect();

        Ok(Maps {
            name: name.to_string(),
            maps: maps?,
        })
    }
}

fn parse_seeds(s: &str) -> Vec<Range<usize>> {
    let mut result = Vec::new();
    let mut numbers = s.split(' ').filter_map(|s| s.parse::<usize>().ok());
    while let Some(start) = numbers.next() {
        let count: usize = numbers.next().unwrap();
        result.push(start..start + count);
    }
    result
}

fn main() {
    let file_content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let file_content: Vec<&str> = file_content.split("\n\n").collect();
    let seeds = file_content[0].split_once(": ").unwrap().1;
    let seeds = parse_seeds(seeds);
    let total: usize = seeds.iter().map(|r| r.end - r.start).sum();
    let all_seeds = seeds.iter().flat_map(|it| it.clone());

    let maps: Vec<Maps> = file_content[1..]
        .iter()
        .filter_map(|s| s.parse::<Maps>().ok())
        .collect();

    let result = tqdm!(all_seeds, total = total)
        .map(|seed| seed_to_location(seed, &maps))
        .min()
        .unwrap();
    println!("{}", result);
}
