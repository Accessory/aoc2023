use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut entries: HashSet<String> = HashSet::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split([':', ' ']).filter(|s| !s.is_empty()).collect();
        entries.extend(split.clone().iter().map(|i| i.to_string()));
        {
            let entry = map.entry(split[0].to_string()).or_default();

            for s in split.iter().skip(1) {
                entry.push(s.to_string());
            }
        }
        for s in split.iter().skip(1) {
            let entry = map.entry(s.to_string()).or_default();
            entry.push(split[0].to_string());
        }
    }

    // Solve
    let mut start_group: HashSet<String> = entries.clone();

    while start_group
        .iter()
        .map(|s| count(&map, s, &start_group))
        .sum::<usize>()
        != 3
    {
        let value = start_group
            .iter()
            .max_by_key(|s| count(&map, s, &start_group))
            .unwrap()
            .clone();
        start_group.remove(&value);
    }
    let result = start_group.len() * (entries.len() - start_group.len());

    println!("Result of part 1 is {}", result);
}

fn count(map: &HashMap<String, Vec<String>>, s: &String, start_group: &HashSet<String>) -> usize {
    let result = map
        .get(s)
        .unwrap()
        .iter()
        .filter(|i| !start_group.contains(i.as_str()))
        .count();
    result
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }
}
