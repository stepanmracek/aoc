use regex::Regex;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Blueprint {
    ores_for_ore_robot: usize,
    ores_for_clay_robot: usize,
    ores_for_obsidian_robot: usize,
    clays_for_obsidian_robot: usize,
    ores_for_geode_robot: usize,
    obsidians_for_geode_robot: usize,
}

#[derive(Debug, Clone)]
struct State {
    ores: usize,
    clays: usize,
    obsidians: usize,
    geodes: usize,
}

impl Default for State {
    fn default() -> Self {
        State {
            ores: 0,
            clays: 0,
            obsidians: 0,
            geodes: 0,
        }
    }
}

fn harvest(robots: &Vec<Robot>, state: &mut State) {
    for robot in robots.iter() {
        match robot {
            Robot::Ore => state.ores += 1,
            Robot::Clay => state.clays += 1,
            Robot::Obsidian => state.obsidians += 1,
            Robot::Geode => state.geodes += 1,
        }
    }
}

fn can_create(robot: &Robot, state: &State, blueprint: &Blueprint) -> bool {
    match robot {
        Robot::Ore => state.ores >= blueprint.ores_for_ore_robot,
        Robot::Clay => state.ores >= blueprint.ores_for_clay_robot,
        Robot::Obsidian => {
            state.ores >= blueprint.ores_for_obsidian_robot
                && state.clays >= blueprint.clays_for_obsidian_robot
        }
        Robot::Geode => {
            state.ores >= blueprint.ores_for_geode_robot
                && state.obsidians >= blueprint.obsidians_for_geode_robot
        }
    }
}

fn assemble(state: &mut State, robot: &Robot, blueprint: &Blueprint) {
    match robot {
        Robot::Ore => state.ores -= blueprint.ores_for_ore_robot,
        Robot::Clay => state.ores -= blueprint.ores_for_clay_robot,
        Robot::Obsidian => {
            state.ores -= blueprint.ores_for_obsidian_robot;
            state.clays -= blueprint.clays_for_obsidian_robot;
        }
        Robot::Geode => {
            state.ores -= blueprint.ores_for_geode_robot;
            state.obsidians -= blueprint.obsidians_for_geode_robot;
        }
    };
}

/// based on current robots return what other robots might be assembled in future
fn get_strategy(robots: &Vec<Robot>) -> Vec<Robot> {
    let mut strategy = vec![];
    if robots.contains(&Robot::Obsidian) {
        strategy.push(Robot::Geode);
    }
    if robots.contains(&Robot::Clay) {
        strategy.push(Robot::Obsidian);
    }
    strategy.push(Robot::Clay);
    strategy.push(Robot::Ore);
    strategy
}

const MAX_DEPTH: usize = 24;

fn evaluate_blueprint(
    depth: usize,
    blueprint: &Blueprint,
    state: State,
    robots: Vec<Robot>,
) -> usize {
    let strategy = get_strategy(&robots);

    if depth == MAX_DEPTH {
        return state.geodes;
    }

    let mut max = 0;
    for next_robot in strategy {
        let mut next_state = state.clone();
        let mut next_depth = depth;
        let mut next_robots = robots.clone();
        while !can_create(&next_robot, &next_state, blueprint) {
            harvest(&robots, &mut next_state);
            next_depth += 1;
            if next_depth == MAX_DEPTH {
                break;
            }
        }

        if next_depth < MAX_DEPTH {
            harvest(&robots, &mut next_state);
            assemble(&mut next_state, &next_robot, blueprint);
            next_robots.push(next_robot);
            next_depth += 1;
        }

        let value = if next_depth < MAX_DEPTH {
            evaluate_blueprint(next_depth, blueprint, next_state, next_robots)
        } else {
            next_state.geodes
        };

        if value > max {
            max = value;
        }
    }

    max
}

#[derive(Debug)]
enum BlueprintParseError {
    ParseIntError(ParseIntError),
    RegexCaptureError,
}

impl Blueprint {
    fn parse_cost(cost: &str) -> Result<usize, BlueprintParseError> {
        cost.parse()
            .map_err(|e| BlueprintParseError::ParseIntError(e))
    }
}

impl FromStr for Blueprint {
    type Err = BlueprintParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Blueprint (?<num>\d+): Each ore robot costs (?<ores_for_ore_robot>\d+) ore. Each clay robot costs (?<ores_for_clay_robot>\d+) ore. Each obsidian robot costs (?<ores_for_obsidian_robot>\d+) ore and (?<clays_for_obsidian_robot>\d+) clay. Each geode robot costs (?<ores_for_geode_robot>\d+) ore and (?<obsidians_for_geode_robot>\d+) obsidian.").unwrap();
        let caps = re
            .captures(s)
            .ok_or(BlueprintParseError::RegexCaptureError)?;

        Result::Ok(Blueprint {
            ores_for_ore_robot: Blueprint::parse_cost(&caps["ores_for_ore_robot"])?,
            ores_for_clay_robot: Blueprint::parse_cost(&caps["ores_for_clay_robot"])?,
            ores_for_obsidian_robot: Blueprint::parse_cost(&caps["ores_for_obsidian_robot"])?,
            clays_for_obsidian_robot: Blueprint::parse_cost(&caps["clays_for_obsidian_robot"])?,
            ores_for_geode_robot: Blueprint::parse_cost(&caps["ores_for_geode_robot"])?,
            obsidians_for_geode_robot: Blueprint::parse_cost(&caps["obsidians_for_geode_robot"])?,
        })
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let blueprints: Vec<Blueprint> = std::fs::read_to_string(arg)
        .unwrap()
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse().ok())
        .collect();

    let mut result = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let state = State::default();
        let robots = vec![Robot::Ore];
        let value = evaluate_blueprint(0, blueprint, state, robots);
        println!("{}: {}", i + 1, value);
        result += (i + 1) * value;
    }
    println!("result: {}", result);
}
