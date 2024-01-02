use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use geo::{Contains, LineString, Point, Polygon};
use utils::{get_input_path, grid::Grid, grid_direction::GridDirection, grid_point::GridPoint};

fn next_direction(next_value: &char, current_direction: GridDirection) -> GridDirection {
    match *next_value {
        '|' => current_direction,
        '-' => current_direction,
        'L' => match current_direction {
            GridDirection::Down => GridDirection::Right,
            GridDirection::Left => GridDirection::Up,
            _ => panic!("Should not be here"),
        },
        'J' => match current_direction {
            GridDirection::Down => GridDirection::Left,
            GridDirection::Right => GridDirection::Up,
            _ => panic!("Should not be here"),
        },
        '7' => match current_direction {
            GridDirection::Up => GridDirection::Left,
            GridDirection::Right => GridDirection::Down,
            _ => panic!("Should not be here"),
        },
        'F' => match current_direction {
            GridDirection::Up => GridDirection::Right,
            GridDirection::Left => GridDirection::Down,
            _ => panic!("Should not be here"),
        },
        _ => panic!("Should not be here."),
    }
}

fn is_valid_move(next_value: Option<&char>, last_direction: GridDirection) -> bool {
    match next_value {
        Some(c) => match c {
            '|' => last_direction == GridDirection::Up || last_direction == GridDirection::Down,
            '-' => last_direction == GridDirection::Right || last_direction == GridDirection::Left,
            'L' => last_direction == GridDirection::Down || last_direction == GridDirection::Left,
            'J' => last_direction == GridDirection::Down || last_direction == GridDirection::Right,
            '7' => last_direction == GridDirection::Up || last_direction == GridDirection::Right,
            'F' => last_direction == GridDirection::Up || last_direction == GridDirection::Left,
            '.' => false,
            _ => panic!("Should not be here."),
        },
        None => false,
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut result = 0;
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut start_position: GridPoint = GridPoint::default();
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (y, line) in reader.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.unwrap().trim().chars().enumerate() {
            if c == 'S' {
                start_position.x = x;
                start_position.y = y;
            }
            row.push(c);
        }
        data.push(row);
    }

    let grid: Grid<char> = data.into();

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    // Solve
    let start_directions = GridDirection::get_directions();

    for start_direction in start_directions {
        let mut current_direction = start_direction;
        let mut current_position = start_position;
        let mut steps: usize = 0;

        loop {
            steps += 1;
            let next_position =
                current_position.next_by_direction_with_check(&current_direction, max_x, max_y);

            if next_position.is_none() {
                break;
            }

            let next_value = grid.get_from_point(&next_position.unwrap());

            if next_position.unwrap() == start_position {
                result = steps / 2;
                break;
            }

            if !is_valid_move(next_value, current_direction) {
                break;
            }

            current_direction = next_direction(next_value.unwrap(), current_direction);
            current_position = next_position.unwrap();
        }
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut start_position: GridPoint = GridPoint::default();
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (y, line) in reader.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.unwrap().trim().chars().enumerate() {
            if c == 'S' {
                start_position.x = x;
                start_position.y = y;
            }
            row.push(c);
        }
        data.push(row);
    }

    let mut grid: Grid<char> = data.into();

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    // Solve
    let start_directions = GridDirection::get_directions();

    let mut pipe_loop_option = None;

    for start_direction in start_directions {
        let mut current_direction = start_direction;
        let mut current_position = start_position;
        let mut steps: Vec<GridPoint> = Vec::new();

        loop {
            steps.push(current_position);
            let next_position =
                current_position.next_by_direction_with_check(&current_direction, max_x, max_y);

            if next_position.is_none() {
                break;
            }

            let next_value = grid.get_from_point(&next_position.unwrap());

            if next_position.unwrap() == start_position {
                pipe_loop_option = Some(steps);
                break;
            }

            if !is_valid_move(next_value, current_direction) {
                break;
            }

            current_direction = next_direction(next_value.unwrap(), current_direction);
            current_position = next_position.unwrap();
        }
    }

    let point_iter = pipe_loop_option
        .unwrap()
        .into_iter()
        .map(|x| (x.x as i32, x.y as i32));

    let polygon = Polygon::new(LineString::from_iter(point_iter), vec![]);

    for y in 0..max_y {
        for x in 0..max_x {
            let point = Point::new(x as i32, y as i32);
            if polygon.contains(&point) {
                grid.set(x, y, 'I');
            }
        }
    }

    // grid.print_data();

    // Result
    println!("Result of part 2 is {}", grid.count_for(&'I'));
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
