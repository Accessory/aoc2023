use std::collections::{HashMap, VecDeque};

use utils::{get_input_path, grid::Grid, grid_point::GridPoint, parse_into_char_vector_vector};

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    position: GridPoint,
    steps: usize,
    way: Vec<GridPoint>,
}

impl State {
    fn new(position: GridPoint) -> Self {
        Self {
            position,
            steps: 0,
            way: Vec::new(),
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    const START: GridPoint = GridPoint { x: 1, y: 0 };
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();
    let goal = GridPoint::new(max_x - 2, max_y - 1);

    let mut queue: VecDeque<State> = VecDeque::from([State::new(START)]);

    // Solve
    let mut result = Vec::new();
    let mut connections: HashMap<GridPoint, (GridPoint, Vec<GridPoint>)> = HashMap::new();
    let mut line: Option<(GridPoint, Vec<GridPoint>)> = None;
    while let Some(state) = queue.pop_back() {
        if state.position == goal {
            if let Some(l) = line {
                let start = l.0;
                let end = state.position;
                let steps = l.1;
                connections.insert(start, (end, steps));
                line = None;
            }
            result.push(state);
            continue;
        }

        if let Some(connection) = connections.get(&state.position) {
            let mut next_way = state.way.clone();
            next_way.extend(connection.1.iter());
            let next_state = State {
                position: connection.0,
                steps: state.steps + connection.1.len(),
                way: next_way,
            };
            queue.push_back(next_state);
            line = None;
            continue;
        }

        let mut next_posibilities = match *grid.get_from_point(&state.position).unwrap() {
            '>' => vec![state.position.next_right()],
            '<' => vec![state.position.next_left()],
            'v' => vec![state.position.next_down()],
            '^' => vec![state.position.next_up()],
            '.' => state
                .position
                .generate_non_diagonal_neigbors_with_check(max_x, max_y),

            _ => {
                line = None;
                continue;
            }
        };

        let next_step = state.steps + 1;
        let mut next_way = state.way.clone();
        next_way.push(state.position);

        next_posibilities.retain(|i| {
            let c = *grid.get_from_point(i).unwrap();
            c != '#' && !state.way.contains(i)
        });

        if next_posibilities.len() == 1 {
            if line.is_none() {
                line = Some((state.position, vec![]));
            } else {
                line.as_mut().unwrap().1.push(state.position);
            }
        } else {
            if let Some(l) = line {
                let start = l.0;
                let end = state.position;
                let steps = l.1;
                connections.insert(start, (end, steps));
                line = None;
            }
        }

        for next in next_posibilities {
            let next_state = State {
                position: next,
                steps: next_step,
                way: next_way.clone(),
            };
            queue.push_back(next_state);
        }
    }

    // Result to low 4946
    let max_result = result.iter().map(|r| r.steps).max().unwrap();
    println!("Result of part 1 is {}", max_result);
}

fn run2(input_file: &str) {
    // Preamble
    const START: GridPoint = GridPoint { x: 1, y: 0 };
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();
    let goal = GridPoint::new(max_x - 2, max_y - 1);

    let mut queue: VecDeque<State> = VecDeque::from([State::new(START)]);

    // Solve
    let mut result = Vec::new();
    let mut connections: HashMap<GridPoint, (GridPoint, Vec<GridPoint>)> = HashMap::new();
    let mut line: Option<(GridPoint, Vec<GridPoint>)> = None;
    while let Some(state) = queue.pop_back() {
        if state.position == goal {
            if let Some(l) = line {
                let start = l.0;
                let end = state.position;
                let steps = l.1;
                connections.insert(start, (end, steps));
                line = None;
            }
            println!("{}",state.steps);
            result.push(state);
            continue;
        }

        if let Some(connection) = connections.get(&state.position) {
            let mut next_way = state.way.clone();
            next_way.extend(connection.1.iter());
            let next_state = State {
                position: connection.0,
                steps: state.steps + connection.1.len() + 1,
                way: next_way,
            };
            queue.push_back(next_state);
            line = None;
            continue;
        }

        let mut next_posibilities = match *grid.get_from_point(&state.position).unwrap() {
            '>' | '<' | 'v' | '^' | '.' => state
                .position
                .generate_non_diagonal_neigbors_with_check(max_x, max_y),
            _ => {
                line = None;
                continue;
            }
        };

        let next_step = state.steps + 1;
        let mut next_way = state.way.clone();
        next_way.push(state.position);

        next_posibilities.retain(|i| {
            let c = *grid.get_from_point(i).unwrap();
            c != '#' && !state.way.contains(i)
        });

        if next_posibilities.len() == 0 {
            line = None;
            continue;
        }

        if next_posibilities.len() == 1 {
            if line.is_none() {
                line = Some((state.position, vec![]));
            } else {
                line.as_mut().unwrap().1.push(state.position);
            }
        } else {
            if let Some(l) = line {
                let start = l.0;
                let end = state.position;
                let steps = l.1;
                connections.insert(start, (end, steps));
                line = None;
            }
        }

        for next in next_posibilities {
            let next_state = State {
                position: next,
                steps: next_step,
                way: next_way.clone(),
            };
            queue.push_back(next_state);
        }
    }

    // Result to low 4946
    let max_result = result.iter().map(|r| r.steps).max().unwrap();
    println!("Result of part 2 is {}", max_result);
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
