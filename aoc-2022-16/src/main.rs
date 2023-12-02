use kdam::tqdm;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const TIME: usize = 26;

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: i32,
    tunnels: Vec<&'a str>,
}

type Valves<'a> = Vec<Valve<'a>>;

type Distances<'a> = HashMap<(&'a str, &'a str), usize>;

impl<'a> Valve<'a> {
    fn from_str(s: &'a str) -> Self {
        let re = Regex::new(r"Valve (?P<name>[A-Z]{2}) has flow rate=(?P<flow_rate>\d{1,2}); tunnels? leads? to valves? (?P<tunnels>.*)").unwrap();
        let mat = re.captures(s).unwrap();

        Valve {
            name: mat.name("name").unwrap().as_str(),
            flow_rate: mat.name("flow_rate").unwrap().as_str().parse().unwrap(),
            tunnels: mat.name("tunnels").unwrap().as_str().split(", ").collect(),
        }
    }
}

fn check_symmetry(valves: &Valves) {
    for v in valves {
        for dest in v.tunnels.iter() {
            let dest_valve = valves.iter().find(|v| v.name == *dest).unwrap();
            let _symmetry = dest_valve.tunnels.iter().find(|&t| *t == v.name).unwrap();
        }
    }
}

fn get_min<'a>(n: &mut HashSet<&'a str>, d: &HashMap<&'a str, usize>) -> Option<&'a str> {
    let mut min_d: Option<usize> = None;
    let mut min_v: Option<&str> = None;

    for v in n.iter() {
        if d.contains_key(v) && (min_d.is_none() || d.get(v).unwrap() < &min_d.unwrap()) {
            min_d = Some(*d.get(v).unwrap());
            min_v = Some(v);
        }
    }

    if min_v.is_some() {
        n.remove(&min_v.unwrap());
    }

    return min_v;
}

fn dijkstra(valves: &Valves, start: &str, end: &str) -> Option<usize> {
    let mut n: HashSet<&str> = valves.iter().map(|v| v.name).collect();
    let mut d: HashMap<&str, usize> = HashMap::new();
    d.insert(start, 0);

    loop {
        if n.is_empty() {
            return None;
        }
        let u = get_min(&mut n, &d);
        if u.is_none() {
            return None;
        }

        if let Some(u) = u {
            if u == end {
                return Some(d.get(&u).unwrap().clone());
            }

            let neighbours = &valves.iter().find(|v| v.name == u).unwrap().tunnels;
            for v in neighbours {
                let alt = d.get(&u).unwrap() + 1;
                if !d.contains_key(v) || alt < *d.get(v).unwrap() {
                    d.insert(v, alt);
                }
            }
        }
    }
}

fn precompute_distances<'a>(valves: &'a Valves) -> Distances<'a> {
    let start = "AA";

    let mut distances: Distances = Distances::new();
    // 1. from start to each node having non-zero flow rate
    for dest in valves.iter().filter(|v| v.flow_rate > 0) {
        let end = dest.name;
        let d = dijkstra(valves, start, end).unwrap();
        distances.insert((start, end), d);
    }

    // from each non-zero node to other non-zero node
    for start in valves.iter().filter(|v| v.flow_rate > 0) {
        for end in valves.iter().filter(|v| v.flow_rate > 0) {
            if start.name == end.name {
                continue;
            }

            let d = dijkstra(valves, &start.name, &end.name).unwrap();
            distances.insert((start.name, end.name), d);
        }
    }
    distances
}

fn simulate<'a>(route: &Vec<&str>, distances: &Distances, valves: &Valves) -> i32 {
    let mut released_pressure = 0;
    let mut increase = 0;
    let mut time_left = TIME;
    let mut cur_valve = "AA";
    let mut opened_valves: Vec<&str> = vec![];
    let mut route_iter = route.iter();

    loop {
        let dest = route_iter.next();
        if let Some(dest) = dest {
            let from_to = (cur_valve, *dest);
            let d = distances.get(&from_to).unwrap();
            for _ in 0..=*d {
                released_pressure += increase;
                time_left -= 1;

                if time_left == 0 {
                    break;
                }
            }
            cur_valve = dest;
            increase += valves.iter().find(|&v| v.name == *dest).unwrap().flow_rate;
            opened_valves.push(dest);
        } else {
            released_pressure += increase;
            time_left -= 1;
        }

        if time_left == 0 {
            break;
        }
    }

    released_pressure
}

fn generate_paths<'a>(
    cur_path: Vec<&'a str>,
    cur_distance: usize,
    avail_nodes: HashSet<&'a str>,
    distances: &Distances,
) -> Vec<Vec<&'a str>> {
    let mut result = vec![];
    if cur_distance > TIME {
        panic!()
    }
    if !avail_nodes.is_empty() && cur_distance < TIME {
        let mut recursion = false;
        for n in &avail_nodes {
            let mut new_avail_nodes = avail_nodes.clone();
            new_avail_nodes.remove(n);

            let mut new_path = cur_path.clone();
            new_path.push(n);
            let key = if cur_path.is_empty() {
                ("AA", *n)
            } else {
                (*cur_path.last().unwrap(), *n)
            };
            let new_distance = cur_distance + distances.get(&key).unwrap() + 1;
            if new_distance <= TIME {
                recursion = true;
                result.extend(generate_paths(
                    new_path,
                    new_distance,
                    new_avail_nodes,
                    distances,
                ));
            }
        }
        if !recursion {
            result.push(cur_path);
        }
    } else {
        result.push(cur_path);
    }
    result
}

fn generate_paths_for_two<'a>(
    first_path: Vec<&'a str>,
    second_path: Vec<&'a str>,
    first_distance: usize,
    second_distance: usize,
    avail_nodes: HashSet<&'a str>,
    distances: &Distances,
) -> (Vec<Vec<&'a str>>,Vec<Vec<&'a str>>) {
    let mut result = (vec![], vec![]);
    if first_distance > TIME || second_distance > TIME {
        panic!()
    }

    if !avail_nodes.is_empty() {
        for avail_node in &avail_nodes {

        }

    }



    result
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(arg).unwrap();

    let valves: Valves = file_content
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|s| Valve::from_str(s))
        .collect();

    check_symmetry(&valves);

    let distances = precompute_distances(&valves);

    let non_zero_valves: Vec<_> = valves
        .iter()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.name)
        .collect();

    let cur_path: Vec<&str> = vec![];
    let avail_nodes = HashSet::from_iter(non_zero_valves.iter().map(|&n| n));
    let paths = generate_paths(cur_path, 0, avail_nodes, &distances);

    let result: i32 = tqdm!(paths.iter())
        .inspect(|path| println!("{:?}", path))
        .map(|path| simulate(path, &distances, &valves))
        .max()
        .unwrap();

    println!("{}", result);
}
