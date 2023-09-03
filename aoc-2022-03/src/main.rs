use std::collections::HashSet;

fn group_packs<T, I>(mut i: I) -> Vec<[T; 3]>
where
    I: Iterator<Item = T>,
{
    let mut groups: Vec<[T; 3]> = vec![];

    loop {
        let a = i.next();
        let b = i.next();
        let c = i.next();

        if a.is_none() || b.is_none() || c.is_none() {
            break;
        }

        groups.push([a.unwrap(), b.unwrap(), c.unwrap()]);
    }

    groups
}

fn item_type_to_priority(v: char) -> u32 {
    if v.is_lowercase() {
        (v as u32) - 96
    } else {
        (v as u32) - 64 + 26
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_content = std::fs::read_to_string(&args[1]).unwrap();

    let packs_iter = file_content
        .split('\n')
        .filter(|&s| s.len() > 0)
        .map(|s| -> HashSet<char> { HashSet::from_iter(s.chars()) });

    let groups: Vec<[HashSet<char>; 3]> = group_packs(packs_iter);

    let result: u32 = groups
        .iter()
        .filter_map(|[a, b, c]| {
            let i: HashSet<char> = HashSet::from_iter(a.intersection(b).cloned());
            i.intersection(c).cloned().next()
        })
        .map(item_type_to_priority)
        .sum();

    println!("{:?}", result);
}
