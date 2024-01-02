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
            ResultType::Horizontal => self.value + 1,
            ResultType::Vertical => (self.value + 1) * 100,
        }
    }
}

fn check_vertical(grid: &Grid<char>) -> (usize, usize) {
    let mut vertical_max = usize::MAX;
    let mut vertical_max_at = usize::MAX;

    for y in 0..grid.get_max_x() {
        let (inner_vertical_max, inner_vertical_max_at) = check_vertical_at(grid, y);

        if vertical_max_at != 0 && inner_vertical_max != vertical_max_at {
            return (0, 0);
        }

        vertical_max = inner_vertical_max.min(vertical_max);
        vertical_max_at = inner_vertical_max_at.min(vertical_max_at);
    }

    (vertical_max, vertical_max_at)
}

fn check_vertical_at(grid: &Grid<char>, x: usize) -> (usize, usize) {
    let mut vertical_max = 0;
    let mut vertical_max_at = 0;
    let max_y = grid.get_max_y();

    'top_start: for top_start in 0..max_y {
        let max_mirror_size = top_start.min(max_y - top_start - 1);

        if vertical_max > max_mirror_size {
            break;
        }

        for diff in 0..max_mirror_size {
            let top = top_start - diff;
            let bottom = top_start + 1 + diff;

            if grid.data[top][x] != grid.data[bottom][x] {
                continue 'top_start;
            }
        }
        vertical_max = max_mirror_size;
        vertical_max_at = top_start;
    }
    (vertical_max, vertical_max_at)
}

fn check_horizontal(grid: &Grid<char>) -> (usize, usize) {
    let mut horizontal_max = usize::MAX;
    let mut horizontal_max_at = usize::MAX;

    for y in 0..grid.get_max_y() {
        let (inner_horizontal_max, inner_horizontal_max_at) = check_horizontal_at(grid, y);

        if horizontal_max_at != 0 && inner_horizontal_max_at != horizontal_max_at {
            return (0, 0);
        }
        horizontal_max = inner_horizontal_max.min(horizontal_max);
        horizontal_max_at = inner_horizontal_max_at.min(horizontal_max_at);
    }

    (horizontal_max, horizontal_max_at)
}

fn check_horizontal_at(grid: &Grid<char>, y: usize) -> (usize, usize) {
    let mut horizontal_max = 0;
    let mut horizontal_max_at = 0;
    let max_x = grid.get_max_x();

    'left_start: for left_start in 0..max_x {
        let max_mirror_size = left_start.min(max_x - left_start - 1);

        if horizontal_max > max_mirror_size {
            break;
        }

        for diff in 0..max_mirror_size {
            let left = left_start - diff;
            let right = left_start + 1 + diff;

            if grid.data[y][left] != grid.data[y][right] {
                continue 'left_start;
            }
        }
        horizontal_max = max_mirror_size;
        horizontal_max_at = left_start;
    }
    (horizontal_max, horizontal_max_at)
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
        let (horizontal_max, horizontal_max_at) = check_horizontal_2(&grid);

        // Check Vertical
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

fn check_horizontal_2(grid: &Grid<char>) -> (usize, usize) {
    for left in 0..grid.get_max_x()-1{

    }
    (0,0)
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
