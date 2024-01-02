#![feature(iter_array_chunks)]
use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};


use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    const MAX_RED:usize = 12;
    const MAX_GREEN:usize = 13;
    const MAX_BLUE:usize = 14;

    let mut result = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    
    // Solve
    'game: for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split = line.split([':', ' ',',', ';']);
        let view:Vec<&str> = split.filter(|i| !i.is_empty()).collect();

        for [&num, &color] in view.iter().skip(2).array_chunks(){
            let num_value: usize = num.parse().unwrap();
            match color{
                "red" => {
                    if num_value >MAX_RED {
                        continue 'game;
                    }
                }
                "green" => {
                    if num_value >MAX_GREEN {
                        continue 'game;
                    }
                }
                "blue" => {
                    if num_value >MAX_BLUE {
                        continue 'game;
                    }
                }
                _ => panic!("Should not be here")
            }
        }
        let game_id:usize = view[1].parse().unwrap();
        result += game_id;
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
        // Preamble
        let mut result = 0;
    
        // Parse
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
    
        
        // Solve
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
            let split = line.split([':', ' ',',', ';']);
            let view:Vec<&str> = split.filter(|i| !i.is_empty()).collect();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
    
            for [&num, &color] in view.iter().skip(2).array_chunks(){
                let num_value: usize = num.parse().unwrap();
                match color{
                    "red" => {
                        red = red.max(num_value);
                    }
                    "green" => {
                        green = green.max(num_value);
                    }
                    "blue" => {
                        blue = blue.max(num_value);
                    }
                    _ => panic!("Should not be here")
                }
            }
            result += red * green * blue;
        }
    
        // Result
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
