fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_content = std::fs::read_to_string(&args[1]).unwrap();
    let items = file_content.split('\n').map(|i| i.parse::<i32>());

    let mut totals: Vec<i32> = vec![];
    let mut sum = 0;
    for v in items {
        match v {
            Ok(v) => sum += v,
            _ => {
                totals.push(sum);
                sum = 0;
            }
        }
    }

    totals.sort();
    totals.reverse();
    let sum: i32 = totals[0..3].iter().sum();
    println!("{:?}", sum);
}
