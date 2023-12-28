use std::collections::HashMap;

type Rating = HashMap<String, i32>;
type Rules = HashMap<String, Vec<Rule>>;

#[derive(Debug, Clone, PartialEq)]
enum Output {
    Accepted,
    Rejected,
}

#[derive(Debug)]
enum Then {
    RuleName(String),
    Output(Output),
}

#[derive(Debug)]
struct Condition {
    property: String,
    value: i32,
    then: Then,
}

#[derive(Debug)]
enum Rule {
    Gt(Condition),
    Lt(Condition),
    Fallback(Then),
}

fn parse_then(s: &str) -> Then {
    match s {
        "A" => Then::Output(Output::Accepted),
        "R" => Then::Output(Output::Rejected),
        _ => Then::RuleName(s.into()),
    }
}

fn parse_rule(s: &str) -> (String, Vec<Rule>) {
    let (rule_name, rules) = s[0..s.len() - 1].split_once('{').unwrap();
    let rules = rules
        .split(',')
        .map(|rule| {
            if rule == "A" || rule == "R" {
                Rule::Fallback(parse_then(rule))
            } else if rule.contains('<') {
                let (prop, val_then) = rule.split_once('<').unwrap();
                let (val, then) = val_then.split_once(':').unwrap();
                Rule::Lt(Condition {
                    property: prop.into(),
                    value: val.parse().unwrap(),
                    then: parse_then(then),
                })
            } else if rule.contains('>') {
                let (prop, val_then) = rule.split_once('>').unwrap();
                let (val, then) = val_then.split_once(':').unwrap();
                Rule::Gt(Condition {
                    property: prop.into(),
                    value: val.parse().unwrap(),
                    then: parse_then(then),
                })
            } else {
                Rule::Fallback(Then::RuleName(rule.into()))
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
            (prop.into(), value.parse().unwrap())
        })
        .collect()
}

fn process_rating(rating: &Rating, rules: &Rules, rule_name: &String) -> Output {
    let steps = &rules[rule_name];
    for rule in steps {
        match rule {
            Rule::Gt(condition) => {
                if rating[&condition.property] > condition.value {
                    return match &condition.then {
                        Then::Output(output) => output.clone(),
                        Then::RuleName(rule_name) => process_rating(rating, rules, &rule_name),
                    };
                }
            }
            Rule::Lt(condition) => {
                if rating[&condition.property] < condition.value {
                    return match &condition.then {
                        Then::Output(output) => output.clone(),
                        Then::RuleName(rule_name) => process_rating(rating, rules, &rule_name),
                    };
                }
            }
            Rule::Fallback(fallback) => {
                return match &fallback {
                    Then::Output(output) => output.clone(),
                    Then::RuleName(rule_name) => process_rating(rating, rules, &rule_name),
                };
            }
        }
    }

    Output::Accepted
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(path).unwrap();
    let (rules, ratings) = file_content.split_once("\n\n").unwrap();
    let rules: Rules = rules.lines().map(parse_rule).collect();
    let ratings: Vec<Rating> = ratings.lines().map(parse_rating).collect();

    for (rule_name, r) in rules.iter() {
        println!("{rule_name} {r:?}");
    }

    for rating in ratings.iter() {
        println!(
            "{rating:?} -> {:?}",
            process_rating(&rating, &rules, &"in".to_string())
        );
    }

    let result: i32 = ratings
        .iter()
        .filter(|rating| process_rating(&rating, &rules, &"in".to_string()) == Output::Accepted)
        .map(|rating| rating.values().sum::<i32>())
        .sum();

    println!("{result}");
}
