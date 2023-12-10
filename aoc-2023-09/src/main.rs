fn parse_line(s: &str) -> Vec<i32> {
    s.split(' ').filter_map(|v| v.parse().ok()).collect()
}

fn calc_diff(values: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for i in 1..values.len() {
        result.push(values[i] - values[i - 1]);
    }
    result
}

fn predict_next(values: &Vec<i32>) -> i32 {
    let diff = calc_diff(values);
    if values.iter().all(|&v| v == 0) {
        return 0;
    }

    predict_next(&diff) + values.last().unwrap()
}

fn predict_prev(values: &Vec<i32>) -> i32 {
    let diff = calc_diff(values);
    if values.iter().all(|&v| v == 0) {
        return 0;
    }

    values.first().unwrap() - predict_prev(&diff)
}

fn main() {
    let file_content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let lines: Vec<_> = file_content
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect();

    let result: i32 = lines.iter().map(|line| predict_next(line)).sum();
    println!("{}", result);

    let result: i32 = lines.iter().map(|line| predict_prev(line)).sum();
    println!("{}", result);
}
