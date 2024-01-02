use std::fs;

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut result: usize = 0;
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    // Solve
    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(_input_file: &str) {
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