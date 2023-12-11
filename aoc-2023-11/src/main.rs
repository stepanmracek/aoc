use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Coord>,
}

#[derive(Debug)]
struct UniverseParseError {}

impl FromStr for Universe {
    type Err = UniverseParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split('\n').filter(|line| !line.is_empty());
        let mut galaxies = Vec::new();

        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Coord { x, y });
                }
            }
        }
        Ok(Universe { galaxies })
    }
}

impl Universe {
    fn boundary(&self) -> Coord {
        let x = self.galaxies.iter().map(|g| g.x).max().unwrap();
        let y = self.galaxies.iter().map(|g| g.y).max().unwrap();
        Coord { x, y }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let boundary = self.boundary();
        for y in 0..=boundary.y {
            for x in 0..=boundary.x {
                let coord = Coord { x, y };
                if self.galaxies.contains(&coord) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn count_in_row(&self, y: usize) -> usize {
        self.galaxies.iter().filter(|g| g.y == y).count()
    }

    fn count_in_col(&self, x: usize) -> usize {
        self.galaxies.iter().filter(|g| g.x == x).count()
    }

    fn expand_rows(&mut self, grow_factor: usize) {
        let boundary = self.boundary();
        for y in (0..=boundary.y).rev() {
            if self.count_in_row(y) == 0 {
                self.galaxies
                    .iter_mut()
                    .filter(|g| g.y > y)
                    .for_each(|g| g.y += grow_factor - 1);
            }
        }
    }

    fn expand_cols(&mut self, grow_factor: usize) {
        let boundary = self.boundary();
        for x in (0..=boundary.x).rev() {
            if self.count_in_col(x) == 0 {
                self.galaxies
                    .iter_mut()
                    .filter(|g| g.x > x)
                    .for_each(|g| g.x += grow_factor - 1);
            }
        }
    }

    fn expand(&mut self, grow_factor: usize) {
        self.expand_rows(grow_factor);
        self.expand_cols(grow_factor);
    }

    fn galaxy_pairs(&self) -> Vec<(usize, usize)> {
        let n = self.galaxies.len();
        let mut result = vec![];
        for i in 0..n - 1 {
            for j in i + 1..n {
                result.push((i, j));
            }
        }
        result
    }

    fn distance(&self, pair: &(usize, usize)) -> usize {
        let first = &self.galaxies[pair.0];
        let second = &self.galaxies[pair.1];
        (first.x).abs_diff(second.x) + (first.y).abs_diff(second.y)
    }
}

fn main() {
    let file_content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut universe = file_content.parse::<Universe>().unwrap();
    universe.expand(1_000_000);
    let pairs = universe.galaxy_pairs();
    let result: usize = pairs.iter().map(|pair| universe.distance(pair)).sum();
    println!("{}", result);
}
