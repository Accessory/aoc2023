use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use utils::{get_input_path, hash_point_map::HashPointMap, point::MapPoint};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct WalkState {
    position: MapPoint,
    steps: usize,
}
impl WalkState {
    fn new(start_point: MapPoint) -> WalkState {
        Self {
            position: start_point,
            steps: 0,
        }
    }
}

#[derive(Debug)]
struct Walk {
    duration: usize,
    grid: HashPointMap<char>,
    db: HashMap<WalkState, HashSet<MapPoint>>,
    max_x: i64,
    max_y: i64,
}
impl Walk {
    fn step(&mut self, state: WalkState) -> HashSet<MapPoint> {
        if let Some(value) = self.db.get(&state) {
            return value.clone();
        }

        if state.steps == self.duration {
            self.db.insert(state, [state.position].into());
            return [state.position].into();
        }

        let mut inner_result = HashSet::new();

        let next_points = state
            .position
            .generate_non_diagonal_neigbors_with_check(0, self.max_x, 0, self.max_y);

        let next_step = state.steps + 1;

        for next in next_points {
            if !self.grid.contains(&next) {
                inner_result.extend(self.step(WalkState {
                    position: next,
                    steps: next_step,
                }));
            }
        }

        self.db.insert(state, inner_result.clone());

        inner_result
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

    for y in 0..max_y{
        f 
    }

 

    // Result
    println!("Result of part 1 is {}", result.len());
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
