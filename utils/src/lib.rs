#![feature(array_windows)]
use std::{
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

pub mod grid;
pub mod grid_direction;
pub mod grid_point;
pub mod grid_walker;
pub mod hash_point_map;
pub mod map;
pub mod map_direction;
pub mod point;
pub mod utils;
pub mod vector3;

pub fn parse_file_into<T>(input_file: &str) -> Vec<T>
where
    T: From<String>,
{
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap().into())
        .collect::<Vec<T>>()
}

pub fn parse_into_char_vector_vector(input_file: &str) -> Vec<Vec<char>> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut rtn = Vec::new();

    for line in reader.lines() {
        rtn.push(line.unwrap().trim().chars().collect())
    }

    rtn
}

pub fn parse_into_i64_vector_vector(input_file: &str) -> Vec<Vec<i64>> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split(' ')
                .into_iter()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

pub fn parse_into_usize_vector_vector(input_file: &str) -> Vec<Vec<usize>> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split(' ')
                .into_iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

pub fn parse_into_usize_vector_vector_by_char(input_file: &str) -> Vec<Vec<usize>> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|n| n.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

pub fn parse_into_usize_vector(input_file: &str) -> Vec<usize> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .parse::<usize>()
                .expect("Could not parse \"{l}\" into usize")
        })
        .collect::<Vec<usize>>()
}

pub fn parse_into_i64_vector(input_file: &str) -> Vec<i64> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .parse::<i64>()
                .expect("Could not parse \"{l}\" into i64")
        })
        .collect::<Vec<i64>>()
}

pub fn get_input_path(src_path: &str) -> PathBuf {
    let file_path = Path::new(src_path);
    if Path::exists(file_path) {
        file_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("input")
            .join("input.txt")
    } else {
        current_dir().unwrap().join("input").join("input.txt")
    }
}

pub fn get_test_input_path(src_path: &str) -> PathBuf {
    let file_path = Path::new(src_path);
    if Path::exists(file_path) {
        file_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("input")
            .join("input_test.txt")
    } else {
        current_dir().unwrap().join("input").join("input_test.txt")
    }
}

pub fn get_test_input_2_path(src_path: &str) -> PathBuf {
    let file_path = Path::new(src_path);
    if Path::exists(file_path) {
        file_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("input")
            .join("input_test_2.txt")
    } else {
        current_dir()
            .unwrap()
            .join("input")
            .join("input_test_2.txt")
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (x.min(y), x.max(y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use crate::get_input_path;
    use crate::get_test_input_path;
    use crate::map::Map;
    use crate::map::Point;

    #[test]
    fn test_get_test_input_path() {
        println!("{}", get_test_input_path(file!()).to_string_lossy());
    }

    #[test]
    fn test_get_input_path() {
        println!("{}", get_input_path(file!()).to_string_lossy());
    }

    #[test]
    fn test_map() {
        let mut map = Map::new(10, 10);
        map.set(5, 5, 10);
        let rtn = map.get(5, 5);
        let rtn2 = map.get_from_point(Point { x: 5, y: 5 });
        assert!(rtn == 10);
        assert!(rtn2 == 10);
    }

    #[test]
    fn test_point() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 5, y: 5 };
        let distance = p1.manhatten_distance(p2);
        assert!(distance == 10);
    }
}
