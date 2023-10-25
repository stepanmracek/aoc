use regex::Regex;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
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
    robots: Vec<Robot>,
}

impl Default for State {
    fn default() -> Self {
        State {
            ores: 0,
            clays: 0,
            obsidians: 0,
            geodes: 0,
            robots: vec![Robot::Ore],
        }
    }
}

fn round(state: &State, action: Option<Robot>, blueprint: &Blueprint) -> State {
    let mut new_state = state.clone();
    for robot in new_state.robots.iter() {
        match robot {
            Robot::Ore => new_state.ores += 1,
            Robot::Clay => new_state.clays += 1,
            Robot::Obsidian => new_state.obsidians += 1,
            Robot::Geode => new_state.geodes += 1,
        }
    }
    if let Some(robot) = action {
        match robot {
            Robot::Ore => new_state.ores -= blueprint.ores_for_ore_robot,
            Robot::Clay => new_state.ores -= blueprint.ores_for_clay_robot,
            Robot::Obsidian => {
                new_state.ores -= blueprint.ores_for_obsidian_robot;
                new_state.clays -= blueprint.clays_for_obsidian_robot;
            }
            Robot::Geode => {
                new_state.ores -= blueprint.ores_for_geode_robot;
                new_state.obsidians -= blueprint.obsidians_for_geode_robot;
            }
        }
        new_state.robots.push(robot);
    }
    new_state
}

fn get_actions(state: &State, blueprint: &Blueprint) -> Vec<Option<Robot>> {
    let mut actions = vec![None];

    if state.ores >= blueprint.ores_for_ore_robot {
        actions.push(Some(Robot::Ore))
    }
    if state.ores >= blueprint.ores_for_clay_robot {
        actions.push(Some(Robot::Clay))
    }
    if state.ores >= blueprint.ores_for_obsidian_robot
        && state.clays >= blueprint.clays_for_obsidian_robot
    {
        actions.push(Some(Robot::Obsidian))
    }
    if state.ores >= blueprint.ores_for_geode_robot
        && state.obsidians >= blueprint.obsidians_for_geode_robot
    {
        actions.push(Some(Robot::Geode))
    }

    actions
}

fn evaluate_blueprint(depth: usize, blueprint: &Blueprint, state: &State) -> usize {
    //println!("{}", depth);
    if depth == 24 {
        return state.geodes;
    }

    get_actions(state, blueprint)
        .iter()
        .map(|action| {
            let new_state = round(state, action.clone(), blueprint);
            evaluate_blueprint(depth + 1, blueprint, &new_state)
        })
        .max()
        .unwrap()
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

    let blueprint = &blueprints[0];
    let state = State::default();
    let value = evaluate_blueprint(0, blueprint, &state);
    println!("{}", value)
    /*let mut state = State::default();
    let strategy: Vec<Option<Robot>> = vec![
        None,
        None,
        Some(Robot::Clay),
        None,
        Some(Robot::Clay),
        None,
        Some(Robot::Clay),
        None,
        None,
        None,
        Some(Robot::Obsidian),
        Some(Robot::Clay),
        None,
        None,
        Some(Robot::Obsidian),
        None,
        None,
        Some(Robot::Geode),
        None,
        None,
        Some(Robot::Geode),
        None,
        None,
        None,
    ];
    assert_eq!(strategy.len(), 24);
    for action in strategy {
        state = round(&state, action, blueprint);
        println!("{:?}", state);
        println!("-> {:?}", get_actions(&state, blueprint));
    }*/
}
