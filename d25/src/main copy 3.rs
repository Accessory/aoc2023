use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut entries = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut connections = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split([':', ' ']).filter(|s| !s.is_empty()).collect();
        entries.push(split[0].to_string());
        {
            let entry = map.entry(split[0].to_string()).or_default();

            for s in split.iter().skip(1) {
                entry.push(s.to_string());
                connections.push((split[0].to_string(), s.to_string()));
            }
        }
        for s in split.iter().skip(1) {
            let entry = map.entry(s.to_string()).or_default();
            entry.push(split[0].to_string());
            // connections.push((s.to_string(), split[0].to_string()));
        }
    }

    // Solve
    let mut results = Vec::new();
    for (i, e1) in entries.iter().enumerate() {
        for e2 in entries.iter().skip(i + 1) {
            let distance = get_distance(e1, e2, &map);
            results.push((e1, e2, distance));
        }
    }

    results.sort_by_key(|f| f.2);

    dbg!(results);

    // Result

    println!("Result of part 1 is {}", 2);
}

#[derive(Debug, PartialEq, Eq)]
struct DState {
    current: String,
    distance: usize,
}

// impl Ord for DState {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.distance.cmp(&other.distance)
//     }
// }

// impl PartialOrd for DState {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

fn get_distance(e1: &str, e2: &str, map: &HashMap<String, Vec<String>>) -> usize {
    let mut seen = HashSet::new();

    let mut queue = VecDeque::from([DState {
        current: e1.to_string(),
        distance: 0,
    }]);

    while let Some(current) = queue.pop_front() {

        if current.current == e2{
            return current.distance;
        }


        if !seen.insert(current.current.clone()) {
            continue;
        }

        let next_distance = current.distance + 1;

        for next in map.get(&current.current).unwrap() {
            queue.push_back(DState {
                current: next.clone(),
                distance: next_distance,
            });
        }
    }

    0
}

fn run2(_input_file: &str) {}

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
