use std::str::FromStr;

#[derive(Debug)]
struct Range {
    from: u32,
    to: u32,
}

#[derive(Debug)]
struct RangeParseError;

impl Range {
    fn contains(&self, other: &Range) -> bool {
        other.from >= self.from && other.to <= self.to
    }

    fn overlap(&self, other: &Range) -> bool {
        return (self.to >= other.from && self.from <= other.from)
            || (other.from <= self.to && other.to >= self.from);
    }
}

impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').ok_or(RangeParseError)?;
        let from = from.parse::<u32>().map_err(|_| RangeParseError)?;
        let to = to.parse::<u32>().map_err(|_| RangeParseError)?;
        Ok(Range { from, to })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_content = std::fs::read_to_string(&args[1]).unwrap();

    let result = file_content
        .split('\n')
        .filter(|s| (*s).len() > 0)
        .filter_map(|row| row.split_once(','))
        .map(|(first, second)| {
            (
                first.parse::<Range>().unwrap(),
                second.parse::<Range>().unwrap(),
            )
        })
        .filter(|(first, second)| {
            //first.contains(second) || second.contains(first)
            first.overlap(second)
        })
        .count();

    /*for v in result {
        println!("{:?}", v)
    }*/
    println!("{}", result)
}
