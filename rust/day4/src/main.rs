use std::{array, io::Read, fs::File};

#[derive(Debug)]
#[derive(Clone)]
struct Card {
    id: i32,
    winning_numbers: [i32; 10],
    your_numbers: [i32; 25],
}

impl Card {
    fn from(line: &str) -> Card {
        let id: i32;
        let mut winning_numbers = array::from_fn(|_| 0 );
        let mut your_numbers = array::from_fn(|_| 0 );

        let mut s1 = line.split(":");
        let id_split = s1.next().expect("Cannot get id plit");
        let numbers_split = s1.next().expect("Cannot get numbers split");

        let mut s2 = numbers_split.split("|");
        let winning_split = s2.next().expect("Cannot get winning numbers split");
        let your_split = s2.next().expect("Cannot get your numbers split");

        match id_split.trim().split_whitespace().skip(1).next().expect("Cannot read card id").parse::<i32>() {
                Ok(v) => id = v,
                Err(_) => id = -999,
        }

        for number in winning_split.split_whitespace().enumerate() {
            match number.1.parse::<i32>() {
                Ok(v) => winning_numbers[number.0] = v,
                Err(_) => panic!("Cannot parse number")
            }
        }

        for number in your_split.split_whitespace().enumerate() {
            match number.1.parse::<i32>() {
                Ok(v) => your_numbers[number.0] = v,
                Err(_) => panic!("Cannot parse number")
            }
        }

        Card { id, winning_numbers, your_numbers }
    }

    fn count_points(&self) -> i32 {
        let n = self.your_numbers.iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count();

        return n2points(n.try_into().unwrap());
    }
}

fn new_card_from_deck(deck: &Vec<Card>, id: i32) -> Option<Card> {
    deck.iter()
        .filter(|d| d.id == id)
        .cloned()
        .collect::<Vec<Card>>()
        .first()
        .cloned()
}

fn spawn_new_cards(card: &Card, original_deck: &Vec<Card>, new_deck: &mut Vec<Card>) {
    let n: i32 = card.your_numbers.iter()
        .filter(|x| card.winning_numbers.contains(x))
        .count() .try_into().unwrap();
    let begin = card.id+1;
    let end = card.id+1+n;

    for id in (begin..end).into_iter() {
        match new_card_from_deck(original_deck, id) {
            None => {},
            Some(c) => new_deck.push(c)
        }
    }
} 

fn n2points(n: i32) -> i32 {
    if n < 2 {
        return n;
    } else {
        let mut points = 1;
        for _ in 1..n {
            points *= 2;
        }
        return points;
    }
}

const INPUT_NAME: &str = "input.txt";

fn get_input() -> String {
    let mut file = File::open(INPUT_NAME).expect("Cannot open an input file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Cannot read an input file");
    return content;
}

fn main() {
    let mut original_cards: Vec<Card> = Vec::new();
    original_cards.reserve(15000000);
    let input = get_input();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        original_cards.push(Card::from(line));
    }

    let sum: i32 = original_cards.iter()
        .map(|c| c.count_points())
        .sum();
    println!("{}", sum);

    let mut current_cards = original_cards.clone();
    let mut another_cards = Vec::new();
    another_cards.reserve(current_cards.len());
    let mut total = 0;
    loop {
        total += current_cards.len();
        for c in current_cards.iter() {
            spawn_new_cards(&c, &original_cards, &mut another_cards);
        }
        if another_cards.len() == 0 {
            break;
        } else {
            let tmp = current_cards;
            current_cards = another_cards;
            another_cards = tmp;
            another_cards.clear();
        }
    }
    println!("{}", total);
}
