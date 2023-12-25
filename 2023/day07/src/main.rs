use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;

use counter::Counter;

mod counter;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let hands = parse_hands(&input);
    part1(&hands);
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            if let [cards, bid] = line.split_whitespace().collect::<Vec<&str>>()[..] {
                let cards: Vec<Card> = cards.chars().map(|c| Card::try_from(c).unwrap()).collect();
                let bid: u64 = bid.parse().unwrap();
                Hand::new(cards, bid)
            } else {
                panic!("unexpected input");
            }
        })
        .collect()
}

#[derive(Ord, Hash, Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_uppercase()[..] {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::T),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(String::from("unknown card")),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        format!("{value}").parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        assert_eq!("4".parse::<Card>().unwrap(), Card::Four);
        assert_eq!("Q".parse::<Card>().unwrap(), Card::Queen);
        assert_eq!("a".parse::<Card>().unwrap(), Card::Ace);
        assert!("Đ".parse::<Card>().is_err());
    }

    #[test]
    fn test_card_order() {
        assert_eq!(Card::Ace, Card::Ace);
        assert!(Card::Ace >= Card::Ace);
        assert!(Card::Ace <= Card::Ace);
        assert!(Card::Three > Card::Two);
        assert!(Card::Ace > Card::Two);
        assert!(Card::Queen < Card::Ace);
    }
}

#[derive(Debug, Copy, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

#[derive(PartialOrd, Ord, Debug)]
enum HandType {
    HighCard(Card),
    OnePair(Card),
    TwoPair(Card, Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    FiveOfAKind(Card),
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        use HandType::*;
        match (self, other) {
            (HighCard(_), HighCard(_)) => true,
            (OnePair(_), OnePair(_)) => true,
            (TwoPair(_, _), TwoPair(_, _)) => true,
            (ThreeOfAKind(_), ThreeOfAKind(_)) => true,
            (FullHouse(_, _), FullHouse(_, _)) => true,
            (FourOfAKind(_), FourOfAKind(_)) => true,
            (FiveOfAKind(_), FiveOfAKind(_)) => true,
            _ => false,
        }
    }
}

impl Eq for HandType {}

impl Hand {
    fn new(cards: Vec<Card>, bid: u64) -> Hand {
        Hand {
            cards: cards.try_into().unwrap(),
            bid,
        }
    }

    fn get_type(&self) -> HandType {
        let counter = Counter::from_iter(self.cards.iter().copied());
        let mut counts = counter.into_iter().collect::<Vec<_>>();
        counts.sort_by_key(|(card, count)| (*count, *card));
        if let [(card, 5)] = counts[..] {
            return HandType::FiveOfAKind(card);
        }
        if let [_, (card, 4)] = counts[..] {
            return HandType::FourOfAKind(card);
        }
        if let [(card2, 2), (card1, 3)] = counts[..] {
            return HandType::FullHouse(card1, card2);
        }
        if let [(_, 1), (_, 1), (card1, 3)] = counts[..] {
            return HandType::ThreeOfAKind(card1);
        }
        if let [(_, 1), (card2, 2), (card1, 2)] = counts[..] {
            return HandType::TwoPair(card1, card2);
        }
        if let [.., (card1, 2)] = counts[..] {
            return HandType::OnePair(card1);
        }
        HandType::HighCard(self.cards.iter().max().unwrap().clone())
    }
}

fn part1(hands: &Vec<Hand>) {
    let mut hands: Vec<Hand> = hands.iter().copied().collect();
    hands.sort_by(|hand1, hand2| {
        let type1 = hand1.get_type();
        let type2 = hand2.get_type();
        if type1 == type2 {
            for (card1, card2) in hand1.cards.iter().zip(hand2.cards.iter()) {
                if card1 > card2 {
                    return Ordering::Greater;
                } else if card1 < card2 {
                    return Ordering::Less;
                }
            }
            panic!("no order for {hand1:?} and {hand2:?}");
        }
        type1.cmp(&type2)
    });
    let total_winnings: u64 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank as u64 + 1))
        .sum();
    println!("Part 1: {total_winnings}");
}
