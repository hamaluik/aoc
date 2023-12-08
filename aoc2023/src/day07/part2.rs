use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Copy, Clone)]
enum Card {
    Joker,
    Number(u8),
    Queen,
    King,
    Ace,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Card::Joker, Card::Joker) => std::cmp::Ordering::Equal,
            (Card::Joker, _) => std::cmp::Ordering::Less,
            (_, Card::Joker) => std::cmp::Ordering::Greater,
            (Card::Number(a), Card::Number(b)) => a.cmp(b),
            (Card::Number(_), _) => std::cmp::Ordering::Less,
            (_, Card::Number(_)) => std::cmp::Ordering::Greater,
            (Card::Queen, Card::Queen) => std::cmp::Ordering::Equal,
            (Card::Queen, _) => std::cmp::Ordering::Less,
            (_, Card::Queen) => std::cmp::Ordering::Greater,
            (Card::King, Card::King) => std::cmp::Ordering::Equal,
            (Card::King, _) => std::cmp::Ordering::Less,
            (_, Card::King) => std::cmp::Ordering::Greater,
            (Card::Ace, Card::Ace) => std::cmp::Ordering::Equal,
        }
    }
}

impl Card {
    fn parse(c: char) -> Self {
        match c {
            'J' => Card::Joker,
            'T' => Card::Number(10),
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            n if n >= '2' && n <= '9' => Card::Number(n as u8 - '0' as u8),
            _ => panic!("invalid card {c}"),
        }
    }
}

type Cards = [Card; 5];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Copy, Clone, Ord)]
struct Hand {
    cards: Cards,
    hand_type: HandType,
    bid: usize,
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &card in &self.cards {
            match card {
                Card::Joker => write!(f, "J")?,
                Card::Number(n) if n < 10 => write!(f, "{}", n)?,
                Card::Number(10) => write!(f, "T")?,
                Card::Queen => write!(f, "Q")?,
                Card::King => write!(f, "K")?,
                Card::Ace => write!(f, "A")?,
                _ => panic!("invalid card"),
            }
        }
        write!(f, " ({:>3})", self.bid)
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self.hand_type {
            HandType::HighCard => "HighCard",
            HandType::OnePair => "OnePair",
            HandType::TwoPair => "TwoPair",
            HandType::ThreeOfAKind => "ThreeOfAKind",
            HandType::FullHouse => "FullHouse",
            HandType::FourOfAKind => "FourOfAKind",
            HandType::FiveOfAKind => "FiveOfAKind",
        };
        write!(f, "{self} ({kind:>12})")
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    match self.cards[i].cmp(&other.cards[i]) {
                        std::cmp::Ordering::Equal => {}
                        o => return Some(o),
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
            o => {
                Some(o)
            }
        }
    }
}

impl Hand {
    fn parse(input: &str, hashmap: &mut HashMap<Card, u8>) -> Self {
        let mut cards = [Card::Number(0); 5];
        let mut parts = input.split_whitespace();
        let mut card_chars = parts.next().unwrap().chars();
        for i in 0..5 {
            cards[i] = Card::parse(card_chars.next().expect("card char"));
        }
        let bid = parts.next().unwrap().parse().unwrap();
        let hand_type = Hand::hand_type(&cards, hashmap);
        Hand {
            cards,
            hand_type,
            bid,
        }
    }

