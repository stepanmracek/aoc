type Snafu = Vec<i64>;

fn parse_snafu(s: &str) -> Snafu {
    s.chars()
        .map(|char| match char {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            c => panic!("Unknown SNAFU symbol {}", c),
        })
        .collect()
}

fn snafu_to_decimal(snafu: &Snafu) -> i64 {
    snafu
        .iter()
        .rev()
        .enumerate()
        .map(|(exp, value)| 5_i64.pow(exp as u32) * value)
        .sum()
}

fn decimal_to_snafu(d: i64) -> Snafu {
    let mut snafu = vec![];
    let mut value = d;
    loop {
        let mut remainder = value % 5;
        let mut borrow = false;

        // if remainder is 3 or 4, we have to subtract instead
        // and borrow 5 for next round
        if remainder > 2 {
            borrow = true;
            remainder = remainder - 5;
        }

        snafu.push(remainder);
        value = value / 5;
        if borrow {
            value += 1;
        }
        if value == 0 {
            break;
        }
    }
    snafu.reverse();
    snafu
}

fn format_snafu(snafu: &Snafu) -> String{
    snafu.iter().map(|v| match v {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        c => panic!("Unknown SNAFU symbol {}", c),
    }).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let sum: i64 = std::fs::read_to_string(&args[1])
        .unwrap()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| parse_snafu(line))
        .map(|snafu_number| snafu_to_decimal(&snafu_number))
        .sum();

    println!("{}", format_snafu(&decimal_to_snafu(sum)));
}
