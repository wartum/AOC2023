use std::{collections::HashMap, io::Read};

const INPUT_NAME: &str = "input.txt";

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(Clone, Copy)]
struct XY(i32, i32);

impl XY {
    fn get_adjacent_symbol(&self, symbols: &Vec<Point>) -> Option<Point>{
        for symbol in symbols {
            if (symbol.0.0 != self.0 - 1) && (symbol.0.0 != self.0) && (symbol.0.0 != self.0 + 1) {
                continue;
            }
    
            if symbol.0.get_adjacent_xy().contains(self) {
                return Some(symbol.clone());
            }
        }
        None
    }

    fn is_part_number(&self, symbols: &Vec<Point>) -> bool{
        match self.get_adjacent_symbol(symbols) {
            Some(_) => true,
            None => false,
        }
    }

    fn get_adjacent_xy(&self) -> [XY; 8] {
        [
            XY(self.0-1, self.1-1), XY(self.0,   self.1-1), XY(self.0+1, self.1-1),
            XY(self.0-1, self.1),   XY(self.0+1, self.1),
            XY(self.0-1, self.1+1), XY(self.0,   self.1+1), XY(self.0+1, self.1+1),
        ]
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(Clone, Copy)]
struct Point(XY, char);

fn get_input() -> String {
    let mut file = std::fs::File::open(INPUT_NAME).expect("Cannot open input file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Cannot read input file");
    return content;
}

fn get_points_list(input: &str) -> Vec<Point> {
    let mut all_points = Vec::<Point>::new();
    let mut x: i32;
    let mut y: i32 = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        y += 1;
        x = 0;
        for c in line.chars() {
            x += 1;
            all_points.push(Point(XY(x, y), c));
        }
    }
    return all_points;

}

fn get_symbols_list(points: &Vec<Point>) -> Vec<Point> {
    points.iter()
        .filter(|p| !p.1.is_ascii_digit() && p.1 != '.')
        .copied()
        .collect()
}

fn get_digits_list(points: &Vec<Point>) -> Vec<Point> {
    points.iter()
        .filter(|p| p.1.is_ascii_digit())
        .copied()
        .collect()
}

fn get_parts_numbers(digits: &Vec<Point>, symbols: &Vec<Point>) -> Vec<i32> {
    let mut parts = Vec::<i32>::new();
    let mut string_builder = String::new();
    string_builder.reserve(5);
    let mut last_xy = XY(0,0);
    let mut is_part_nmb = false;
    
    for digit in digits {
        if (digit.0.0 != last_xy.0 + 1) || (digit.0.1 != last_xy.1) {
            if string_builder.len() > 0 && is_part_nmb {
                match string_builder.parse::<i32>() {
                    Ok(v) => parts.push(v),
                    Err(_) => println!("Couldn't parse '{}'", string_builder.as_str()),
                }
            }
            is_part_nmb = digit.0.is_part_number(symbols);
            string_builder.clear();
        }
        if !is_part_nmb {
            is_part_nmb = digit.0.is_part_number(symbols);
        }
        string_builder.push(digit.1);
        last_xy = digit.0;
    }

    if string_builder.len() > 0 && is_part_nmb {
        match string_builder.parse::<i32>() {
            Ok(v) => parts.push(v),
            Err(_) => println!("Couldn't parse '{}'", string_builder.as_str()),
        }
    }

    return parts;
}

fn push_number_into_map(map: &mut HashMap<XY, Vec<i32>>, xy: &XY, value: i32) {
    if map.contains_key(xy) {
        map.get_mut(xy).expect("Cannot get mutable reference").push(value);
    } else {
        map.insert(*xy, vec![value]);
    }
}

fn get_gears(digits: &Vec<Point>, symbols: &Vec<Point>) -> HashMap<XY, Vec<i32>> {
    let gears: Vec<Point> = symbols.iter().filter(|d| d.1 == '*').cloned().collect();
    let mut gear_map = HashMap::new();
    let mut string_builder = String::new();
    string_builder.reserve(5);
    let mut last_xy = XY(0,0);
    let mut last_symbol_xy = XY(0,0);
    let mut is_part_nmb = false;
    
    for digit in digits {
        if (digit.0.0 != last_xy.0 + 1) || (digit.0.1 != last_xy.1) {
            if string_builder.len() > 0 && is_part_nmb {
                match string_builder.parse::<i32>() {
                    Ok(v) => push_number_into_map(&mut gear_map, &last_symbol_xy, v),
                    Err(_) => println!("Couldn't parse '{}'", string_builder.as_str()),
                }
            }
            match digit.0.get_adjacent_symbol(&gears) {
                Some(v) => {
                    is_part_nmb = true;
                    last_symbol_xy = v.0;
                }
                None => is_part_nmb = false,
            }
            string_builder.clear();
        }
        if !is_part_nmb {
            match digit.0.get_adjacent_symbol(&gears) {
                Some(v) => {
                    is_part_nmb = true;
                    last_symbol_xy = v.0;
                }
                None => is_part_nmb = false,
            }
        }
        string_builder.push(digit.1);
        last_xy = digit.0;
    }

    if string_builder.len() > 0 && is_part_nmb {
        match string_builder.parse::<i32>() {
            Ok(v) => push_number_into_map(&mut gear_map, &last_symbol_xy, v),
            Err(_) => println!("Couldn't parse '{}'", string_builder.as_str()),
        }
    }

    return gear_map;
}

fn main() {
    let input = get_input();
    let all_points = get_points_list(&input);
    let symbols = get_symbols_list(&all_points);
    let digits = get_digits_list(&all_points);
    let parts = get_parts_numbers(&digits, &symbols);
    let gears = get_gears(&digits, &symbols);

    let sum: i32 = parts.iter().sum();
    let mut ratios = 0;
    for gear in gears {
        if gear.1.len() == 2 {
            ratios += gear.1.get(0).unwrap() * gear.1.get(1).unwrap();
        }
    }
    println!("{}, {}", sum, ratios);
}
