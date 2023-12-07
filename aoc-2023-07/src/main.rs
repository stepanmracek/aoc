use core::cmp::Ordering;
use counter::Counter;

struct Hand {
    cards: Vec<u8>,
}

impl Hand {
    fn get_kind(&self) -> u8 {
        let counter: Counter<u8, usize> = Counter::from_iter(self.cards.iter().cloned());
        let mut occurences: Vec<usize> = counter.values().cloned().collect();
        occurences.sort();

        if occurences == vec![5] {
            return 6; // Five of a kind
        } else if occurences == vec![1, 4] {
            return 5; // Four of a kind
        } else if occurences == vec![2, 3] {
            return 4; // Full house
        } else if occurences == vec![1, 1, 3] {
            return 3; // Three of a kind
        } else if occurences == vec![1, 2, 2] {
            return 2; // Two pair
        } else if counter.len() == 5 {
            return 0; // High card
        } else {
            return 1; // One pair
        }
    }

    fn get_kind_using_joker(&self) -> u8 {
        // replace all jokers with card that has most occurences
        let other_cards: Counter<u8, usize> =
            Counter::from_iter(self.cards.iter().filter(|c| **c != 1).cloned());
        if other_cards.len() == 0 {
            return 6; // Five jokers of a kind
        }

        let most_common_card = other_cards.most_common()[0].0;
        let new_cards = self
            .cards
            .iter()
            .map(|&c| if c == 1 { most_common_card } else { c })
            .collect();

        let new_hand = Hand { cards: new_cards };
        return new_hand.get_kind();
    }

    fn compare_same_kind(&self, other: &Self) -> Ordering {
        for (self_card, other_card) in std::iter::zip(self.cards.iter(), other.cards.iter()) {
            if self_card != other_card {
                return self_card.cmp(other_card);
            }
        }
        Ordering::Equal
    }
}

struct Row {
    hand: Hand,
    bid: usize,
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_kind = self.hand.get_kind_using_joker();
        let other_kind = other.hand.get_kind_using_joker();
        if self_kind != other_kind {
            return self_kind.cmp(&other_kind);
        }
        self.hand.compare_same_kind(&other.hand)
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.hand.cards == other.hand.cards
    }
}

impl Eq for Row {}

fn parse_hand(s: &str) -> Hand {
    let cards = s
        .trim()
        .chars()
        .filter_map(|c| match c {
            'T' => Some(10),
            'J' => Some(1), // Joker
            'Q' => Some(12),
            'K' => Some(13),
            'A' => Some(14),
            _ => c.to_string().parse().ok(),
        })
        .collect();

    Hand { cards }
}

fn parse_row(row: &str) -> Option<Row> {
    let (hand, bid) = row.split_once(' ')?;
    let hand = parse_hand(hand);
    let bid = bid.parse::<usize>().ok()?;
    Some(Row { hand, bid })
}

fn main() {
    let file_content = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut rows: Vec<_> = file_content.split('\n').filter_map(parse_row).collect();
    rows.sort();

    let result: usize = std::iter::zip(1.., rows)
        .map(|(rank, row)| rank * row.bid)
        .sum();
    println!("{}", result);
}
