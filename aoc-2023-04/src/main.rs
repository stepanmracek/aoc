use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
struct Card {
    winning: HashSet<i32>,
    owned: HashSet<i32>,
}

#[derive(Debug)]
struct CardParseError {}

impl FromStr for Card {
    type Err = CardParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_card_id, numbers) = s.split_once(": ").ok_or(CardParseError {})?;
        let (winning, owned) = numbers.split_once(" | ").ok_or(CardParseError {})?;
        let winning: Result<HashSet<i32>, _> = winning
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|n| n.parse())
            .collect();
        let owned: Result<HashSet<i32>, _> = owned
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|n| n.parse())
            .collect();

        Ok(Card {
            owned: owned.map_err(|_| CardParseError {})?,
            winning: winning.map_err(|_| CardParseError {})?,
        })
    }
}

impl Card {
    fn points(&self) -> i32 {
        let len = self.intersection_len() as u32;
        if len == 0 {
            return 0;
        }
        2_i32.pow(len - 1)
    }

    fn intersection_len(&self) -> usize {
        self.owned.intersection(&self.winning).count()
    }
}

fn count_winning_scratchcards(
    index: usize,
    cards: &Vec<Card>,
    counter: &mut HashMap<usize, usize>,
) {
    if let Some(v) = counter.get_mut(&index) {
        *v += 1;
    } else {
        counter.insert(index, 1);
    }

    let intersection = cards[index].intersection_len();
    if intersection > 0 {
        for new_index in index + 1..=index + intersection {
            count_winning_scratchcards(new_index, cards, counter)
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(path).unwrap();
    let cards: Vec<Card> = file_content
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Card>().unwrap())
        .collect();
    let result: i32 = cards.iter().map(|card| card.points()).sum();
    println!("{:?}", result);

    let mut counter: HashMap<usize, usize> = HashMap::new();
    for (index, _) in cards.iter().enumerate() {
        count_winning_scratchcards(index, &cards, &mut counter)
    }

    let result: usize = counter.values().sum();
    println!("{:?}", result);
}
