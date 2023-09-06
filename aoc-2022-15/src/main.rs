use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

struct CoordParseError;

impl FromStr for Coord {
    type Err = CoordParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(", ").ok_or(CoordParseError)?;
        let x = x_str[2..].parse().map_err(|_| CoordParseError)?;
        let y = y_str[2..].parse().map_err(|_| CoordParseError)?;
        Ok(Coord { x, y })
    }
}

#[derive(Debug)]
struct SensorBeacon {
    sensor: Coord,
    beacon: Coord,
}

struct SensorBeaconParseError;

impl FromStr for SensorBeacon {
    type Err = SensorBeaconParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor_str, beacon_str) = s.split_once(": ").ok_or(SensorBeaconParseError)?;
        let sensor = sensor_str["Sensor at ".len()..]
            .parse()
            .map_err(|_| SensorBeaconParseError)?;
        let beacon = beacon_str["closest beacon is at ".len()..]
            .parse()
            .map_err(|_| SensorBeaconParseError)?;
        Ok(SensorBeacon { sensor, beacon })
    }
}

fn manhattan_distance(first: &Coord, second: &Coord) -> i32 {
    (first.x.abs_diff(second.x) + first.y.abs_diff(second.y)) as i32
}

fn intersect(sensor_beacon: &SensorBeacon, y: i32) -> Option<RangeInclusive<i32>> {
    let distance = manhattan_distance(&sensor_beacon.sensor, &sensor_beacon.beacon);
    if !(sensor_beacon.sensor.y - distance..=sensor_beacon.sensor.y + distance).contains(&y) {
        None
    } else {
        let dy = y.abs_diff(sensor_beacon.sensor.y) as i32;
        Some(sensor_beacon.sensor.x - distance + dy..=sensor_beacon.sensor.x + distance - dy)
    }
}

fn sort_ranges(ranges: &mut Vec<RangeInclusive<i32>>) {
    ranges.sort_by(|a, b| {
        if a.start() < b.start() {
            Ordering::Less
        } else if a.start() > b.start() {
            Ordering::Greater
        } else {
            if a.end() < b.end() {
                Ordering::Less
            } else if a.end() > b.end() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    });
}

fn merge_ranges(ranges: &Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    let mut merged = Vec::new();
    let mut cur_start = *ranges[0].start();
    let mut max_end = *ranges[0].end();
    for rng in ranges.iter().skip(1) {
        if *rng.start() <= max_end + 1 && *rng.end() > max_end {
            max_end = *rng.end();
        } else if *rng.start() > max_end + 1 {
            merged.push(cur_start..=max_end);
            cur_start = *rng.start();
            max_end = *rng.end();
        }
    }
    merged.push(cur_start..=max_end);
    merged
}

fn process_row(y: i32, sensors_and_beacons: &Vec<SensorBeacon>, max_x: i32) -> Option<Coord> {
    let mut intersections: Vec<_> = sensors_and_beacons
        .iter()
        .filter_map(|sb| intersect(sb, y))
        .collect();

    if intersections.is_empty() {
        return None;
    }

    sort_ranges(&mut intersections);
    let merged = merge_ranges(&intersections);

    let mut candidates: HashSet<i32> = HashSet::new();
    if merged[0].start() > &0 {
        candidates.extend(0..=*merged[0].start() - 1);
    }
    if merged.last().unwrap().end() < &max_x {
        candidates.extend(*merged[0].end() + 1..max_x);
    }

    for (prev, next) in merged.iter().tuple_windows() {
        if prev.end() + 1 < *next.start() {
            candidates.extend(*prev.end() + 1..=*next.start() - 1)
        }
    }

    for x in sensors_and_beacons.iter().filter_map(|sb| {
        if sb.beacon.y == y && sb.beacon.x >= 0 && sb.beacon.x <= max_x {
            Some(sb.beacon.x)
        } else {
            None
        }
    }) {
        candidates.remove(&x);
    }

    if candidates.len() > 0 {
        Some(Coord {
            x: candidates.iter().next().unwrap().clone(),
            y,
        })
    } else if candidates.is_empty() {
        None
    } else {
        panic!("More than 1 candidate found!")
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();
    let sensors_and_beacons: Vec<SensorBeacon> = file_content
        .split('\n')
        .filter_map(|line| line.parse().ok())
        .collect();

    for y in 0..=4_000_000 {
        if let Some(coord) = process_row(y, &sensors_and_beacons, 4_000_000) {
            println!(
                "({},{}) -> {}",
                coord.x,
                coord.y,
                coord.x as i64 * 4_000_000 + coord.y as i64
            );
            break;
        }
    }
}
