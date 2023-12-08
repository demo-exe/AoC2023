use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    hand_type: Type,
    bid: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        debug_assert!(self.cards.len() == 5);

        if self.hand_type == other.hand_type {
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    return false;
                }
            }
            return true;
        }
        false
    }
}
impl Eq for Hand {}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        debug_assert!(self.cards.len() == 5);

        if self.hand_type == other.hand_type {
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    return Some(self.cards[i].cmp(&other.cards[i]));
                }
            }
            return Some(Ordering::Equal);
        }
        Some(self.hand_type.cmp(&other.hand_type))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl Hand {
    fn faces_to_u8(faces: Vec<char>) -> Vec<u8> {
        let mut cards = Vec::new();
        for face in faces.iter() {
            match face {
                'T' => cards.push(10),
                'J' => cards.push(11),
                'Q' => cards.push(12),
                'K' => cards.push(13),
                'A' => cards.push(14),
                _ => cards.push(face.to_digit(10).unwrap() as u8),
            }
        }
        cards
    }
    fn new(cards: Vec<char>, bid: usize) -> Hand {
        let cards = Hand::faces_to_u8(cards);

        let mut map = HashMap::new();
        for card in cards.iter() {
            let count = map.entry(card).or_insert(0);
            *count += 1;
        }
        let mut hand = Hand {
            cards: cards.clone(),
            hand_type: Type::FiveOfAKind,
            bid,
        };
        if map.len() == 2 {
            for (_, count) in map.iter() {
                if *count == 4 {
                    hand.hand_type = Type::FourOfAKind;
                } else if *count == 3 {
                    hand.hand_type = Type::FullHouse;
                }
            }
        } else if map.len() == 3 {
            for (_, count) in map.iter() {
                if *count == 3 {
                    hand.hand_type = Type::ThreeOfAKind;
                } else if *count == 2 {
                    hand.hand_type = Type::TwoPair;
                }
            }
        } else if map.len() == 4 {
            hand.hand_type = Type::Pair;
        } else if map.len() == 5 {
            hand.hand_type = Type::HighCard;
        }
        hand
    }
}

fn main() {
    let mut lines = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(rawlines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in rawlines {
            if let Ok(ip) = line {
                lines.push(ip);
            }
        }
    }

    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|line| {
            let line = line.split_whitespace().collect::<Vec<&str>>();
            Hand::new(line[0].chars().collect(), line[1].parse::<usize>().unwrap())
        })
        .collect();

    // dbg!(&hands);

    hands.sort_by(|a, b| a.cmp(b));

    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1));

    dbg!(result);
}
