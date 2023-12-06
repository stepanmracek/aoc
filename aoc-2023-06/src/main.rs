fn parse_numbers(s: &str) -> Vec<usize> {
    s.split(' ').filter_map(|s| s.parse().ok()).collect()
}

fn parse_number(s: &str) -> usize {
    s.split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap()
}

fn parse_input(s: &str) -> Vec<(usize, usize)> {
    let mut lines = s.split('\n').filter(|line| !line.is_empty());
    let time = parse_number(lines.next().unwrap());
    let distance = parse_number(lines.next().unwrap());
    vec![(time, distance)]
}

fn simulate_race(race_time: usize, record_distance: usize) -> usize {
    (1..race_time - 1)
        .map(|button_time| {
            let resulting_speed = button_time;
            let remaining_time = race_time - button_time;
            let distance = resulting_speed * remaining_time;
            distance
        })
        .filter(|&distance| distance >= record_distance)
        .count()
}

fn main() {
    let file_content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let input = parse_input(&file_content);

    let result = input
        .iter()
        .map(|&(race_time, record_distance)| simulate_race(race_time, record_distance))
        .reduce(|acc, e| acc * e);
    println!("{:?}", result);
}
