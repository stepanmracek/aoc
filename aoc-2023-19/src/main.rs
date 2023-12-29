use std::{collections::HashMap, ops::RangeInclusive};

type Rating = HashMap<char, i64>;
type Rules = HashMap<String, Vec<Rule>>;
type StateSpace = HashMap<char, RangeInclusive<i64>>;

#[derive(Clone, PartialEq)]
enum Output {
    Accepted,
    Rejected,
}

enum Then {
    RuleName(String),
    Output(Output),
}

struct Condition {
    property: char,
    value: i64,
    then: Then,
}

enum Rule {
    Gt(Condition),
    Lt(Condition),
    Fallback(Then),
}

struct RangeDivide {
    result: RangeInclusive<i64>,
    complement: RangeInclusive<i64>,
}

fn parse_then(s: &str) -> Then {
    match s {
        "A" => Then::Output(Output::Accepted),
        "R" => Then::Output(Output::Rejected),
        _ => Then::RuleName(s.into()),
    }
}

fn parse_condition(s: &str, condition_delimiter: char) -> Condition {
    let (prop, val_then) = s.split_once(condition_delimiter).unwrap();
    let (val, then) = val_then.split_once(':').unwrap();
    Condition {
        property: prop.chars().next().unwrap(),
        value: val.parse().unwrap(),
        then: parse_then(then),
    }
}

fn parse_rule(s: &str) -> (String, Vec<Rule>) {
    let (rule_name, rules) = s[0..s.len() - 1].split_once('{').unwrap();
    let rules = rules
        .split(',')
        .map(|rule| {
            if rule.contains('<') {
                Rule::Lt(parse_condition(rule, '<'))
            } else if rule.contains('>') {
                Rule::Gt(parse_condition(rule, '>'))
            } else {
                Rule::Fallback(parse_then(rule))
            }
        })
        .collect::<Vec<_>>();
    (rule_name.into(), rules)
}

fn parse_rating(s: &str) -> Rating {
    let s = &s[1..s.len() - 1];
    s.split(',')
        .map(|s| {
            let (prop, value) = s.split_once('=').unwrap();
            (prop.chars().next().unwrap(), value.parse().unwrap())
        })
        .collect()
}

fn process_rating(rating: &Rating, rules: &Rules, rule_name: &String) -> Output {
    let then_to_output = |then: &Then| match then {
        Then::Output(output) => output.clone(),
        Then::RuleName(rule_name) => process_rating(rating, rules, rule_name),
    };

    let steps = &rules[rule_name];
    for rule in steps {
        match rule {
            Rule::Gt(condition) => {
                if rating[&condition.property] > condition.value {
                    return then_to_output(&condition.then);
                }
            }
            Rule::Lt(condition) => {
                if rating[&condition.property] < condition.value {
                    return then_to_output(&condition.then);
                }
            }
            Rule::Fallback(then) => return then_to_output(then),
        }
    }

    Output::Accepted
}

fn state_space_size(state_space: &StateSpace) -> usize {
    state_space
        .values()
        .map(|range| range.clone().count())
        .product()
}

fn apply_gt(r: &RangeInclusive<i64>, val: i64) -> RangeDivide {
    let new_start = (val + 1).max(*r.start());
    let result = new_start..=*r.end();
    let complement = *r.start()..=(new_start - 1).min(*r.end());
    RangeDivide { result, complement }
}

fn apply_lt(r: &RangeInclusive<i64>, val: i64) -> RangeDivide {
    let new_end = (val - 1).min(*r.end());
    let result = *r.start()..=new_end;
    let complement = (new_end + 1).max(*r.start())..=*r.end();
    RangeDivide { result, complement }
}