    fn hand_type(cards: &Cards, hashmap: &mut HashMap<Card, u8>) -> HandType {
        hashmap.clear();
        for &card in cards {
            hashmap.entry(card).and_modify(|e| *e += 1).or_insert(1);
        }
        let joker_count = hashmap.remove(&Card::Joker).unwrap_or(0);
        let mut counts = hashmap.values().collect::<Vec<_>>();
        debug_assert!(counts.len() <= 5);
        counts.sort();

        if joker_count == 0 {
            match counts.as_slice() {
                [1, 1, 1, 1, 1] => HandType::HighCard,
                [1, 1, 1, 2] => HandType::OnePair,
                [1, 2, 2] => HandType::TwoPair,
                [1, 1, 3] => HandType::ThreeOfAKind,
                [2, 3] => HandType::FullHouse,
                [1, 4] => HandType::FourOfAKind,
                [5] => HandType::FiveOfAKind,
                _ => panic!("invalid hand"),
            }
        }
        else {
            match counts.len() {
                0 | 1 => HandType::FiveOfAKind,
                2 => {
                    if counts[0] == &1 {
                        HandType::FourOfAKind
                    }
                    else {
                        HandType::FullHouse
                    }
                }
                3 => {
                    if counts[0] == &1 {
                        HandType::ThreeOfAKind
                    }
                    else {
                        HandType::TwoPair
                    }
                }
                4 => HandType::OnePair,
                _ => panic!("invalid hand: {}; cards: {cards:?}", counts.len()),
            }
        }
    }
}

pub fn part2(input: &str) -> usize {
    let mut hands = input
        .par_lines()
        .map_with(HashMap::new(), |mut hashmap, line| {
            Hand::parse(line, &mut hashmap)
        })
        .collect::<Vec<_>>();
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn day07_part1_correct_ordering() {
        assert!(Card::Joker < Card::Number(2));
        assert!(Card::Number(2) < Card::Number(3));
        assert!(Card::Number(3) < Card::Number(4));
        assert!(Card::Number(4) < Card::Number(5));
        assert!(Card::Number(5) < Card::Number(6));
        assert!(Card::Number(6) < Card::Number(7));
        assert!(Card::Number(7) < Card::Number(8));
        assert!(Card::Number(8) < Card::Number(9));
        assert!(Card::Number(9) < Card::Number(10));
        assert!(Card::Number(10) < Card::Queen);
        assert!(Card::Queen < Card::King);
        assert!(Card::King < Card::Ace);

        assert!(HandType::HighCard < HandType::OnePair);
        assert!(HandType::OnePair < HandType::TwoPair);
        assert!(HandType::TwoPair < HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind < HandType::FullHouse);
        assert!(HandType::FullHouse < HandType::FourOfAKind);
        assert!(HandType::FourOfAKind < HandType::FiveOfAKind);
    }

    #[test]
    fn day07_part2_can_parse_hand() {
        let mut hashmap: HashMap<Card, u8> = HashMap::new();

        let hand = Hand::parse("32T3K 765", &mut hashmap);
        assert_eq!(
            hand,
            Hand {
                cards: [
                    Card::Number(3),
                    Card::Number(2),
                    Card::Number(10),
                    Card::Number(3),
                    Card::King
                ],
                hand_type: HandType::OnePair,
                bid: 765
            }
        );

        let hand = Hand::parse("QQQJA 483", &mut hashmap);
        assert_eq!(
            hand,
            Hand {
                cards: [Card::Queen, Card::Queen, Card::Queen, Card::Joker, Card::Ace],
                hand_type: HandType::FourOfAKind,
                bid: 483
            }
        );
    }

    #[test]
    fn day07_part2_can_order_hands() {
        let mut hashmap: HashMap<Card, u8> = HashMap::new();
        let hand1 = Hand::parse("32T3K 765", &mut hashmap);
        let hand2 = Hand::parse("T55J5 684", &mut hashmap);
        let hand3 = Hand::parse("KK677 28", &mut hashmap);
        let hand4 = Hand::parse("KTJJT 220", &mut hashmap);
        let hand5 = Hand::parse("QQQJA 483", &mut hashmap);
        let mut hands = vec![hand1, hand2, hand3, hand4, hand5];
        hands.sort();
        assert_eq!(hands, vec![hand1, hand3, hand2, hand5, hand4]);
    }

    #[test]
    fn day07_sample_part2() {
        assert_eq!(part2(SAMPLE), 5905);
    }
}



