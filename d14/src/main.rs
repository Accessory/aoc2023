use utils::{get_input_path, grid::Grid, grid_point::GridPoint, parse_into_char_vector_vector};

fn tilt_south(values: &mut Grid<char>) {
    let mut swap_point = GridPoint::default();
    let mut needs_new_swappoint;
    for x in 0..values.get_max_x() {
        needs_new_swappoint = true;
        for y in (0..values.get_max_y()).rev() {
            if values.data[y][x] == '#' {
                needs_new_swappoint = true;
                continue;
            }
            if needs_new_swappoint {
                if values.data[y][x] == '.' {
                    swap_point = GridPoint::new(x, y);
                    needs_new_swappoint = false;
                    continue;
                }
            }

            if !needs_new_swappoint {
                if values.data[y][x] == 'O' {
                    values.swap(swap_point.x, swap_point.y, x, y);
                    swap_point.move_up();
                }
            }
        }
    }
}

fn tilt_west(values: &mut Grid<char>) {
    let mut swap_point = GridPoint::default();
    let mut needs_new_swappoint;
    for y in 0..values.get_max_y() {
        needs_new_swappoint = true;
        for x in (0..values.get_max_x()).rev() {
            if values.data[y][x] == '#' {
                needs_new_swappoint = true;
                continue;
            }
            if needs_new_swappoint {
                if values.data[y][x] == '.' {
                    swap_point = GridPoint::new(x, y);
                    needs_new_swappoint = false;
                    continue;
                }
            }

            if !needs_new_swappoint {
                if values.data[y][x] == 'O' {
                    values.swap(swap_point.x, swap_point.y, x, y);
                    swap_point.move_left();
                }
            }
        }
    }
}

fn tilt_east(values: &mut Grid<char>) {
    let mut swap_point = GridPoint::default();
    let mut needs_new_swappoint;
    for y in 0..values.get_max_y() {
        needs_new_swappoint = true;
        for x in 0..values.get_max_x() {
            if values.data[y][x] == '#' {
                needs_new_swappoint = true;
                continue;
            }
            if needs_new_swappoint {
                if values.data[y][x] == '.' {
                    swap_point = GridPoint::new(x, y);
                    needs_new_swappoint = false;
                    continue;
                }
            }

            if !needs_new_swappoint {
                if values.data[y][x] == 'O' {
                    values.swap(swap_point.x, swap_point.y, x, y);
                    swap_point.move_right();
                }
            }
        }
    }
}

fn tilt_north(values: &mut Grid<char>) {
    let mut swap_point = GridPoint::default();
    let mut needs_new_swappoint;
    for x in 0..values.get_max_x() {
        needs_new_swappoint = true;
        for y in 0..values.get_max_y() {
            if values.data[y][x] == '#' {
                needs_new_swappoint = true;
                continue;
            }
            if needs_new_swappoint {
                if values.data[y][x] == '.' {
                    swap_point = GridPoint::new(x, y);
                    needs_new_swappoint = false;
                    continue;
                }
            }

            if !needs_new_swappoint {
                if values.data[y][x] == 'O' {
                    values.swap(swap_point.x, swap_point.y, x, y);
                    swap_point.move_down();
                }
            }
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    // Parse
    let mut values: Grid<char> = parse_into_char_vector_vector(input_file).into();

    // Solve
    tilt_north(&mut values);

    // Result
    // values.print_data();
    let rocks = values.get_all_positions_for(&'O');
    let mut result = 0;
    let max_y = values.get_max_y();
    for rock in rocks {
        result += max_y - rock.y;
    }

    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    const ROUNDS: usize = 1_000_000_000;
    let mut seen: Vec<u64> = Vec::new();
    
    // Parse
    let mut values: Grid<char> = parse_into_char_vector_vector(input_file).into();

    // Solve
    let mut round = 0;
    let mut hash = 0;
    while round < ROUNDS {
        tilt_north(&mut values);
        tilt_east(&mut values);
        tilt_south(&mut values);
        tilt_west(&mut values);
        hash = values.get_hash();
        if seen.contains(&hash) {
            break;
        }
        seen.push(hash);
        round += 1;
    }

    // Calculate rest
    let (hash_encounter_position, _) = seen.iter().enumerate().find(|(_, &i)| i == hash).unwrap();

    let round_delta = round - hash_encounter_position;
    let remaining_rounds = ROUNDS - round;
    let last_remaing_rounds = remaining_rounds % round_delta;
    let finished_rounds = ROUNDS - last_remaing_rounds + 1;
    let remaining_rounds = ROUNDS - finished_rounds;

    for _ in 0..remaining_rounds {
        tilt_north(&mut values);
        tilt_east(&mut values);
        tilt_south(&mut values);
        tilt_west(&mut values);
    }

    // Result
    // values.print_data();
    let rocks = values.get_all_positions_for(&'O');
    let mut result = 0;
    let max_y = values.get_max_y();
    for rock in rocks {
        result += max_y - rock.y;
    }

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
