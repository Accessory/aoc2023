#![feature(iter_advance_by)]
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::{get_input_path, lcm};

#[derive(Debug)]
enum LeftRight {
    Left,
    Right,
}

impl From<char> for LeftRight {
    fn from(value: char) -> Self {
        match value {
            'L' => LeftRight::Left,
            'R' => LeftRight::Right,
            _ => panic!("Should not be here"),
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    const START: &str = "AAA";
    const GOAL: &str = "ZZZ";
    let mut ways: HashMap<String, (String, String)> = HashMap::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let left_rights: Vec<LeftRight> = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c.into())
        .collect();
    lines.advance_by(1).unwrap();
    for line in lines {
        let line = line.unwrap().trim().to_string();
        let node = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();

        ways.insert(node, (left, right));
    }

    // Solve
    let mut current_node = START;
    let lr_pos = left_rights.iter().cycle();
    let mut result: usize = 0;

    for lr in lr_pos {
        if current_node == GOAL {
            break;
        }
        result += 1;

        let possible_ways = ways.get(current_node).expect("This is not the way!");

        current_node = match lr {
            LeftRight::Left => &possible_ways.0,
            LeftRight::Right => &possible_ways.1,
        }
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut ways: HashMap<String, (String, String)> = HashMap::new();
    let mut current_nodes: Vec<String> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let left_rights: Vec<LeftRight> = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c.into())
        .collect();
    lines.advance_by(1).unwrap();
    for line in lines {
        let line = line.unwrap().trim().to_string();
        let node = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();

        if node.as_bytes()[2] == b'A' {
            current_nodes.push(node.clone());
        }

        ways.insert(node, (left, right));
    }

    // Solve
    let lr_pos = left_rights.iter().cycle();
    let mut results = Vec::new();
    let mut rounds: usize = 0;

    for lr in lr_pos {
        let mut next_nodes = Vec::with_capacity(current_nodes.len());
        for node in current_nodes.iter() {
            let possible_ways = ways.get(node).expect("This is not the way!");

            let next_node = match lr {
                LeftRight::Left => &possible_ways.0,
                LeftRight::Right => &possible_ways.1,
            };
            next_nodes.push(next_node.clone());
        }

        rounds += 1;

        current_nodes.clear();

        for i in (0..next_nodes.len()).rev() {
            if next_nodes[i].as_bytes()[2] == b'Z' {
                results.push(rounds);
            } else {
                current_nodes.push(next_nodes[i].clone());
            }
        }

        if current_nodes.is_empty() {
            break;
        }
    }

    let mut result = results[0];

    for value in results.iter().skip(1){
        result = lcm(result, *value);
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
