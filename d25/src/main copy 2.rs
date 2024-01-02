use std::collections::HashMap;
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
    // let mut result_groups = None;
    let mut results = None;
    'outer: for (i1, e1) in connections.iter().enumerate() {
        println!(
            "Start {} of {}, - {}",
            i1 + 1,
            connections.len(),
            i1 as f32 * 100.0 / connections.len() as f32
        );
        for (i2, e2) in connections.iter().enumerate().skip(i1 + 1) {
            for e3 in connections.iter().skip(i2 + 1) {
                // let e1 = ("hfx".to_string(), "pzl".to_string());
                // let e2 = ("bvb".to_string(), "cmg".to_string());
                // let e3 = ("nvd".to_string(), "jqt".to_string());

                let disconnected = vec![e1.clone(), e2.clone(), e3.clone()];
                let (gc, g1, g2) = check_groups_count(&entries, &map, disconnected);
                if gc == 2 {
                    // result_groups = Some((e1, e2, e3));
                    results = Some((gc, g1, g2));
                    break 'outer;
                }
            }
        }
    }

    // dbg!(results);
    // dbg!(result_groups);

    // Result
    let result = results.unwrap().1 * results.unwrap().2;

    println!("Result of part 1 is {}", result);
}

fn check_groups_count(
    entries: &[String],
    map: &HashMap<String, Vec<String>>,
    disconnected: Vec<(String, String)>,
) -> (usize, usize, usize) {
    let mut rtn: usize = 0;
    let mut g1: usize = 0;
    let mut g2: usize = 0;
    let mut seen: Vec<&String> = Vec::new();
    for start in entries.iter() {
        if !seen.contains(&start) {
            rtn += 1;
            // Early exit
            if rtn == 3 {
                return (3, 0, 0);
            }

            let mut queue = vec![start];
            let mut count = 0;
            while let Some(current) = queue.pop() {
                if seen.contains(&current) {
                    continue;
                }
                seen.push(current);
                count += 1;
                let nexts = map.get(current).unwrap();

                for next in nexts {
                    if !disconnected.contains(&(current.to_string(), next.to_string()))
                        && !disconnected.contains(&(next.to_string(), current.to_string()))
                    {
                        queue.push(next);
                    }
                    //  else {
                    //     println!("{} - {}", current, next);
                    // }
                }
            }
            if g1 == 0 {
                g1 = count;
            } else {
                g2 = count;
            }
        }
    }

    (rtn, g1, g2)
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
