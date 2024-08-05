mod camel_cards;
use camel_cards::*;
use std::fs;

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Cannot read input file")
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| String::from(s))
        .collect()
}

fn hand_from_line(line: &str) -> Hand {
    let cards: [Card; 5] = line
        .split_whitespace()
        .take(1)
        .collect::<String>()
        .chars()
        .map(|c| Card::from(c))
        .collect::<Vec<Card>>()
        .try_into()
        .expect("Cards number in hand incorrect");
    let bid: u32 = line
        .split_whitespace()
        .skip(1)
        .take(1)
        .collect::<String>()
        .parse()
        .expect("Cannot parse bid value");
    Hand::new(cards, bid)
}

fn main() {
    let mut hands = read_input()
        .iter()
        .map(|l| hand_from_line(l))
        .collect::<Vec<Hand>>();
    hands.sort();

    let mut score = 0;
    let mut rank = 1;
    for hand in hands.iter() {
        score += hand.bid * rank;
        rank += 1;
    }
    println!("{}", score);
}
