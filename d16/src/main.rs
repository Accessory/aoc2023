#![feature(coroutines)]
use std::{collections::HashSet, usize};

use utils::{
    get_input_path, grid::Grid, grid_direction::GridDirection, grid_point::GridPoint,
    grid_walker::GridWalker, parse_into_char_vector_vector,
};

fn generate_all_starts(max_x: usize, max_y: usize) -> Vec<GridWalker> {
    let mut rtn = Vec::new();

    for x in 0..max_x {
        rtn.push(GridWalker::new(GridPoint::new(x, 0), GridDirection::Down));
        rtn.push(GridWalker::new(
            GridPoint::new(x, max_y - 1),
            GridDirection::Up,
        ));
    }

    for y in 0..max_y {
        rtn.push(GridWalker::new(GridPoint::new(0, y), GridDirection::Right));
        rtn.push(GridWalker::new(
            GridPoint::new(max_x - 1, y),
            GridDirection::Left,
        ));
    }

    rtn
}

// fn print_map(energized: &HashSet<utils::grid_point::GridPoint>, max_x: usize, max_y: usize) {
//     for y in 0..max_y {
//         for x in 0..max_x {
//             if energized.contains(&GridPoint::new(x, y)) {
//                 print!("{}", '#');
//             } else {
//                 print!("{}", '.');
//             }
//         }
//         println!();
//     }
// }

fn run(input_file: &str) {
    // Preamble
    let mut beams = vec![GridWalker::new(GridPoint::new(0, 0), GridDirection::Right)];
    let mut seen: HashSet<GridWalker> = HashSet::new();

    let mut energized = HashSet::new();
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    let beams_deref = unsafe { &mut *std::ptr::addr_of_mut!(beams) };

    // Solve
    loop {
        for idx in (0..beams.len()).rev() {
            let beam = beams.get_mut(idx).unwrap();
            energized.insert(beam.position);
            if !seen.insert(*beam) {
                beams.remove(idx);
                continue;
            }

            let c = beam.get_from_grid(&grid).unwrap();

            match c {
                '.' => {}
                '|' => {
                    if beam.direction == GridDirection::Left
                        || beam.direction == GridDirection::Right
                    {
                        beam.direction = GridDirection::Up;
                        beams_deref.push(GridWalker::new(beam.position, GridDirection::Down));
                    }
                }
                '-' => {
                    if beam.direction == GridDirection::Up || beam.direction == GridDirection::Down
                    {
                        beam.direction = GridDirection::Left;
                        beams_deref.push(GridWalker::new(beam.position, GridDirection::Right));
                    }
                }
                '\\' => {
                    beam.direction = match beam.direction {
                        GridDirection::Up => GridDirection::Left,
                        GridDirection::Right => GridDirection::Down,
                        GridDirection::Down => GridDirection::Right,
                        GridDirection::Left => GridDirection::Up,
                    };
                }
                '/' => {
                    beam.direction = match beam.direction {
                        GridDirection::Up => GridDirection::Right,
                        GridDirection::Right => GridDirection::Up,
                        GridDirection::Down => GridDirection::Left,
                        GridDirection::Left => GridDirection::Down,
                    };
                }
                _ => panic!("Should not be here."),
            }

            if !beam.move_with_check(max_x, max_y) {
                beams.remove(idx);
                continue;
            }
        }

        if beams.is_empty() {
            break;
        }
    }
    // print_map(&energized, max_x, max_y);

    // Result
    println!("Result of part 1 is {}", energized.len());
}

fn run2(input_file: &str) {
    // Preamble
    let mut result = 0;
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    let starts = generate_all_starts(max_x, max_y);

    // Solve
    for start in starts {
        let mut seen: HashSet<GridWalker> = HashSet::new();
        let mut energized = HashSet::new();
        let mut beams = vec![start];
        let mut next = Vec::new();

        loop {
            beams.append(&mut next);
            for idx in (0..beams.len()).rev() {
                let beam = beams.get_mut(idx).unwrap();

                energized.insert(beam.position);
                if !seen.insert(*beam) {
                    beams.remove(idx);
                    continue;
                }

                let c = beam.get_from_grid(&grid).unwrap();

                match c {
                    '.' => {}
                    '|' => {
                        if beam.direction == GridDirection::Left
                            || beam.direction == GridDirection::Right
                        {
                            beam.direction = GridDirection::Up;
                            next.push(GridWalker::new(beam.position, GridDirection::Down));
                        }
                    }
                    '-' => {
                        if beam.direction == GridDirection::Up
                            || beam.direction == GridDirection::Down
                        {
                            beam.direction = GridDirection::Left;
                            next.push(GridWalker::new(beam.position, GridDirection::Right));
                        }
                    }
                    '\\' => {
                        beam.direction = match beam.direction {
                            GridDirection::Up => GridDirection::Left,
                            GridDirection::Right => GridDirection::Down,
                            GridDirection::Down => GridDirection::Right,
                            GridDirection::Left => GridDirection::Up,
                        };
                    }
                    '/' => {
                        beam.direction = match beam.direction {
                            GridDirection::Up => GridDirection::Right,
                            GridDirection::Right => GridDirection::Up,
                            GridDirection::Down => GridDirection::Left,
                            GridDirection::Left => GridDirection::Down,
                        };
                    }
                    _ => panic!("Should not be here."),
                }

                if !beam.move_with_check(max_x, max_y) {
                    beams.remove(idx);
                    continue;
                }
            }

            if beams.is_empty() {
                break;
            }
        }
        result = result.max(energized.len());
    }
    // print_map(&energized, max_x, max_y);

    // Result
    println!("Result of part 2 is {}", result);
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
