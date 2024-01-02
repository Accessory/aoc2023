use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;
use utils::grid_point::GridPoint;

#[derive(Debug)]
struct NumberRange {
    pub x_from: usize,
    pub x_to: usize,
    pub y: usize,
    pub number: usize,
    pub flaged: bool,
}

impl NumberRange {
    fn new(x_from: usize, y: usize) -> Self {
        Self {
            x_from,
            x_to: x_from,
            y,
            number: 0,
            flaged: false,
        }
    }
}

fn overlap(neigbor: &GridPoint, number: &NumberRange) -> bool {
    neigbor.y == number.y && number.x_from <= neigbor.x && number.x_to >= neigbor.x
}

fn run(input_file: &str) {
    // Preamble
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut current_number: Option<NumberRange> = None;

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();

        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let cn = current_number.get_or_insert(NumberRange::new(x, y));
                cn.x_to = x;
                continue;
            } else {
                if let Some(mut cn) = current_number {
                    cn.number = line[cn.x_from..cn.x_to + 1].parse().unwrap();
                    numbers.push(cn);
                    current_number = None;
                }
            }

            if c == '.' {
                continue;
            }

            symbols.push(GridPoint::new(x, y));
        }

        if let Some(mut cn) = current_number {
            cn.number = line[cn.x_from..cn.x_to + 1].parse().unwrap();
            numbers.push(cn);
            current_number = None;
        }
    }

    // Solve
    for symbol in symbols {
        let neigbors = symbol.generate_neigbors();
        for neigbor in neigbors {
            for number in numbers.iter_mut() {
                if !number.flaged && overlap(&neigbor, number) {
                    number.flaged = true;
                }
            }
        }
    }

    // Result
    let result: usize = numbers.iter().filter(|x| x.flaged).map(|y| y.number).sum();

    println!("Result of part 1 is {}", result);
}

#[derive(Debug)]
struct Gears {
    position: GridPoint,
    numbers: HashSet<usize>,
}

impl Gears {
    fn new(position: GridPoint) -> Self {
        Self {
            position,
            numbers: HashSet::new(),
        }
    }
}

fn run2(input_file: &str) {
    // Preamble
    let mut gears = Vec::new();
    let mut numbers = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut current_number: Option<NumberRange> = None;

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();

        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let cn = current_number.get_or_insert(NumberRange::new(x, y));
                cn.x_to = x;
                continue;
            } else {
                if let Some(mut cn) = current_number {
                    cn.number = line[cn.x_from..cn.x_to + 1].parse().unwrap();
                    numbers.push(cn);
                    current_number = None;
                }
            }

            if c == '.' {
                continue;
            }
            if c == '*' {
                gears.push(Gears::new(GridPoint::new(x, y)));
            }
        }

        if let Some(mut cn) = current_number {
            cn.number = line[cn.x_from..cn.x_to + 1].parse().unwrap();
            numbers.push(cn);
            current_number = None;
        }
    }

    // Solve
    for gear in gears.iter_mut() {
        let neigbors = gear.position.generate_neigbors();
        for neigbor in neigbors {
            for (i,number) in numbers.iter().enumerate() {
                if overlap(&neigbor, number) {
                    gear.numbers.insert(i);
                }
            }
        }
    }

    // Result
    let mut result = 0;

    for gear in gears {
        if gear.numbers.len() != 2 {
            continue;
        }

        let mut gear_ratio = 0;

        for number_idx in gear.numbers {
            if gear_ratio == 0 {
                gear_ratio += numbers[number_idx].number;
            } else {
                gear_ratio *= numbers[number_idx].number;
            }
        }

        result += gear_ratio;
    }

    println!("Result of part 2 is {}", result);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
    run2(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
