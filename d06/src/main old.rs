use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn calculate_winning_margin(time: usize, to_beat: usize) -> usize {
    let mut left = 0;
    for pressed in 1..time {
        let run_time = time - pressed;
        let distance = run_time * pressed;
        if distance > to_beat {
            left = pressed;
            break;
        }
    }
    let mut right = 0;
    for pressed in (1..time).rev() {
        let run_time = time - pressed;
        let distance = run_time * pressed;
        if distance > to_beat {
            right = pressed;
            break;
        }
    }

    right - left + 1
}

fn run(input_file: &str) {
    // Preamble
    let mut result: usize = 0;
    let mut result2: usize = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let times: Vec<usize> = lines.next().unwrap().unwrap()[10..]
        .split_whitespace()
        .filter(|x| !x.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();
    let to_beat: Vec<usize> = lines.next().unwrap().unwrap()[10..]
        .split_whitespace()
        .filter(|x| !x.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

    // Solve

    for (i, time) in times.iter().enumerate() {
        if i == 0 {
            result = calculate_winning_margin(*time, to_beat[i]);
        } else {
            result *= calculate_winning_margin(*time, to_beat[i]);
        }
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let time_string = String::from_iter(
        lines.next().unwrap().unwrap()[10..]
            .split_whitespace()
            .filter(|x| !x.is_empty()),
    );
    let time: usize = time_string.parse().unwrap();

    let to_beat_string: String = String::from_iter(
        lines.next().unwrap().unwrap()[10..]
            .split_whitespace()
            .filter(|x| !x.is_empty()),
    );

    let to_beat: usize = to_beat_string.parse().unwrap();

    // Solve
    let result: usize = calculate_winning_margin(time, to_beat);

    // Result
    println!("Result of part 1 is {} or {}", result);
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
