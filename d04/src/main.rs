use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    const VALUES: [usize; 11] = [0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
    let mut result = 0;

    // Parse & Solve
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split = line.split(":");
        let number_part = split.skip(1).next().unwrap();
        let mut number_split = number_part.split("|");
        let left_numbers_part = number_split.next().unwrap();
        let right_numbers_part = number_split.next().unwrap();

        let left_numbers: Vec<usize> = left_numbers_part
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let right_numbers: Vec<usize> = right_numbers_part
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let intersections = right_numbers
            .iter()
            .filter(|n| left_numbers.contains(&n))
            .count();

        result += VALUES[intersections];
    }

    // Result
    println!("Result of part 1 is {}", result);
}

#[allow(dead_code)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    your_numbers: Vec<usize>,
    intersections: usize,
    points: usize,
    copies: usize,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Id: {}, Intersections: {}, Copies: {}", self.id, self.intersections, self.copies)
    }
}

impl Card {
    fn new(id: usize, winning_numbers: Vec<usize>, your_numbers: Vec<usize>) -> Self {
        const VALUES: [usize; 11] = [0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
        let intersections = your_numbers
            .iter()
            .filter(|n| winning_numbers.contains(&n))
            .count();
        Self {
            id,
            winning_numbers,
            your_numbers,
            intersections,
            points: VALUES[intersections],
            copies: 1,
        }
    }
}

fn run2(input_file: &str) {
    // Preamble
    let mut cards = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split(":");
        let game_part = split.next().unwrap();
        let id = game_part[5..].trim_start().parse().unwrap();
        let number_part = split.next().unwrap();
        let mut number_split = number_part.split("|");
        let left_numbers_part = number_split.next().unwrap();
        let right_numbers_part = number_split.next().unwrap();

        let left_numbers: Vec<usize> = left_numbers_part
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let right_numbers: Vec<usize> = right_numbers_part
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        cards.push(Card::new(id, left_numbers, right_numbers))
    }

    // Solve
    for pos in 0..cards.len() {
        let intersections = cards[pos].intersections;
        let end = cards.len().min(pos + intersections + 1);
        for to_add_idx in pos + 1..end {
            cards[to_add_idx].copies += cards[pos].copies;
        }
    }

    // Result
    let result = cards.iter().map(|c| c.copies).sum::<usize>();
    // cards.iter().for_each(|c| println!("{c}"));
    println!("Result of part 2 is {result}");
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
