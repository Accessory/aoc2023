use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn calc_margin_with_quadratic_formular(time: f64, to_beat: f64) -> usize {
    let sqr = f64::sqrt(time * time - 4.0 * to_beat);
    let root1 = 0.5 * (time + sqr);
    let root2 = 0.5 * (time - sqr);

    (f64::floor(root1) - f64::ceil(root2)) as usize + 1
}

fn run(input_file: &str) {
    // Preamble
    let mut result: usize = 0;

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
            result = calc_margin_with_quadratic_formular(*time as f64, to_beat[i] as f64);
        } else {
            result *= calc_margin_with_quadratic_formular(*time as f64, to_beat[i] as f64);
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
    let time_string = lines.next().unwrap().unwrap()[10..].replace(' ', "");
    let time: usize = time_string.parse().unwrap();
    let to_beat_string: String = lines.next().unwrap().unwrap()[10..].replace(' ', "");
    let to_beat: usize = to_beat_string.parse().unwrap();

    // Solve
    let result = calc_margin_with_quadratic_formular(time as f64, to_beat as f64);

    // Result
    println!("Result of part 1 is {}", result);
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