fn count_accepted_combinations(
    rules: &Rules,
    rule_name: &String,
    state_space: StateSpace,
) -> usize {
    let mut state_space = state_space;
    let mut result = 0;
    let set = &rules[rule_name];

    for rule in set {
        match rule {
            Rule::Gt(condition) => {
                let mut new_state_space = state_space.clone();
                let division = apply_gt(&state_space[&condition.property], condition.value);
                new_state_space.insert(condition.property, division.result);
                state_space.insert(condition.property, division.complement);
                match &condition.then {
                    Then::Output(Output::Accepted) => result += state_space_size(&new_state_space),
                    Then::Output(Output::Rejected) => {}
                    Then::RuleName(rule_name) => {
                        result += count_accepted_combinations(rules, rule_name, new_state_space);
                    }
                }
            }
            Rule::Lt(condition) => {
                let mut new_state_space = state_space.clone();
                let division = apply_lt(&state_space[&condition.property], condition.value);
                new_state_space.insert(condition.property, division.result);
                state_space.insert(condition.property, division.complement);
                match &condition.then {
                    Then::Output(Output::Accepted) => result += state_space_size(&new_state_space),
                    Then::Output(Output::Rejected) => {}
                    Then::RuleName(rule_name) => {
                        result += count_accepted_combinations(rules, rule_name, new_state_space);
                    }
                }
            }
            Rule::Fallback(Then::Output(Output::Accepted)) => {
                result += state_space_size(&state_space)
            }
            Rule::Fallback(Then::Output(Output::Rejected)) => {}
            Rule::Fallback(Then::RuleName(rule_name)) => {
                result += count_accepted_combinations(rules, rule_name, state_space.clone())
            }
        }
    }
    result
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(path).unwrap();
    let (rules, ratings) = file_content.split_once("\n\n").unwrap();
    let rules: Rules = rules.lines().map(parse_rule).collect();
    let ratings: Vec<Rating> = ratings.lines().map(parse_rating).collect();

    let result: i64 = ratings
        .iter()
        .filter(|rating| process_rating(rating, &rules, &"in".to_string()) == Output::Accepted)
        .map(|rating| rating.values().sum::<i64>())
        .sum();
    println!("Part 1: {result}");

    let state_space = HashMap::from([
        ('x', 1..=4000),
        ('m', 1..=4000),
        ('a', 1..=4000),
        ('s', 1..=4000),
    ]);
    let result = count_accepted_combinations(&rules, &"in".to_string(), state_space);
    println!("Part 2: {result}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_apply_gt() {
        let r = 600..=3400i64;
        let result = apply_gt(&r, 900);
        assert_eq!(result.result, 901..=3400i64);
        assert_eq!(result.complement, 600..=900i64);

        let result = apply_gt(&r, 599);
        assert_eq!(result.result, 600..=3400i64);
        assert!(result.complement.count() == 0);

        let result = apply_gt(&r, 600);
        assert_eq!(result.result, 601..=3400i64);
        assert_eq!(result.complement, 600..=600i64);

        let result = apply_gt(&r, 50);
        assert_eq!(result.result, 600..=3400i64);
        assert!(result.complement.is_empty());

        let result = apply_gt(&r, 3399);
        assert_eq!(result.result, 3400..=3400i64);
        assert_eq!(result.complement, 600..=3399i64);

        let result = apply_gt(&r, 3400);
        assert!(result.result.is_empty());
        assert_eq!(result.complement, 600..=3400i64);

        let result = apply_gt(&r, 3800);
        assert!(result.result.is_empty());
        assert_eq!(result.complement, 600..=3400i64);
    }

    #[test]
    fn test_apply_lt() {
        let r = 600..=3400i64;
        let result = apply_lt(&r, 900);
        assert_eq!(result.result, 600..=899i64);
        assert_eq!(result.complement, 900..=3400i64);

        let result = apply_lt(&r, 600);
        assert!(result.result.is_empty());
        assert_eq!(result.complement, 600..=3400i64);

        let result = apply_lt(&r, 601);
        assert_eq!(result.result, 600..=600i64);
        assert_eq!(result.complement, 601..=3400i64);

        let result = apply_lt(&r, 50);
        assert!(result.result.is_empty());
        assert_eq!(result.complement, 600..=3400i64);

        let result = apply_lt(&r, 3400);
        assert_eq!(result.result, 600..=3399i64);
        assert_eq!(result.complement, 3400..=3400);

        let result = apply_lt(&r, 3401);
        assert_eq!(result.result, 600..=3400i64);
        assert!(result.complement.is_empty());

        let result = apply_lt(&r, 3800);
        assert_eq!(result.result, 600..=3400i64);
        assert!(result.complement.is_empty());
    }
}
