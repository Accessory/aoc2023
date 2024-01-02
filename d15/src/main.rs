use std::fs;

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut result: usize = 0;
    // Parse
    let items: Vec<String> = fs::read_to_string(input_file)
        .unwrap()
        .split(',')
        .map(|f| f.trim().into())
        .collect();
    // Solve

    for hash in items {
        let mut hash_value: usize = 0;
        for c in hash.as_bytes().iter() {
            hash_value += *c as usize;
            hash_value *= 17;
            hash_value %= 256;
        }
        result += hash_value;
    }

    // Result
    println!("Result of part 1 is {}", result);
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Dash,
    Equils,
}

#[derive(Debug, Clone)]
struct Item {
    value: String,
    label: String,
    operation: Operation,
    hash: u8,
}
impl Item {
    fn get_focal_length(&self) -> usize {
        self.value.chars().last().unwrap().to_digit(10).unwrap() as usize
    }
}

impl From<&str> for Item {
    fn from(value: &str) -> Self {
        let operation = if value.chars().last().unwrap() == '-' {
            Operation::Dash
        } else {
            Operation::Equils
        };

        let label = match operation {
            Operation::Dash => value[0..value.len() - 1].to_string(),
            Operation::Equils => value[0..value.len() - 2].to_string(),
        };

        let mut hash_value: usize = 0;
        for c in label.as_bytes().iter() {
            hash_value += *c as usize;
            hash_value *= 17;
            hash_value %= 256;
        }

        Self {
            value: value.into(),
            label,
            operation,
            hash: hash_value as u8,
        }
    }
}

fn run2(input_file: &str) {
    // Preamble
    let mut result: usize = 0;
    let mut boxes: Vec<Vec<Item>> = vec![Vec::new(); 256];
    // Parse
    let items: Vec<Item> = fs::read_to_string(input_file)
        .unwrap()
        .split(',')
        .map(|f| f.trim().into())
        .collect();

    // Solve
    for item in items {
        match item.operation {
            Operation::Dash => dash_operation(&mut boxes, item),
            Operation::Equils => equils_operation(&mut boxes, item),
        }
    }

    // Result
    for (box_slot, b) in boxes.iter().enumerate() {
        for (item_slot, item) in b.iter().enumerate() {
            result += (box_slot+1) * (item_slot+1) * item.get_focal_length();
        }
    }

    println!("Result of part 2 is {}", result);
}

fn equils_operation(boxes: &mut [Vec<Item>], item: Item) {
    let current_box = &mut boxes[item.hash as usize];

    if let Some((idx, _)) = current_box
        .iter()
        .enumerate()
        .find(|i| i.1.label == item.label)
    {
        current_box[idx] = item;
    } else {
        current_box.push(item);
    }
}

fn dash_operation(boxes: &mut [Vec<Item>], item: Item) {
    let current_box = &mut boxes[item.hash as usize];
    if let Some((idx, _)) = current_box
        .iter()
        .enumerate()
        .find(|i| i.1.label == item.label)
    {
        current_box.remove(idx);
    }
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
