use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
#[derive(Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    _J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _T,
    _Q,
    _K,
    _A,
}

impl Card {
    pub fn from(c: char) -> Card {
        match c {
            'J' => Card::_J,
            '2' => Card::_2,
            '3' => Card::_3,
            '4' => Card::_4,
            '5' => Card::_5,
            '6' => Card::_6,
            '7' => Card::_7,
            '8' => Card::_8,
            '9' => Card::_9,
            'T' => Card::_T,
            'Q' => Card::_Q,
            'K' => Card::_K,
            'A' => Card::_A,
            _ =>   Card::_2
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    fn from(cards: &[Card; 5]) -> HandType {
        let mut card_counter: HashMap<Card, i32> = HashMap::new();
        for card in cards.to_owned() {
            match card_counter.get(&card) {
                Some(v) => card_counter.insert(card, v+1),
                None => card_counter.insert(card, 1),
            };
        }

        if card_counter.len() > 1 {
            if let Some(joker_counter) = card_counter.remove(&Card::_J) {
                let me = card_counter.iter().max_by( |c1, c2| c1.1.cmp(c2.1)).unwrap();
                card_counter.insert(me.0.clone(), me.1 + joker_counter);
            }
        }

        if card_counter.iter().filter(|c| *c.1 == 5).count() == 1 {
            return HandType::FiveKind;
        }
        if card_counter.iter().filter(|c| *c.1 == 4).count() == 1
        && card_counter.iter().filter(|c| *c.1 == 1).count() == 1 {
            return HandType::FourKind;
        }
        if card_counter.iter().filter(|c| *c.1 == 3).count() == 1
        && card_counter.iter().filter(|c| *c.1 == 2).count() == 1 {
            return HandType::FullHouse;
        }
        if card_counter.iter().filter(|c| *c.1 == 3).count() == 1
        && card_counter.iter().filter(|c| *c.1 == 1).count() == 2 {
            return HandType::ThreeKind;
        }
        if card_counter.iter().filter(|c| *c.1 == 2).count() == 2
        && card_counter.iter().filter(|c| *c.1 == 1).count() == 1 {
            return HandType::TwoPair;
        }
        if card_counter.iter().filter(|c| *c.1 == 2).count() == 1
        && card_counter.iter().filter(|c| *c.1 == 1).count() == 3 {
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq, Ord)]
pub struct Hand {
    pub cards: [Card; 5],
    pub hand_type: HandType,
    pub bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type > other.hand_type {
            return Some(Ordering::Greater);
        }
        if self.hand_type < other.hand_type {
            return Some(Ordering::Less);
        }
        for i in 0..self.cards.len() {
            if self.cards[i] > other.cards[i] {
                return Some(Ordering::Greater);
            }
            if self.cards[i] < other.cards[i] {
                return Some(Ordering::Less);
            }
        }
        return Some(Ordering::Equal);
    }
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: u32) -> Hand {
        Hand {
            hand_type: HandType::from(&cards),
            cards,
            bid,
        }
    }
}
