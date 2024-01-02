use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;
use utils::grid::Grid;

#[derive(Debug)]
enum ResultType {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Results {
    value: usize,
    result_type: ResultType,
}

impl Results {
    fn new(value: usize, result_type: ResultType) -> Self {
        Self { value, result_type }
    }

    fn to_result(&self) -> usize {
        match self.result_type {
            ResultType::Horizontal => self.value,
            ResultType::Vertical => self.value * 100,
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut results: Vec<Results> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut grids: Vec<Grid<char>> = Vec::new();

    let mut next_grid = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if line.is_empty() {
            grids.push(next_grid.into());
            next_grid = Vec::new();
            continue;
        }

        next_grid.push(line.chars().collect::<Vec<char>>());
    }
    grids.push(next_grid.into());

    // Solve
    for grid in grids {
        // Check Horizontal
        let (horizontal_max, horizontal_max_at) = check_horizontal(&grid);
        let (vertical_max, vertical_max_at) = check_vertical(&grid);

        if vertical_max >= horizontal_max {
            results.push(Results::new(vertical_max_at, ResultType::Vertical));
        } else {
            results.push(Results::new(horizontal_max_at, ResultType::Horizontal));
        }
    }

    // Result
    let result: usize = results.iter().map(|x| x.to_result()).sum();

    println!("Result of part 1 is {}", result);
}

fn check_horizontal(grid: &Grid<char>) -> (usize, usize) {
    let mut horizontal_max = 0;
    let mut horizontal_max_at = 0;
    'outer: for left in 0..grid.get_max_x() - 1 {
        let max_mirror_size = left.min(grid.get_max_x() - left - 2);

        if horizontal_max > max_mirror_size + 1 {
            continue 'outer;
        }

        for diff in 0..=max_mirror_size {
            let current_left = left - diff;
            let current_right = left + 1 + diff;
            if !does_vertical_lines_mirror(grid, current_left, current_right) {
                continue 'outer;
            }
        }

        horizontal_max = max_mirror_size + 1;
        horizontal_max_at = left + 1;
    }
    (horizontal_max, horizontal_max_at)
}

fn check_vertical(grid: &Grid<char>) -> (usize, usize) {
    let mut vertical_max = 0;
    let mut vertical_max_at = 0;
    'outer: for top in 0..grid.get_max_y() - 1 {
        let max_mirror_size = top.min(grid.get_max_y() - top - 2);

        if vertical_max > max_mirror_size + 1 {
            continue 'outer;
        }

        for diff in 0..=max_mirror_size {
            let current_top = top - diff;
            let current_bottom = top + 1 + diff;
            if !does_horizontal_lines_mirror(grid, current_top, current_bottom) {
                continue 'outer;
            }
        }

        vertical_max = max_mirror_size + 1;
        vertical_max_at = top + 1;
    }
    (vertical_max, vertical_max_at)
}

fn does_vertical_lines_mirror(grid: &Grid<char>, left: usize, right: usize) -> bool {
    for y in 0..grid.get_max_y() {
        if grid.data[y][left] != grid.data[y][right] {
            return false;
        }
    }
    true
}

fn does_horizontal_lines_mirror(grid: &Grid<char>, top: usize, bottom: usize) -> bool {
    for x in 0..grid.get_max_x() {
        if grid.data[top][x] != grid.data[bottom][x] {
            return false;
        }
    }
    true
}

fn run2(input_file: &str) {
    // Preamble
    let mut results: Vec<Results> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut grids: Vec<Grid<char>> = Vec::new();

    let mut next_grid = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if line.is_empty() {
            grids.push(next_grid.into());
            next_grid = Vec::new();
            continue;
        }

        next_grid.push(line.chars().collect::<Vec<char>>());
    }
    grids.push(next_grid.into());

    // Solve
    for grid in grids {
        let (horizontal_max, horizontal_max_at) = check_horizontal_2(&grid);
        let (vertical_max, vertical_max_at) = check_vertical_2(&grid);

        if vertical_max >= horizontal_max {
            results.push(Results::new(vertical_max_at, ResultType::Vertical));
        } else {
            results.push(Results::new(horizontal_max_at, ResultType::Horizontal));
        }
    }

    // Result
    let result: usize = results.iter().map(|x| x.to_result()).sum();

    println!("Result of part 2 is {}", result);
}

fn check_vertical_2(grid: &Grid<char>) -> (usize, usize) {
    let mut vertical_max = 0;
    let mut vertical_max_at = 0;
    'outer: for top in 0..grid.get_max_y() - 1 {
        let max_mirror_size = top.min(grid.get_max_y() - top - 2);

        if vertical_max > max_mirror_size + 1 {
            continue 'outer;
        }

        let mut errors: usize = 0;

        for diff in 0..=max_mirror_size {
            let current_top = top - diff;
            let current_bottom = top + 1 + diff;
            errors += does_horizontal_lines_mirror_2(grid, current_top, current_bottom);
        }

        if errors != 1 {
            continue 'outer;
        }

        vertical_max = max_mirror_size + 1;
        vertical_max_at = top + 1;
    }
    (vertical_max, vertical_max_at)
}

fn check_horizontal_2(grid: &Grid<char>) -> (usize, usize) {
    let mut horizontal_max = 0;
    let mut horizontal_max_at = 0;
    'outer: for left in 0..grid.get_max_x() - 1 {
        let max_mirror_size = left.min(grid.get_max_x() - left - 2);

        if horizontal_max > max_mirror_size + 1 {
            continue 'outer;
        }

        let mut errors: usize = 0;

        for diff in 0..=max_mirror_size {
            let current_left = left - diff;
            let current_right = left + 1 + diff;
            errors += does_vertical_lines_mirror_2(grid, current_left, current_right)
        }

        if errors != 1 {
            continue 'outer;
        }

        horizontal_max = max_mirror_size + 1;
        horizontal_max_at = left + 1;
    }
    (horizontal_max, horizontal_max_at)
}

fn does_vertical_lines_mirror_2(grid: &Grid<char>, left: usize, right: usize) -> usize {
    let mut rtn = 0;
    for y in 0..grid.get_max_y() {
        if grid.data[y][left] != grid.data[y][right] {
            rtn += 1;
        }
    }
    rtn
}

fn does_horizontal_lines_mirror_2(grid: &Grid<char>, top: usize, bottom: usize) -> usize {
    let mut rtn = 0;
    for x in 0..grid.get_max_x() {
        if grid.data[top][x] != grid.data[bottom][x] {
            rtn += 1;
        }
    }
    rtn
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
