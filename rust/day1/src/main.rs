use std::{collections::HashMap, fs::File, io::Read};

const INPUT_NAME: &str = "input.txt";

fn digit_from_string(str: &str) -> Option<u32> {
    let digit_names = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for digit in digit_names.into_iter() {
        if str.contains(digit.0) {
            return Some(digit.1);
        }
    }
    return None;
}

fn analyze_line(line: &str) -> (u32, u32) {
    let mut a: Option<u32> = None;
    let mut b: Option<u32> = None;
    let mut buf = String::new();

    for c in line.chars() {
        if c.is_ascii_digit() {
            if let None = a {
                a = Some(c.to_digit(10).unwrap_or_default());
            } else {
                b = Some(c.to_digit(10).unwrap_or_default());
            }
        } else {
            buf.push(c);
            if let Some(digit) = digit_from_string(&buf) {
                if let None = a {
                    a = Some(digit);
                } else {
                    b = Some(digit);
                }
                buf.clear();
                buf.push(c);
            }
        }
    }

    if b.is_none() {
        b = a;
    }
    return (a.unwrap_or_default(), b.unwrap_or_default());
}

fn combine(a: u32, b: u32) -> u32 {
    a*10+b
}

fn main() {
    let mut file = File::open(INPUT_NAME).expect("Cannot open input file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Cannot read from input file");

    let mut sum = 0;
    for line in file_content.split('\n') {
        if line.is_empty() {
            continue;
        }

        let (a, b) = analyze_line(line);
        sum += combine(a, b);
    }
    println!("{}", sum);
}
