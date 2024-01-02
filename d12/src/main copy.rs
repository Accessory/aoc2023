use std::collections::HashMap;

use utils::{get_input_path, parse_file_into};

#[derive(Debug)]
struct SpringGroup {
    line: String,
    pattern: Vec<usize>,
}

#[derive(Debug, Clone)]
struct PatternState {
    line: String,
    unkowns: Vec<usize>,
    damage: usize,
}

fn calc_damage(line: &str) -> usize {
    line.chars().filter(|c| *c == '#').count()
}

impl PatternState {
    fn new(line: String, unkowns: Vec<usize>) -> Self {
        let damage = calc_damage(&line);
        Self {
            line,
            unkowns,
            damage,
        }
    }

    fn to_pattern(&self) -> Vec<usize> {
        let mut rtn = Vec::new();
        let mut last_group_size = 0;
        let mut in_group = false;
        for c in self.line.chars() {
            if !in_group && c == '#' {
                in_group = true;
                last_group_size = 1;
                continue;
            }

            if in_group && c != '#' {
                in_group = false;
                rtn.push(last_group_size);
                last_group_size = 0;
                continue;
            }

            if in_group && c == '#' {
                last_group_size += 1;
            }
        }

        if last_group_size != 0 {
            rtn.push(last_group_size);
        }

        rtn
    }

    fn to_current_pattern(&self) -> Vec<usize> {
        let mut rtn = Vec::new();
        let mut last_group_size = 0;
        let mut in_group = false;
        let till = *self.unkowns.first().unwrap_or(&self.line.len());
        for c in self.line[0..till].chars() {
            if !in_group && c == '#' {
                in_group = true;
                last_group_size = 1;
                continue;
            }

            if in_group && c != '#' {
                in_group = false;
                rtn.push(last_group_size);
                last_group_size = 0;
                continue;
            }

            if in_group && c == '#' {
                last_group_size += 1;
            }
        }

        if last_group_size != 0 {
            rtn.push(last_group_size);
        }

        rtn
    }
}

impl SpringGroup {
    fn calculate_arragements(&self) -> usize {
        // let groups = self.to_groups();
        let unkowns = self.get_unkowns();
        let mut rtn: usize = 0;

        let max_damage: usize = self.pattern.iter().sum();

        let mut queue: Vec<PatternState> =
            vec![PatternState::new(self.line.clone(), unkowns.clone())];

        while let Some(mut state) = queue.pop() {
            if state.unkowns.len() == 0 {
                if self.validate(&state) {
                    rtn += 1;
                }
                continue;
            }

            if !self.is_possible(&state) {
                continue;
            }

            let unknown = state.unkowns.remove(0);

            unsafe {
                let mut next_1 = state.clone();
                next_1.line.as_bytes_mut()[unknown] = b'.';
                queue.push(next_1);
                if state.damage <= max_damage {
                    let mut next_2 = state.clone();
                    next_2.line.as_bytes_mut()[unknown] = b'#';
                    queue.push(next_2);
                }
            }
        }

        rtn
    }

    fn get_unkowns(&self) -> Vec<usize> {
        self.line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '?')
            .map(|(i, _)| i)
            .collect()
    }

    fn validate(&self, state: &PatternState) -> bool {
        let pattern = state.to_pattern();
        self.pattern == pattern
    }

    fn is_possible(&self, state: &PatternState) -> bool {
        let state_pattern = state.to_current_pattern();

        let min_len = state_pattern.len().min(self.pattern.len());
        if min_len == 0 {
            return true;
        }

        let last = min_len - 1;

        for i in 0..min_len {
            if i == last {
                if self.pattern[i] < state_pattern[i] {
                    // println!("{} - {}", self.line, state.line);
                    state.to_current_pattern();
                    return false;
                }
            } else {
                if self.pattern[i] != state_pattern[i] {
                    // println!("{} - {}", self.line, state.line);
                    return false;
                }
            }
        }

        true
    }

    fn return_arragements(&self) -> Vec<String> {
        // let groups = self.to_groups();
        let unkowns = self.get_unkowns();
        let mut rtn = Vec::new();

        let max_damage: usize = self.pattern.iter().sum();

        let mut queue: Vec<PatternState> =
            vec![PatternState::new(self.line.clone(), unkowns.clone())];

        while let Some(mut state) = queue.pop() {
            if state.unkowns.len() == 0 {
                if self.validate(&state) {
                    rtn.push(state.line.clone());
                }
                continue;
            }

            if !self.is_possible(&state) {
                continue;
            }

            let unknown = state.unkowns.remove(0);

            unsafe {
                let mut next_1 = state.clone();
                next_1.line.as_bytes_mut()[unknown] = b'.';
                queue.push(next_1);
                if state.damage <= max_damage {
                    let mut next_2 = state.clone();
                    next_2.line.as_bytes_mut()[unknown] = b'#';
                    queue.push(next_2);
                }
            }
        }

        rtn
    }

    fn validate_arragements(&self, arragenents: Vec<String>, arragenents2: Vec<String>) -> usize {
        let long_pattern = self.pattern.repeat(5);
        // let mut rtn = Vec::new();
        let mut rtn = 0;

        for a1 in arragenents.iter() {
            for a2 in arragenents2.iter() {
                for a3 in arragenents2.iter() {
                    for a4 in arragenents2.iter() {
                        for a5 in arragenents2.iter() {
                            let to_check = format!("{}{}{}{}{}", a1, a2, a3, a4, a5);
                            if validate_pattern(&to_check, &long_pattern) {
                                rtn += 1;
                            }
                        }
                    }
                }
            }
        }

        rtn
    }
}

