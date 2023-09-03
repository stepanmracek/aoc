use itertools::Itertools;

struct MapIterator<'a> {
    r: i32,
    c: i32,
    dr: i32,
    dc: i32,
    map: &'a Vec<Vec<i32>>,
}

impl Iterator for MapIterator<'_> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.r < 0
            || self.c < 0
            || self.r as usize >= self.map.len()
            || self.c as usize >= self.map[0].len()
        {
            return None;
        }
        let v = self.map[self.r as usize][self.c as usize];
        self.r += self.dr;
        self.c += self.dc;
        return Some(v);
    }
}

fn is_visible(map_iterator: &mut MapIterator) -> bool {
    let start = map_iterator.next().unwrap();
    map_iterator.all(|v| v < start)
}

fn scenic_score(map_iterator: &mut MapIterator) -> usize {
    let start = map_iterator.next().unwrap();
    map_iterator
        .take_while_inclusive(|&v| v < start)
        .count()
}

fn main() {
    let arg = std::env::args().skip(1).next().unwrap();
    let file_content = std::fs::read_to_string(&arg).unwrap();

    let map: Vec<Vec<i32>> = file_content
        .split('\n')
        .filter(|&line| line.len() > 0)
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let rows = 1..map.len() - 1;
    let cols = 1..map[0].len() - 1;
    let coords = rows.cartesian_product(cols);
    let best = coords
        .map(|(r, c)| {
            scenic_score(&mut MapIterator {map: &map, r: r as i32, c: c as i32, dr: 1, dc: 0})
                * scenic_score(&mut MapIterator {map: &map, r: r as i32, c: c as i32, dr: -1, dc: 0})
                * scenic_score(&mut MapIterator {map: &map, r: r as i32, c: c as i32, dr: 0, dc: 1})
                * scenic_score(&mut MapIterator {map: &map, r: r as i32, c: c as i32, dr: 0, dc: -1})
        })
        .max();
    println!("{}", best.unwrap());
}
