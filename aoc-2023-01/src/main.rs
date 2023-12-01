fn extract_numbers(s: &str) -> usize {
    let digits: Vec<usize> = s
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|v| v as usize)
        .collect();
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    return first * 10 + last;
}

fn get_numbers() -> Vec<(usize, String)> {
    let mut digits: Vec<_> = (0..=9).map(|d| (d, d.to_string())).collect();
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|s| s.to_string())
    .enumerate();

    digits.extend(words);
    digits
}

fn find_from_left(s: &str) -> usize {
    return get_numbers()
        .iter()
        .filter_map(|(num, pat)| s.find(pat).and_then(|index| Some((index, *num))))
        .min_by_key(|(index, _num)| *index)
        .unwrap()
        .1;
}

fn find_from_right(s: &str) -> usize {
    return get_numbers()
        .iter()
        .filter_map(|(num, pat)| s.rfind(pat).and_then(|index| Some((index, *num))))
        .max_by_key(|(index, _num)| *index)
        .unwrap()
        .1;
}

fn extract_numbers_2(s: &str) -> usize {
    let first = find_from_left(s);
    let last = find_from_right(s);
    return first * 10 + last;
}

fn main() {
    let file_content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let lines = file_content.split("\n").filter(|&line| !line.is_empty());
    let result: usize = lines.map(|s| extract_numbers_2(s)).sum();
    println!("{}", result);
}
