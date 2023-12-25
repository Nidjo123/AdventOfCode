use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;

use counter::Counter;

mod counter;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let hands = parse_hands(&input);
    part1(&hands);
    part2(&hands);
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

impl Card {
    fn iter_cards() -> impl Iterator<Item = Self> {
        use Card::*;
        [
            Ace, King, Queen, Jack, T, Nine, Eight, Seven, Six, Five, Four, Three, Two,
        ]
        .into_iter()
    }
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

fn joker_card_value(card: Card) -> usize {
    if card == Card::Jack {
        0
    } else if card < Card::Jack {
        card as usize + 1
    } else {
        card as usize
    }
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u64) -> Hand {
        Hand {
            cards: cards.try_into().unwrap(),
            bid,
        }
    }

    fn get_cards_hand_type(cards: &[Card; 5], card_values: Option<fn(Card) -> usize>) -> HandType {
        let counter = Counter::from_iter(cards.iter());
        let mut counts = counter.into_iter().collect::<Vec<_>>();
        let card_value = card_values.unwrap_or(|card| card as usize);
        counts.sort_by_key(|(card, count)| (*count, card_value(**card)));
        if let [(&card, 5)] = counts[..] {
            return HandType::FiveOfAKind(card);
        }
        if let [_, (&card, 4)] = counts[..] {
            return HandType::FourOfAKind(card);
        }
        if let [(&card2, 2), (&card1, 3)] = counts[..] {
            return HandType::FullHouse(card1, card2);
        }
        if let [(_, 1), (_, 1), (&card1, 3)] = counts[..] {
            return HandType::ThreeOfAKind(card1);
        }
        if let [(_, 1), (&card2, 2), (&card1, 2)] = counts[..] {
            return HandType::TwoPair(card1, card2);
        }
        if let [.., (&card1, 2)] = counts[..] {
            return HandType::OnePair(card1);
        }
        HandType::HighCard(
            cards
                .iter()
                .map(|card| (card, card_value(*card)))
                .max_by_key(|(_, card_val)| *card_val)
                .unwrap()
                .0
                .clone(),
        )
    }

    fn get_type(&self, card_values: Option<fn(Card) -> usize>) -> HandType {
        Self::get_cards_hand_type(&self.cards, card_values)
    }

    fn get_joker_type(&self) -> HandType {
        Self::get_joker_type_(self.cards.clone())
    }

    fn get_joker_type_(cards: [Card; 5]) -> HandType {
        let mut best_hand_type = Self::get_cards_hand_type(&cards, Some(joker_card_value));
        for (i, _) in cards
            .iter()
            .enumerate()
            .filter(|(_, &card)| card == Card::Jack)
        {
            for new_card in Card::iter_cards().filter(|&x| x != Card::Jack) {
                let mut new_cards = cards.clone();
                new_cards[i] = new_card;
                let new_hand_type = Self::get_joker_type_(new_cards);
                best_hand_type = best_hand_type.max(new_hand_type);
            }
        }
        best_hand_type
    }
}

fn part1(hands: &Vec<Hand>) {
    let mut hands: Vec<_> = hands
        .iter()
        .copied()
        .map(|hand| (hand, hand.get_type(None)))
        .collect();
    hands.sort_by(|(hand1, type1), (hand2, type2)| {
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
        .map(|(rank, (hand, _))| hand.bid * (rank as u64 + 1))
        .sum();
    println!("Part 1: {total_winnings}");
}

fn part2(hands: &Vec<Hand>) {
    let mut hands: Vec<_> = hands
        .iter()
        .copied()
        .map(|hand| (hand, hand.get_joker_type()))
        .collect();

    hands.sort_by(|(hand1, type1), (hand2, type2)| {
        if type1 == type2 {
            for (card1, card2) in hand1.cards.iter().zip(hand2.cards.iter()) {
                match joker_card_value(*card1).cmp(&joker_card_value(*card2)) {
                    ordering @ (Ordering::Less | Ordering::Greater) => return ordering,
                    _ => continue,
                }
            }
            panic!("no order for {hand1:?} and {hand2:?}");
        }
        type1.cmp(&type2)
    });
    let total_winnings: u64 = hands
        .iter()
        .enumerate()
        .map(|(rank, (hand, _))| hand.bid * (rank as u64 + 1))
        .sum();
    println!("Part 2: {total_winnings}");
}
