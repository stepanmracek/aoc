use std::{ops::Range, str::FromStr, time::Instant};

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

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<_> = s.split(' ').collect();
        let dest_start = items[0]
            .parse::<usize>()
            .map_err(|_| "Destination range start parse error")?;
        let src_start = items[1]
            .parse::<usize>()
            .map_err(|_| "Source range start parse error")?;
        let count = items[2]
            .parse::<usize>()
            .map_err(|_| "Range count parse error")?;
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

    fn map_range(&self, range: &Range<usize>) -> Vec<Range<usize>> {
        let mut ans = vec![];

        if self.maps.len() == 0 {
            return ans;
        }

        // if range starts before first source range, add that part of input range into result
        let first_source_range = &self.maps[0].source_range;
        if range.start < first_source_range.start {
            let end = range.end.clamp(range.start, first_source_range.start);
            ans.push(range.start..end);
        }

        // Go through all maps and intersect their source range with input range
        for map in self.maps.iter() {
            if range.end > map.source_range.start && range.start < map.source_range.end {
                // there is an intersection, map it to the destination range
                let mut start = range.start;
                if start < map.source_range.start {
                    start = map.source_range.start;
                }

                let mut end = range.end;
                if end > map.source_range.end {
                    end = map.source_range.end;
                }

                let offset = start - map.source_range.start;
                let count = end - start;
                let dest_range =
                    map.dest_range.start + offset..map.dest_range.start + offset + count;
                ans.push(dest_range);
            }
        }

        // if range ends after last source range, add that part of the input range into result
        let last_source_range = &self.maps[self.maps.len() - 1].source_range;
        if range.end > last_source_range.end {
            let start = range.start.clamp(last_source_range.end, range.end);
            ans.push(start..range.end);
        }
        return ans;
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
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, maps) = s.split_once(":\n").ok_or_else(|| "Maps parse error")?;
        let maps: Result<Vec<Map>, String> = maps.lines().map(|line| line.parse()).collect();
        let mut maps = maps?;
        maps.sort_by_key(|m| m.source_range.start);

        // fill "holes" (missing ranges), so the range is continuous from start of the first range
        // to the end of the last range
        let mut missing = vec![];
        for i in 0..maps.len() - 1 {
            let current_end = maps[i].source_range.end;
            let next_start = maps[i + 1].source_range.start;
            if current_end != next_start {
                println!("{name}: adding missing range {current_end}..{next_start}");
                missing.push(Map {
                    source_range: current_end..next_start,
                    dest_range: current_end..next_start,
                });
            }
        }
        maps.extend(missing.into_iter());
        maps.sort_by_key(|m| m.source_range.start);

        Ok(Maps {
            name: name.to_string(),
            maps,
        })
    }
}

fn parse_seed_ranges(s: &str) -> Vec<Range<usize>> {
    let mut result = Vec::new();
    let mut numbers = s.split(' ').filter_map(|s| s.parse::<usize>().ok());
    while let Some(start) = numbers.next() {
        if let Some(count) = numbers.next() {
            result.push(start..start + count);
        }
    }
    result
}

fn find_min(input_range: &Range<usize>, maps: &Vec<Maps>, depth: usize) -> usize {
    let ranges = maps[depth].map_range(input_range);
    if depth == maps.len() - 1 {
        ranges.iter().map(|r| r.start).min().unwrap()
    } else {
        ranges
            .iter()
            .map(|r| find_min(r, maps, depth + 1))
            .min()
            .unwrap()
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(path).unwrap();
    let file_content: Vec<&str> = file_content.split("\n\n").collect();
    let seeds_str = file_content[0].split_once(": ").unwrap().1;
    let seeds = seeds_str
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let maps: Vec<Maps> = file_content[1..]
        .iter()
        .filter_map(|s| s.parse::<Maps>().ok())
        .collect();

    for m in maps.iter() {
        println!("{}", m.name);
        for m in m.maps.iter() {
            println!("  {:?}", m);
        }
    }

    println!("Seeds: {:?}", seeds);
    let result = seeds
        .iter()
        .map(|seed| seed_to_location(*seed, &maps))
        .min()
        .unwrap();
    println!("Part 1 result: {}", result);

    let seed_ranges = parse_seed_ranges(seeds_str);
    println!("Seed ranges: {:?}", seed_ranges);

    let stamp = Instant::now();
    let result = seed_ranges
        .iter()
        .map(|range| find_min(range, &maps, 0))
        .min()
        .unwrap();
    let elapsed = Instant::now() - stamp;
    println!("Part 2 result: {} (took {:?})", result, elapsed);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_map_range() {
        let maps = Maps {
            name: String::from("input"),
            maps: vec![
                Map {
                    source_range: 50..100,
                    dest_range: 1050..1100,
                },
                Map {
                    source_range: 100..150,
                    dest_range: 100..150,
                },
                Map {
                    source_range: 150..200,
                    dest_range: 3050..3100,
                },
            ],
        };

        assert_eq!(maps.map_range(&(10..20)), [(10..20)]);
        assert_eq!(maps.map_range(&(10..50)), [(10..50)]);
        assert_eq!(maps.map_range(&(10..60)), [(10..50), (1050..1060)]);
        assert_eq!(
            maps.map_range(&(10..160)),
            [(10..50), (1050..1100), (100..150), (3050..3060)]
        );
        assert_eq!(maps.map_range(&(150..250)), [(3050..3100), (200..250)]);
    }
}
