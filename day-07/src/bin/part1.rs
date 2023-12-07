use anyhow::{anyhow, Error};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let mut lines: Vec<Hand> = input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => {
                Some(trimmed.parse().unwrap_or_else(|err| panic!("{err:?}")))
            }
            _ => None,
        })
        .collect();
    lines.sort();

    lines
        .into_iter()
        .enumerate()
        .fold(0, |sum, (idx, hand)| sum + (idx + 1) * hand.bid)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_hand_type(cards: &[Card]) -> HandType {
    let mut map: HashMap<Card, usize> = HashMap::with_capacity(5);
    cards.iter().for_each(|card| {
        map.entry(*card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    let mut three_of_a_kind_count = 0;
    let mut two_of_a_kind_count = 0;
    for count in map.values() {
        if *count == 5 {
            return HandType::FiveOfAKind;
        } else if *count == 4 {
            return HandType::FourOfAKind;
        } else if *count == 3 {
            three_of_a_kind_count += 1;
        } else if *count == 2 {
            two_of_a_kind_count += 1;
        }
    }

    if three_of_a_kind_count > 0 && two_of_a_kind_count > 0 {
        return HandType::FullHouse;
    } else if three_of_a_kind_count > 0 {
        return HandType::ThreeOfAKind;
    } else if two_of_a_kind_count > 1 {
        return HandType::TwoPair;
    } else if two_of_a_kind_count > 0 {
        return HandType::OnePair;
    }
    HandType::HighCard
}

#[derive(Debug, Clone)]
struct Hand {
    bid: usize,
    cards: Vec<Card>,
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cards_str, bid_str)) = s.split_once(' ') {
            let bid = bid_str.parse::<usize>()?;
            let cards: Vec<Card> = cards_str.chars().map(|c| c.into()).collect();
            let hand_type = get_hand_type(&cards);

            return Ok(Self {
                bid,
                cards,
                hand_type,
            });
        }
        Err(anyhow!("Could not parse a Hand: {s}"))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {
                for (l, r) in self.cards.iter().zip(other.cards.iter()) {
                    match l.cmp(r) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
            }
            Ordering::Greater => return Ordering::Greater,
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
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
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("unrecognized card"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "#;
        assert_eq!(6440, process(input));
    }
}
