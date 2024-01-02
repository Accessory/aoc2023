use std::collections::HashMap;

use utils::{get_input_path, parse_file_into};

#[derive(Debug)]
struct SpringConfig {
    numbers: Vec<usize>,
    configuration: String,
    dp: HashMap<(usize, usize, usize), usize>,
}
impl SpringConfig {
    fn extend(&mut self, by: usize){
        self.numbers = self.numbers.repeat(by+1);
        let to_add = format!("?{}", self.configuration);
        for _ in 0..by {
            self.configuration.push_str(&to_add);
        }
    }
    fn permute(&mut self, char_index: usize, number_index: usize, current_block: usize) -> usize {
        // dp cache check
        if let Some(value) = self.dp.get(&(char_index, number_index, current_block)) {
            return *value;
        }
        // if we reached the end of the configuration, check if it is valid
        if char_index == self.configuration.len() {
            let result = if (current_block == 0 && number_index >= self.numbers.len())
                || (number_index == self.numbers.len() - 1
                    && current_block == self.numbers[number_index])
            {
                1
            } else {
                0
            };
            self.dp
                .insert((char_index, number_index, current_block), result);
            return result;
        }

        // if the char is a known, and is working spring
        if self.configuration.as_bytes()[char_index] == b'.' {
            // the current block must end, and the next block must start
            if number_index >= self.numbers.len() || current_block == self.numbers[number_index] {
                let result = self.permute(char_index + 1, number_index + 1, 0);
                self.dp
                    .insert((char_index, number_index, current_block), result);
                return result;
            }

            // the current block hasn't begun, we begin it later
            if current_block == 0 {
                let result = self.permute(char_index + 1, number_index, current_block);
                self.dp
                    .insert((char_index, number_index, current_block), result);
                return result;
            }

            // the current block is not enough, this permutation is not valid
            self.dp.insert((char_index, number_index, current_block), 0);
            return 0;
        }

        // if the char is a known, and is broken spring
        if self.configuration.as_bytes()[char_index] == b'#' {
            // the current block must be less than the number
            if number_index < self.numbers.len() && current_block < self.numbers[number_index] {
                let result = self.permute(char_index + 1, number_index, current_block + 1);
                self.dp
                    .insert((char_index, number_index, current_block), result);
                return result;
            }

            // the current block is too much, this permutation is not valid
            self.dp.insert((char_index, number_index, current_block), 0);
            return 0;
        }

        // if the char is unknown
        // first case, we set the unknown to be a working spring
        // the current block ends, we move to the next block
        let mut first_perm = 0;
        let mut second_perm = 0;
        if number_index >= self.numbers.len() || current_block == self.numbers[number_index] {
            first_perm = self.permute(char_index + 1, number_index + 1, 0);
        } else if current_block == 0 {
            first_perm = self.permute(char_index + 1, number_index, 0)
        }

        // second case, we set the unknown to be a broken spring
        // the current block is less than the number
        if number_index < self.numbers.len() && current_block < self.numbers[number_index] {
            second_perm = self.permute(char_index + 1, number_index, current_block + 1);
        }
        let result = first_perm + second_perm;
        self.dp
            .insert((char_index, number_index, current_block), result);
        result
    }
}

impl From<String> for SpringConfig {
    fn from(value: String) -> Self {
        let mut split = value.split(' ');
        let configuration: String = split.next().unwrap().into();
        let numbers = split
            .next()
            .unwrap()
            .split(',')
            .into_iter()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
       
        Self {
            numbers,
            configuration,
            dp: HashMap::new(),
        }
    }
}

fn run(input_file: &str) {
   // Preamble
   let mut result: usize = 0;
   // Parse
   let mut values: Vec<SpringConfig> = parse_file_into(input_file);

   for value in values.iter_mut() {
       result += value.permute(0, 0, 0);
   }

   // Result
   println!("Result of part 2 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result: usize = 0;
    // Parse
    let mut values: Vec<SpringConfig> = parse_file_into(input_file);

    for value in values.iter_mut() {
        value.extend(4);
        result += value.permute(0, 0, 0);
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
