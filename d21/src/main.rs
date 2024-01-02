use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use utils::{get_input_path, hash_point_map::HashPointMap, point::MapPoint};

#[derive(Debug)]
struct Walk {
    steps: Vec<usize>,
    map: HashPointMap<char>,
    max_x: i64,
    max_y: i64,
}
impl Walk {
    fn go(&mut self, start: MapPoint) -> Vec<usize> {
        let mut seen: HashMap<MapPoint, usize> = HashMap::new();
        seen.insert(start, 0);
        let mut dist = 0;

        let mut queue: VecDeque<MapPoint> = VecDeque::from([start]);

        let mut result = Vec::new();
        let max_steps = *self.steps.iter().max().unwrap();

        while dist < max_steps {
            dist += 1;
            let mut queue2: VecDeque<MapPoint> = VecDeque::new();
            while let Some(point) = queue.pop_front() {
                let neighbors = point.generate_non_diagonal_neigbors();
                for neighbor in neighbors {
                    let adjusted_neighbor = MapPoint::new(
                        neighbor.x.rem_euclid(self.max_x),
                        neighbor.y.rem_euclid(self.max_y),
                    );

                    if !self.map.contains(&adjusted_neighbor) && !seen.contains_key(&neighbor) {
                        seen.insert(neighbor, dist);
                        queue2.push_back(neighbor);
                    }
                }
            }
            queue = queue2;
            if self.steps.contains(&dist) {
                let value = seen.iter().filter(|s| s.1 % 2 == dist % 2).count();
                result.push(value);
            }
        }

        result
    }
}

fn run(input_file: &str) {
    // Preamble
    #[cfg(test)]
    const STEPS: usize = 6;

    #[cfg(not(test))]
    const STEPS: usize = 64;

    let mut map: HashPointMap<char> = HashPointMap::default();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut start_point: MapPoint = MapPoint::default();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();

        max_y = max_y.max(y);

        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x);
            match c {
                '#' => {
                    map.insert(MapPoint::new(x as i64, y as i64), '#');
                }
                'S' => {
                    start_point = MapPoint::new(x as i64, y as i64);
                }
                _ => {}
            }
        }
    }

    let mut walk = Walk {
        steps: vec![STEPS],
        map,
        max_x: max_x as i64 + 1,
        max_y: max_y as i64 + 1,
    };

    // Solve
    let result = walk.go(start_point);

    // Result
    println!("Result of part 1 is {}", result.first().unwrap());
}

fn run2(input_file: &str) {
    // Preamble
    // Grid is quadratic (SIDE x SIDE) (131 x 131)
    // We start in the middle (65, 65)
    // filled area increase will cycle with a period of SIDE
    // So we can fit a 2nd degree polynomial f(n)
    // Let n=0 be SIDE // 2 steps, n=1 is SIDE // 2 + SIDE
    // n=3 is SIDE // 2 + 2*SIDE
    // 26501365 - 65 happens to be divisible by 131
    const STEPS: usize = 26501365;

    let mut map: HashPointMap<char> = HashPointMap::default();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut start_point: MapPoint = MapPoint::default();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();

        max_y = max_y.max(y);

        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x);
            match c {
                '#' => {
                    map.insert(MapPoint::new(x as i64, y as i64), '#');
                }
                'S' => {
                    start_point = MapPoint::new(x as i64, y as i64);
                }
                _ => {}
            }
        }
    }

    let half = start_point.x as usize;
    let side = max_x + 1;

    let mut walk = Walk {
        steps: vec![half, half + side, half + 2 * side],
        map,
        max_x: max_x as i64 + 1,
        max_y: max_y as i64 + 1,
    };

    // Solve
    let result = walk.go(start_point);

    let f0 = result[0];
    let f1 = result[1];
    let f2 = result[2];

    let c = f0;
    let a = (f2 - 2 * f1 + f0) / 2;
    let b = f1 - f0 - a;
    let n = (STEPS - half) / side;

    let final_result = a * n.pow(2) + b * n + c;

    // Result 612941134797232
    println!("Result of part 1 is {}", final_result);
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
    use utils::get_input_path;
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
        let input_path = get_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