fn validate_pattern(to_check: &str, long_pattern: &[usize]) -> bool {
    let mut check_pattern = Vec::new();
    let mut last_group_size = 0;
    let mut in_group = false;
    for c in to_check.chars() {
        if !in_group && c == '#' {
            in_group = true;
            last_group_size = 1;
            continue;
        }

        if in_group && c != '#' {
            in_group = false;
            check_pattern.push(last_group_size);
            last_group_size = 0;
            continue;
        }

        if in_group && c == '#' {
            last_group_size += 1;
        }
    }

    if last_group_size != 0 {
        check_pattern.push(last_group_size);
    }

    check_pattern == long_pattern
}

impl From<String> for SpringGroup {
    fn from(value: String) -> Self {
        let mut split = value.split(' ');
        let line = split.next().unwrap().into();
        let pattern = split
            .next()
            .unwrap()
            .split(',')
            .into_iter()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        Self { line, pattern }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut result: usize = 0;
    // Parse
    let values: Vec<SpringGroup> = parse_file_into(input_file);

    // Solve
    for value in values {
        result += value.calculate_arragements();
    }

    // Result
    println!("Result of part 1 is {}", result);
}

#[derive(Debug)]
struct SpringConfig {
    numbers: Vec<usize>,
    configuration: String,
    dp: HashMap<(usize, usize, usize), usize>,
}
impl SpringConfig {
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
        let mut configuration: String = split.next().unwrap().into();
        let numbers = split
            .next()
            .unwrap()
            .split(',')
            .into_iter()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .repeat(5);

        let to_add = format!("?{}", configuration);
        for _ in 0..4 {
            configuration.push_str(&to_add);
        }

        Self {
            numbers,
            configuration,
            dp: HashMap::new(),
        }
    }
}

fn run2(input_file: &str) {
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

fn _run2(input_file: &str) {
    // Preamble
    let mut result: usize = 0;
    // Parse
    let mut values: Vec<SpringGroup> = parse_file_into(input_file);
    // for value in values.iter_mut() {
    //     let to_add = format!("?{}", value.line);
    //     let to_add_2 = value.pattern.clone();
    //     for _ in 0..4 {
    //         value.line.push_str(&to_add);
    //         value.pattern.extend(to_add_2.iter());
    //     }
    // }

    // Solve
    let _len = values.len();
    for value in values.iter_mut() {
        // println!("Start {} of {}", idx + 1, len);
        let arragenents = value.return_arragements();
        value.line.insert(0, '?');
        let arragenents2 = value.return_arragements();

        let to_add = value.validate_arragements(arragenents, arragenents2);
        result += to_add;
        println!("Line: {} - Result: {}", value.line, to_add);
    }

    // Result
    println!("Result of part 2 is {}", result);
}

fn _run2old(input_file: &str) {
    // Preamble
    let mut result: usize = 0;
    // Parse
    let mut values: Vec<SpringGroup> = parse_file_into(input_file);
    for value in values.iter_mut() {
        let to_add = format!("?{}", value.line);
        let to_add_2 = value.pattern.clone();
        for _ in 0..4 {
            value.line.push_str(&to_add);
            value.pattern.extend(to_add_2.iter());
        }
    }

    // Solve
    for (idx, value) in values.iter().enumerate() {
        println!("Start {} of {}", idx + 1, values.len());
        result += value.calculate_arragements();
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
