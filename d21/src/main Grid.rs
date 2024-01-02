use std::collections::{HashMap, HashSet};

use utils::{get_input_path, grid::Grid, grid_point::GridPoint, parse_into_char_vector_vector};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct WalkState {
    position: GridPoint,
    steps: usize,
}
impl WalkState {
    fn new(start_point: GridPoint) -> WalkState {
        Self {
            position: start_point,
            steps: 0,
        }
    }
}

#[derive(Debug)]
struct Walk {
    duration: usize,
    grid: Grid<char>,
    db: HashMap<WalkState, HashSet<GridPoint>>,
    max_x: usize,
    max_y: usize,
}
impl Walk {
    fn step(&mut self, state: WalkState) -> HashSet<GridPoint> {
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
            .generate_non_diagonal_neigbors_with_check(self.max_x, self.max_y);

        let next_step = state.steps + 1;

        for next in next_points {
            if self.grid.data[next.y][next.x] == '.' {
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

    // Parse
    let mut grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    let start_point: GridPoint = grid.find(|c| *c == 'S').unwrap();

    grid.set_from_point(&start_point, '.');
    let start: WalkState = WalkState::new(start_point);
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();
    let mut walk: Walk = Walk {
        duration: STEPS,
        grid: grid,
        db: HashMap::new(),
        max_x,
        max_y,
    };

    // Solve
    let result = walk.step(start);

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
