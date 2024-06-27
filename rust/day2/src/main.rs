use std::{fs::File, io::Read, vec};

const INPUT_NAME: &str = "input.txt";

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32)
}

impl Cube {
    fn new(str: &str, number: u32) -> Option<Cube> {
        match str {
            "red" => Some(Cube::Red(number)),
            "green" => Some(Cube::Green(number)),
            "blue" => Some(Cube::Blue(number)),
            _ => None
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<Cube>
}

fn get_game_cubes(str: &str) -> Vec<Cube> {
    let mut cubes: Vec<Cube> = vec![];
    for token in str.split(';') {
        let mut new_cubes = get_game_cubes_internal(token.trim());
        cubes.append(&mut new_cubes);
    }
    
    return cubes;
}

fn get_game_cubes_internal(str: &str) -> Vec<Cube> {
    let mut cubes: Vec<Cube> = vec![];
    let mut number = 0;

    for t in str.split(',') {
        for token in t.split(' ') {
            if token.trim().is_empty() {
                continue;
            }
            if token.chars().any(|c| c.is_ascii_digit()) {
                number = token.trim().parse().expect("Can't parse to int");
            } else {
                let cube = Cube::new(token.trim(), number);
                if let Some(c) = cube {
                    cubes.push(c);
                }
            }
        }
    }

    return cubes;
}

fn get_game_id(str: &str) -> u32 {
    for token in str.split(' ') {
        if token.chars().any(|c| c.is_ascii_digit()) {
            return token.parse().expect("Can't parse to int");
        }
    }

    return 0;
}

fn optimize_cubes(mut game: Game) -> Game {
    let original_cubes = game.cubes;
    let mut max_r: u32 = 0;
    let mut max_g: u32 = 0;
    let mut max_b: u32 = 0;
    for cube in original_cubes.iter() {
        match cube {
            Cube::Red(v) => {
                if v > &max_r {
                    max_r = *v;
                }
            },
            Cube::Green(v) => {
                if v > &max_g {
                    max_g = *v;
                }
            },
            Cube::Blue(v) => {
                if v > &max_b {
                    max_b = *v;
                }
            },

        }
    }
    game.cubes = vec![
        Cube::Red(max_r),
        Cube::Green(max_g),
        Cube::Blue(max_b)];
    return game;
}

fn line_to_game(line: &str) -> Game {
    let mut game = Game {
        id: 0,
        cubes: vec![]
    };

    for token in line.split(':') {
        if token.trim().starts_with("Game") {
            game.id = get_game_id(token);
        } else {
            game.cubes = get_game_cubes(token);
        }
    }

    return optimize_cubes(game);
}

fn main() {
    let mut file = File::open(INPUT_NAME).expect("Cannot open input file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Cannot read input file");

    let mut games: Vec<Game> = vec![];
    for line in file_content.split('\n') {
        if line.is_empty() {
            continue;
        }
        games.push(line_to_game(line));
    }

    let mut sum = 0;
    let mut power = 0;
    for game in games.iter() {
        if game.cubes.iter().all(|c| match c {
            Cube::Red(v) => *v <= 12,
            Cube::Green(v) => *v <= 13,
            Cube::Blue(v) => *v <= 14,
        }) {
            sum += game.id;
        }

        let mut x = 0;
        game.cubes.iter().for_each(|c| {
                let v = match c {
                    Cube::Red(v) => *v,
                    Cube::Green(v) => *v,
                    Cube::Blue(v) => *v,
                };
                if x == 0 {
                    x = v;
                } else {
                    x *= v;
                }
            });
        power += x;
    }

    println!("{}, {}", sum, power);
}
