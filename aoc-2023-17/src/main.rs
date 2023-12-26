use inline_colorization::{bg_green, bg_reset, color_black, color_reset};
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashMap};

type Grid = Vec<Vec<u32>>;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    r: usize,
    c: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    coord: Coord,
    direction: (i32, i32),
    count: usize,
}

fn parse_grid(s: &str) -> Grid {
    let mut result = vec![];

    for line in s.lines() {
        let row: Vec<u32> = line
            .chars()
            .filter_map(|s| s.to_string().parse().ok())
            .collect();
        result.push(row)
    }

    result
}

fn get_neighbours(grid: &Grid, u: &Coord, directions: &[(i32, i32)]) -> Vec<Option<Coord>> {
    let mut ans = vec![];
    let rows_count = grid.len() as i32;
    let cols_count = grid[0].len() as i32;
    for (dr, dc) in directions.iter() {
        let r = u.r as i32 + dr;
        let c = u.c as i32 + dc;
        if r >= 0 && c >= 0 && r < rows_count && c < cols_count {
            ans.push(Some(Coord {
                r: r as usize,
                c: c as usize,
            }));
        } else {
            ans.push(None);
        }
    }
    ans
}

fn get_next_directions(current_direction: &(i32, i32), count: usize) -> Vec<(i32, i32)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter(|d| {
            let is_opposite = d == &&(-current_direction.0, -current_direction.1);
            let is_max = count >= 10 && d == &current_direction;
            let is_below_min = count > 0 && count < 4 && d != &current_direction;
            if is_opposite || is_max || is_below_min {
                return false;
            }
            true
        })
        .cloned()
        .collect()
}

fn dijkstra(grid: &Grid, start: Coord, end: Coord) -> Result<(u32, Vec<Coord>), &'static str> {
    // priority queue of nodes to explore
    let mut q = PriorityQueue::new();

    // known best distances to given coord
    let mut d: HashMap<Node, u32> = HashMap::new();

    // reverse path
    let mut prev: HashMap<Node, Node> = HashMap::new();

    let start_node = Node {
        coord: start,
        direction: (0, 0),
        count: 0,
    };

    // Because you already start in the top-left block, you don't incur that block's heat loss
    // unless you leave that block and then return to it.
    d.insert(start_node, 0);
    q.push(start_node, Reverse(0));
    while let Some((u, _)) = q.pop() {
        if u.coord == end {
            let mut path = vec![u.coord];
            let mut n = Some(&u);
            loop {
                n = prev.get(n.unwrap());
                if let Some(n) = n {
                    path.push(n.coord);
                } else {
                    break;
                }
            }
            path.reverse();
            return Ok((d[&u], path));
        }

        let directions = get_next_directions(&u.direction, u.count);
        let neighbours = get_neighbours(grid, &u.coord, &directions);

        let candidate_nodes: Vec<Node> = std::iter::zip(directions, neighbours)
            .filter_map(|(direction, coord)| {
                if let Some(coord) = coord {
                    let count = if direction == u.direction {
                        u.count + 1
                    } else {
                        1
                    };
                    Some(Node {
                        coord,
                        direction,
                        count,
                    })
                } else {
                    None
                }
            })
            .collect();

        for v in candidate_nodes {
            let alt = d[&u] + grid[v.coord.r][v.coord.c];
            if !d.contains_key(&v) || alt < d[&v] {
                d.insert(v, alt);
                prev.insert(v, u);
                q.push(v, Reverse(alt));
            }
        }
    }
    Err("Shortest path not found!")
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(path).unwrap();
    let grid = parse_grid(&file_content);

    let (cost, path) = dijkstra(
        &grid,
        Coord { r: 0, c: 0 },
        Coord {
            r: grid.len() - 1,
            c: grid[0].len() - 1,
        },
    )
    .unwrap();

    for (r, row) in grid.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            if path.contains(&Coord { r, c }) {
                print!("{color_black}{bg_green}{val}{bg_reset}{color_reset}");
            } else {
                print!("{val}");
            };
        }
        println!();
    }

    println!("{:?}", cost);
}
