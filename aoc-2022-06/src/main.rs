use std::collections::HashSet;

fn main() {
    const N: usize = 14;
    let arg = std::env::args().skip(1).next().unwrap();
    let file_content: Vec<char> = std::fs::read_to_string(&arg).unwrap().chars().collect();

    let index = file_content[..]
        .windows(N)
        .enumerate()
        .find_map(|(i, window)| {
            let chunk: HashSet<char> = window.iter().cloned().collect();
            if chunk.len() >= N {
                Some(i + N)
            } else {
                None
            }
        })
        .unwrap();

    println!("{}", index);
}
