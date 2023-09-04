use std::cmp::Ordering;

use itertools::{EitherOrBoth, Itertools};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum Packet {
    Number(i32),
    Array(Vec<Packet>),
}

struct Pair {
    left: Vec<Packet>,
    right: Vec<Packet>,
}

fn compare(left: &Vec<Packet>, right: &Vec<Packet>) -> Option<bool> {
    for pair in left.iter().zip_longest(right.iter()) {
        match pair {
            // Compare the first value of each list, then the second value, and so on.
            EitherOrBoth::Both(left, right) => {
                match (left, right) {
                    // If both values are integers, the lower integer should come first.
                    (Packet::Number(left), Packet::Number(right)) => {
                        if left < right {
                            return Some(true);
                        } else if left > right {
                            return Some(false);
                        }
                        // The inputs are the same integer; continue checking the next part of the input.
                    }
                    // If both values are lists, compare the first value of each list, then the second value, and so on.
                    (Packet::Array(left), Packet::Array(right)) => {
                        let comparison_result = compare(left, right);
                        if comparison_result.is_some() {
                            return comparison_result;
                        }
                    }
                    // If exactly one value is an integer, convert the integer to a list which contains
                    // that integer as its only value, then retry the comparison.
                    (Packet::Number(left), Packet::Array(right)) => {
                        let left = vec![Packet::Number(*left)];
                        let comparison_result = compare(&left, right);
                        if comparison_result.is_some() {
                            return comparison_result;
                        }
                    }
                    (Packet::Array(left), Packet::Number(right)) => {
                        let right = vec![Packet::Number(*right)];
                        let comparison_result = compare(left, &right);
                        if comparison_result.is_some() {
                            return comparison_result;
                        }
                    }
                }
            }
            // If the right list runs out of items first, the inputs are not in the right order.
            EitherOrBoth::Left(_) => {
                return Some(false);
            }
            // If the left list runs out of items first, the inputs are in the right order.
            EitherOrBoth::Right(_) => {
                return Some(true);
            }
        }
    }
    return None;
}

/*fn main() {
    let arg = std::env::args().skip(1).next().unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();

    let pairs = file_content.split("\n\n").filter_map(|p| {
        p.split_once('\n').and_then(|(left, right)| {
            Some(Pair {
                left: serde_json::from_str::<Vec<Packet>>(left).unwrap(),
                right: serde_json::from_str::<Vec<Packet>>(right).unwrap(),
            })
        })
    });

    let result: usize = pairs
        .enumerate()
        .filter_map(|(i, pair)| {
            compare(&pair.left, &pair.right).and_then(|r| if r { Some(i + 1) } else { None })
        })
        .sum();
    println!("{:?}", result);
}*/

fn main() {
    let arg = std::env::args().skip(1).next().unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();

    let mut packets: Vec<_> = file_content
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str::<Vec<Packet>>(line).ok())
        .collect();

    let sep1 = vec![Packet::Array(vec![Packet::Number(2)])];
    let sep2 = vec![Packet::Array(vec![Packet::Number(6)])];

    packets.push(sep1.clone());
    packets.push(sep2.clone());

    packets.sort_by(|first, second| match compare(first, second) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    let pos1 = packets
        .iter()
        .find_position(|&p| compare(p, &sep1).is_none())
        .and_then(|(i, _)| Some(i + 1))
        .unwrap();

    let pos2 = packets
        .iter()
        .find_position(|&p| compare(p, &sep2).is_none())
        .and_then(|(i, _)| Some(i + 1))
        .unwrap();

    for p in packets {
        println!("{:?}", p);
    }

    println!("{} * {} = {}", pos1, pos2, pos1 * pos2);
}
