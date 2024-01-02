use std::collections::HashMap;

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

    let mut queue: Vec<State> = vec![State::new(START)];

    // Solve
    let mut result = Vec::new();
    while let Some(state) = queue.pop() {
        if state.position == goal {
            result.push(state);
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

        for next in next_posibilities {
            let next_state = State {
                position: next,
                steps: next_step,
                way: next_way.clone(),
            };
            queue.push(next_state);
        }
    }

    let max_result = result.iter().map(|r| r.steps).max().unwrap();
    println!("Result of part 1 is {}", max_result);
}

#[derive(Debug)]
struct Connection {
    start: GridPoint,
    ends: Vec<(GridPoint, usize)>,
}

fn run2(input_file: &str) {
    // Preamble
    const START: GridPoint = GridPoint { x: 1, y: 0 };
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();
    let goal = GridPoint::new(max_x - 2, max_y - 1);

    let mut intersetions: Vec<GridPoint> = vec![START, goal];

    for y in 1..max_y - 1 {
        for x in 1..max_x - 1 {
            let current_position = GridPoint::new(x, y);
            if grid
                .get_from_point(&current_position)
                .is_some_and(|c| *c == '#')
            {
                continue;
            }

            let neigbors_count = current_position
                .generate_non_diagonal_neigbors()
                .iter()
                .filter(|p| grid.get_from_point(&p).is_some_and(|c| *c != '#'))
                .count();

            if neigbors_count > 2 {
                intersetions.push(current_position);
            }
        }
    }

    let mut connections: Vec<Connection> = Vec::new();

    // Find connection
    for intersection in intersetions.iter() {
        if intersection != &goal {
            let connection: Connection = find_connections(*
            intersection, &intersetions, &grid);
            connections.push(connection);
        }
    }

    let connection_map: HashMap<GridPoint, Connection> =
        connections.into_iter().map(|c| (c.start, c)).collect();

    let mut queue = vec![GState {
        position: START,
        preview: Vec::new(),
        steps: 0,
    }];

    let mut results = Vec::new();

    while let Some(mut state) = queue.pop() {
        if state.position == goal {
            results.push(state);
            continue;
        }

        state.preview.push(state.position);

        let next_connections = connection_map.get(&state.position).unwrap();

        for end in next_connections.ends.iter() {
            if state.preview.contains(&end.0) {
                continue;
            }

            let next_state = GState {
                position: end.0,
                preview: state.preview.clone(),
                steps: state.steps + end.1,
            };
            queue.push(next_state);
        }
    }

    let result = results.iter().max_by_key(|s| s.steps).unwrap();

    println!("Result of part 2 is {}", result.steps);
}

struct GState {
    position: GridPoint,
    preview: Vec<GridPoint>,
    steps: usize,
}

struct FcState {
    position: GridPoint,
    preview: Vec<GridPoint>,
}

fn find_connections(start: GridPoint, intersetions: &[GridPoint], grid: &Grid<char>) -> Connection {
    let neigbors = start
        .generate_non_diagonal_neigbors()
        .into_iter()
        .filter(|p| {
            let char = *grid.get_from_point(p).unwrap();
            char != '#'
        });
    let mut ends = Vec::new();

    for neigbor in neigbors {
        let mut state = FcState {
            position: neigbor,
            preview: vec![start],
        };
        loop {
            if intersetions.contains(&state.position) {
                break;
            }

            let next_neigbors = state
                .position
                .generate_non_diagonal_neigbors()
                .into_iter()
                .find(|c| {
                    let char = *grid.get_from_point(c).unwrap();
                    char != '#' && !state.preview.contains(c)
                })
                .unwrap();

            state.preview.push(state.position);
            state.position = next_neigbors;
        }
        ends.push((state.position, state.preview.len()));
    }

    Connection { start, ends }
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
