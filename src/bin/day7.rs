use core::panic;
use std::{cmp::Ordering, collections::HashMap};

use aoc::lines;

fn main() {
    let input = lines("input/work7");
    let mut hands = parse_hands(&input, Card::new);
    hands.sort_by(|a, b| compare_hands(a, b, |h| primary_rank_part1(hand_counts(h))));

    let part1: usize = hands
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let rank = i + 1;
            rank * h.bet
        })
        .sum();

    dbg!(part1);

    let mut hands2 = parse_hands(&input, Card::new_part2);
    hands2.sort_by(|a, b| compare_hands(a, b, |h| primary_rank_part2(hand_counts(h))));

    let part2: usize = hands2
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let rank = i + 1;
            rank * h.bet
        })
        .sum();

    dbg!(part2);
}

fn primary_rank_part2(mut counts: HashMap<Card, usize>) -> usize {
    let jokers = counts.remove(&Card(JOKER_VALUE_PART2)).unwrap_or(0);
    let three_of_kind = counts.values().any(|c| *c == 3);
    let kinds = counts.len();

    if jokers == 5 || jokers == 4 || kinds == 1 {
        // 5 of a kind
        return 6;
    }

    if jokers == 3 {
        match kinds {
            2 => return 5, // 4kind, 1,1 + 3 jokers
            _ => panic!(),
        }
    }

    if jokers == 2 {
        match kinds {
            2 => return 5, // 4kind: 2,1 + 2j
            3 => return 3, // 3kind: 1,1,1. 2j
            _ => panic!(),
        }
    }

    if jokers == 1 {
        match kinds {
            2 => {
                return if three_of_kind {
                    5 // 3,1 + j
                } else {
                    4 // 2,2 + j full house
                };
            }
            3 => return 3, // 3 kind? 2,1,1 + 1joker
            4 => return 1, // 1 pair, 1,1,1,1 + joker
            _ => panic!(),
        }
    }

    if jokers == 0 {
        match kinds {
            2 => {
                return if three_of_kind {
                    4 // 3,2
                } else {
                    5 // 4,1
                };
            } // 4,1 or 3,2
            // 3 kind? 2,2,1 or 3,1,1
            3 => {
                return if three_of_kind {
                    3 // 3,1,1
                } else {
                    2 // 2,2,1 two pair
                };
            }
            4 => return 1, // 1 pair, 2,1,1,1
            5 => return 0,
            _ => panic!(),
        }
    }

    panic!()
}

fn compare_hands(left: &Hand, right: &Hand, rank_fn: fn(&Hand) -> usize) -> Ordering {
    let leftr = rank_fn(left);
    let rightr = rank_fn(right);

    leftr.cmp(&rightr).then_with(|| {
        for i in 0..left.cards.len() {
            if left.cards[i] < right.cards[i] {
                return Ordering::Less;
            } else if left.cards[i] > right.cards[i] {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    })
}

fn primary_rank_part1(counts: HashMap<Card, usize>) -> usize {
    if counts.values().any(|v| *v == 5) {
        return 6;
    }

    if counts.values().any(|v| *v == 4) {
        return 5;
    }

    if counts.len() == 2 && counts.values().all(|v| *v == 3 || *v == 2) {
        // full house
        return 4;
    }

    if counts.values().any(|v| *v == 3) {
        // three of a kind
        return 3;
    }

    let pairs = counts.values().filter(|v| **v == 2).count();

    if pairs == 2 {
        return 2;
    }

    if pairs == 1 {
        return 1;
    }

    0
}

fn hand_counts(h: &Hand) -> HashMap<Card, usize> {
    let mut counts = HashMap::new();
    for card in &h.cards {
        *counts.entry(*card).or_insert(0) += 1;
    }
    counts
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
struct Card(u8);

const JOKER_VALUE_PART2: u8 = 1;

impl Card {
    pub fn new(c: char) -> Self {
        Card(match c {
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!(),
        })
    }

    pub fn new_part2(c: char) -> Self {
        Card(match c {
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            'T' => 10,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            'J' => JOKER_VALUE_PART2,
            _ => panic!(),
        })
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bet: usize,
}

fn parse_hands(input: &[String], f: fn(char) -> Card) -> Vec<Hand> {
    let mut hands = vec![];
    for line in input {
        let (cards, bet) = line.split_once(' ').unwrap();
        hands.push(Hand {
            cards: cards.chars().map(f).collect(),
            bet: bet.parse().unwrap(),
        });
    }

    hands
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_answer() {
        let input = lines("input/day7");
        let mut hands = parse_hands(&input, Card::new);
        hands.sort_by(|a, b| compare_hands(a, b, |h| primary_rank_part1(hand_counts(h))));

        let part1: usize = hands
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let rank = i + 1;
                rank * h.bet
            })
            .sum();

        assert_eq!(249748283, part1);

        let mut hands2 = parse_hands(&input, Card::new_part2);
        hands2.sort_by(|a, b| compare_hands(a, b, |h| primary_rank_part2(hand_counts(h))));

        let part2: usize = hands2
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let rank = i + 1;
                rank * h.bet
            })
            .sum();

        assert_eq!(248029057, part2);
    }
}
