fn modulo(a: i64, b: usize) -> usize {
    (((a % b as i64) + b as i64) % b as i64) as usize
}

fn shift(input: &mut Vec<(usize, i64)>, index: usize) {
    let s = input[index].1;
    if s == 0 {
        return;
    }
    let mut cur_index = index;
    let count = modulo(s.abs(), input.len() - 1);
    for _ in 0..count {
        let other = modulo(cur_index as i64 + s.signum(), input.len());
        input.swap(cur_index, other);
        cur_index = other;
    }
}

fn mix(input: &mut Vec<(usize, i64)>) {
    for orig_index in 0..input.len() {
        let index = input.iter().position(|&(i, _)| i == orig_index).unwrap();
        shift(input, index);
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let mut input: Vec<(usize, i64)> = std::fs::read_to_string(arg)
        .unwrap()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>().unwrap() * 811589153)
        .enumerate()
        .collect();

    for _ in 0..10 {
        mix(&mut input);
    }

    let index_of_zero = input.iter().position(|&(_, v)| v == 0).unwrap();
    let i1 = input[modulo(index_of_zero as i64 + 1000, input.len())].1;
    let i2 = input[modulo(index_of_zero as i64 + 2000, input.len())].1;
    let i3 = input[modulo(index_of_zero as i64 + 3000, input.len())].1;
    println!("{} + {} + {} = {}", i1, i2, i3, i1 + i2 + i3)
}

#[cfg(test)]
mod tests {
    use crate::shift;

    #[test]
    fn test_shift_right() {
        let mut input = vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];
        shift(&mut input, 2);
        assert_eq!(input, vec![(3, 3), (2, 2), (4, 4), (5, 5), (1, 1)]);

        let mut input = vec![(1, 1), (2, 2), (3, 8), (4, 4), (5, 5)];
        shift(&mut input, 2);
        assert_eq!(input, vec![(3, 8), (4, 4), (5, 5), (1, 1), (2, 2)]);

        let mut input = vec![(1, 1), (2, 2), (3, 13), (4, 4), (5, 5)];
        shift(&mut input, 2);
        assert_eq!(input, vec![(3, 13), (5, 5), (1, 1), (2, 2), (4, 4)]);
    }
}
