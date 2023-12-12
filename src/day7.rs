type Output = Vec<Data>;

use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Data {
    cards: Vec<char>,
    hand: Hand,
    bid: u32,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Hand {
    High,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_hand(cards: Vec<char>) -> Hand {
    let mut a_map: HashMap<char, u32> = HashMap::new();
    cards.iter().for_each(|c| {
        a_map.insert(*c, a_map.get(&c).or(Some(&0)).unwrap() + 1);
    });
    if a_map.len() == 2 {
        if a_map.get(&cards[0]).is_some_and(|v| *v == 2)
            || a_map.get(&cards[0]).is_some_and(|v| *v == 3)
        {
            return Hand::FullHouse;
        } else {
            return Hand::FourOfAKind;
        }
    } else if a_map.len() == 3 {
        if a_map.values().any(|v| *v == 3) {
            return Hand::ThreeOfAKind;
        } else {
            return Hand::TwoPair;
        }
    } else if a_map.len() == 4 {
        return Hand::Pair;
    } else if a_map.len() == 5 {
        return Hand::High;
    } else {
        return Hand::FiveOfAKind;
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Output {
    let _input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    input
        .lines()
        .map(|l| {
            let s: Vec<&str> = l.split(" ").collect();
            let cards: Vec<char> = s[0].chars().collect();
            Data {
                cards: cards.clone(),
                hand: get_hand(cards),
                bid: s[1].parse::<u32>().unwrap(),
            }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &Output) -> u32 {
    let mut inp = input.clone();
    let rank: HashMap<char, u32> = HashMap::from([
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);
    let gt = Less;
    let lt = Greater;
    inp.sort_by(|a, b| {
        if a.hand != b.hand {
            a.hand.cmp(&b.hand)
        } else {
            a.cards
                .iter()
                .zip(b.cards.iter())
                .find_map(|(c_a, c_b)| {
                    if c_a == c_b {
                        return None;
                    }
                    if rank.get(c_a) > rank.get(c_b) {
                        Some(lt)
                    } else {
                        Some(gt)
                    }
                })
                .unwrap()
        }
    });
    inp.iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &Output) -> u32 {
    let mut inp = input.clone();
    let rank: HashMap<char, u32> = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);
    let gt = Less;
    let lt = Greater;
    inp.iter_mut().for_each(|h| {
        if h.cards.contains(&'J') {
            let mut a_map: HashMap<char, u32> = HashMap::new();
            h.cards.iter().for_each(|c| {
                a_map.insert(*c, a_map.get(&c).or(Some(&0)).unwrap() + 1);
            });
            let target = a_map
                .iter()
                .filter(|(k, _)| *k != &'J')
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _)| *k)
                .unwrap_or_else(|| 'A');
            dbg!(target);
            let cards = h
                .cards
                .iter()
                .map(|c| if *c == 'J' { target } else { *c })
                .collect::<Vec<char>>();
            dbg!(cards.clone());
            h.hand = get_hand(cards);
            dbg!(h.hand.clone());
        }
    });
    inp.sort_by(|a, b| {
        if a.hand != b.hand {
            a.hand.cmp(&b.hand)
        } else {
            a.cards
                .iter()
                .zip(b.cards.iter())
                .find_map(|(c_a, c_b)| {
                    if c_a == c_b {
                        return None;
                    }
                    if rank.get(c_a) > rank.get(c_b) {
                        Some(lt)
                    } else {
                        Some(gt)
                    }
                })
                .unwrap()
        }
    });
    //println!("{:?}", inp);
    inp.iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}
