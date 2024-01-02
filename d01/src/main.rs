use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn has_number(line: &str) -> Option<u32> {
    const TO_FIND: [&str; 20] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    for (idx, &find) in TO_FIND.iter().enumerate() {
        if line.starts_with(find) {
            return Some(idx as u32 % 10);
        }
    }
    None
}

fn run(input_file: &str) {
    // Preamble
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let first = line
            .chars()
            .find(|&x| x.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap();

        let last = line
            .chars()
            .rfind(|&x| x.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap();

        let numbers = first * 10 + last;
        result += numbers
    }

    // Solve
    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let mut first = None;
        for i in 0..line.len() {
            if let Some(finding) = has_number(&line[i..]) {
                first = Some(finding);
                break;
            }
        }

        let mut last = None;
        for i in (0..line.len()).rev() {
            if let Some(finding) = has_number(&line[i..]) {
                last = Some(finding);
                break;
            }
        }

        let numbers = first.unwrap() * 10 + last.unwrap();
        result += numbers
    }

    // Solve
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
    use utils::get_test_input_2_path;
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
        let input_path = get_test_input_2_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
