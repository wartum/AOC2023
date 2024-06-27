use std::{fs::File, io::Read};

#[derive(Debug)]
struct RaceDetails {
    time: i64,
    distance: i64
}

impl RaceDetails {
    fn calculate_distance(&self, pressed_time: i64) -> i64 {
        let time_left = self.time - pressed_time;
        time_left * pressed_time
    }

    fn get_winning_strategies(&self) -> i32 {
        (1..self.time).into_iter()
            .map(|i| self.calculate_distance(i))
            .filter(|d| *d > self.distance)
            .count()
            .try_into()
            .unwrap()
    }
}

const INPUT_NAME: &str = "input.txt";

fn get_input(input_name: &str) -> String {
    let mut file = File::open(input_name).expect("Cannot open input file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Cannot read input file");
    return buffer;
}

fn get_race_details(input: &str) -> RaceDetails {
    let mut time_str = String::new();
    let mut dst_str  = String::new();
    let mut splits = input.split('\n');

    let first_line = splits.next().unwrap();
    first_line.chars()
        .filter(|c| c.is_ascii_digit())
        .for_each(|c| time_str.push(c));

    let second_line = splits.next().unwrap();
    second_line.chars()
        .filter(|c| c.is_ascii_digit())
        .for_each(|c| dst_str.push(c));

    RaceDetails {
        time: time_str.parse().unwrap(),
        distance: dst_str.parse().unwrap()
    }
}

fn get_races_details(input: &str) -> Vec<RaceDetails> {
    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();
    for line in input.split('\n') {
        let mut t_flag = false;
        let mut d_flag = false;
        for word in line.split_whitespace() {
            if word == "Time:" {
                t_flag = true;
                d_flag = false;
            } else if word == "Distance:" {
                t_flag = false;
                d_flag = true;
            } else {
                if t_flag {
                    times.push(word.parse().expect("Cannot parse a number"));
                }
                if d_flag {
                    distances.push(word.parse().expect("Cannot parse a number"));
                }
            }
        }
    }
    assert_eq!(times.len(), distances.len());

    (0..times.len())
        .into_iter()
        .map(|i| RaceDetails{time: times[i], distance: distances[i]})
        .collect()
}

fn main() {
    let input = get_input(INPUT_NAME);
    let races = get_races_details(&input);
    let winning_strategies_number: Vec<i32> = races.iter().map(|r| r.get_winning_strategies()).collect();
    let mut output = 0;
    for x in winning_strategies_number {
        if output == 0 {
            output = x;
        } else {
            output *= x;
        }
    }
    println!("{}", output);

    let race = get_race_details(&input);
    println!("{}", race.get_winning_strategies());
}
