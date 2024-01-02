use utils::{get_input_path, grid::Grid, parse_into_char_vector_vector};

fn run(input_file: &str) {
    // Preamble
    // Parse
    let mut grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    // Solve
    // Expand
    let mut y = 0;
    while y < grid.get_max_y() {
        if grid.data[y].iter().find(|&x| *x == '#').is_none() {
            grid.data.insert(y, grid.data[y].clone());
            y += 2;
            continue;
        }
        y += 1;
    }

    let mut x = 0;
    while x < grid.get_max_x() {
        let mut found_star = false;
        for y in 0..grid.get_max_y() {
            if grid.data[y][x] == '#' {
                found_star = true;
                break;
            }
        }

        if found_star {
            x += 1;
            continue;
        }

        for y in 0..grid.get_max_y() {
            grid.data[y].insert(x, '.');
        }

        x += 2;
    }

    // grid.print_data();

    // Solution
    let stars = grid.get_all_positions_for(&'#');

    let mut result = 0;
    for (i, s1) in stars.iter().enumerate() {
        for s2 in stars.iter().skip(i) {
            result += s1.manhatten_distance(s2);
        }
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    #[cfg(test)]
    const EXPANSION_VALUE: usize = 100;

    #[cfg(not(test))]
    const EXPANSION_VALUE: usize = 1_000_000;

    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    let mut distant_grid: Grid<usize> =
        Grid::with_width_height(grid.get_max_x(), grid.get_max_y(), 1);

    // Solve
    // Expand
    'row: for (y, row) in grid.data.iter().enumerate() {
        for column in row.iter() {
            if column == &'#' {
                continue 'row;
            }
        }

        for column in distant_grid.data[y].iter_mut() {
            *column = EXPANSION_VALUE;
        }
    }

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    'column: for x in 0..max_x {
        for y in 0..max_y {
            if grid.data[y][x] == '#' {
                continue 'column;
            }
        }
        for y in 0..max_y {
            distant_grid.data[y][x] = EXPANSION_VALUE;
        }
    }

    // Solution
    let stars = grid.get_all_positions_for(&'#');

    let mut results = Vec::new();

    for (i, s1) in stars.iter().enumerate() {
        for s2 in stars.iter().skip(i) {
            let min_x = s1.x.min(s2.x);
            let min_y = s1.y.min(s2.y);
            let max_x = s1.x.max(s2.x);
            let max_y = s1.y.max(s2.y);
            let mut result: usize = 0;

            for x in min_x..max_x {
                result += distant_grid.data[min_y][x];
            }

            for y in min_y..max_y {
                result += distant_grid.data[y][max_x];
            }

            results.push(result);
        }
    }

    // distant_grid.print_data();
    // Result
    println!("Result of part 2 is {}", results.iter().sum::<usize>());
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
