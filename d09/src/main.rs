#![feature(array_windows)]
use utils::{get_input_path, parse_into_i64_vector_vector};

fn get_next_value(values: &Vec<i64>) -> i64 {
    let mut lines = Vec::new();

    let mut current_line = values;

    loop {
        let mut is_finished = true;
        let mut next_line = Vec::new();

        for [v1, v2] in current_line.array_windows() {
            let diff = v2 - v1;
            if diff != 0 {
                is_finished = false;
            }

            next_line.push(diff);
        }

        if is_finished {
            break;
        }
        lines.push(next_line);
        current_line = lines.last().unwrap();
    }

    let mut last_last_value = *lines.last().unwrap().first().unwrap();

    for i in (0..lines.len()-1).rev() {
        let last_value = *lines[i].last().unwrap();
        last_last_value = last_value + last_last_value;
    }

    values.last().unwrap() + last_last_value
}

fn get_previous_value(values: &Vec<i64>) -> i64 {
    let mut lines = Vec::new();

    let mut current_line = values;

    loop {
        let mut is_finished = true;
        let mut next_line = Vec::new();

        for [v1, v2] in current_line.array_windows() {
            let diff = v2 - v1;
            if diff != 0 {
                is_finished = false;
            }

            next_line.push(diff);
        }

        if is_finished {
            break;
        }
        lines.push(next_line);
        current_line = lines.last().unwrap();
    }

    let mut last_first_value = *lines.last().unwrap().first().unwrap();

    for i in (0..lines.len()-1).rev() {
        let first_value = *lines[i].first().unwrap();
        last_first_value = first_value - last_first_value;
    }

    values.first().unwrap() - last_first_value
}

fn run(input_file: &str) {
    // Parse
    let input = parse_into_i64_vector_vector(input_file);

    // Solve
    let result:i64 = input.iter().map(|i| get_next_value(i)).sum();

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
        // Parse
        let input = parse_into_i64_vector_vector(input_file);

        // Solve
        let result:i64 = input.iter().map(|i| get_previous_value(i)).sum();
    
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
