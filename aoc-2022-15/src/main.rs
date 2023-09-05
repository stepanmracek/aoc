use kdam::tqdm;
use std::str::FromStr;

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

fn intersect(sensor_beacon: &SensorBeacon, y: i32) -> Option<std::ops::RangeInclusive<i32>> {
    let distance = manhattan_distance(&sensor_beacon.sensor, &sensor_beacon.beacon);
    if !(sensor_beacon.sensor.y - distance..=sensor_beacon.sensor.y + distance).contains(&y) {
        None
    } else {
        let dy = y.abs_diff(sensor_beacon.sensor.y) as i32;
        Some(sensor_beacon.sensor.x - distance + dy..=sensor_beacon.sensor.x + distance - dy)
    }
}

fn process_row(
    y: i32,
    sensors_and_beacons: &Vec<SensorBeacon>,
    max_x: i32,
) -> Option<Coord> {
    let mut candidates: Vec<bool> = vec![true; max_x as usize + 1];
    for sb in sensors_and_beacons {
        if let Some(range) = intersect(&sb, y) {
            for x in range.filter(|&x| x >= 0 && x <= max_x) {
                candidates[x as usize] = false;
            }
        }
    }

    for x in sensors_and_beacons.iter().filter_map(|sb| {
        if sb.beacon.y == y && sb.beacon.x >= 0 && sb.beacon.x <= max_x {
            Some(sb.beacon.x)
        } else {
            None
        }
    }) {
        candidates[x as usize] = false;
    }

    let x_pos = candidates
        .iter()
        .enumerate()
        .find(|(_, &x)| x == true)
        .map(|(i, _)| i as i32);

    if let Some(x) = x_pos {
        Some(Coord { x, y })
    } else {
        None
    }
}

fn process_range(from: i32, to: i32, tqdm_pos: u16) {
    let arg = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();
    let sensors_and_beacons: Vec<SensorBeacon> = file_content
        .split('\n')
        .filter_map(|line| line.parse().ok())
        .collect();

    for y in tqdm!(from..to, position = tqdm_pos) {
        if let Some(coord) = process_row(y, &sensors_and_beacons, 4_000_000) {
            println!(
                "({},{}) -> {}",
                coord.x,
                coord.y,
                coord.x * 4_000_000 + coord.y
            );
        }
    }
}

fn main() {
    let handles = [
        std::thread::spawn(|| process_range(0, 500_000, 0)),
        std::thread::spawn(|| process_range(500_000, 1_000_000, 1)),
        std::thread::spawn(|| process_range(1_000_000, 1_500_000, 2)),
        std::thread::spawn(|| process_range(1_500_000, 2_000_000, 3)),
        std::thread::spawn(|| process_range(2_000_000, 2_500_000, 4)),
        std::thread::spawn(|| process_range(2_500_000, 3_000_000, 5)),
        std::thread::spawn(|| process_range(3_000_000, 3_500_000, 6)),
        std::thread::spawn(|| process_range(3_500_000, 4_000_001, 7)),
    ];

    for h in handles {
        h.join().unwrap();
    }
}
