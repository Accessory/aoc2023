#![feature(array_windows)]
use geo::Area;
use geo::{LineString, Polygon};
use utils::{
    get_input_path, hash_point_map::HashPointMap, map_direction::MapDirection, parse_file_into,
    point::MapPoint,
};

#[derive(Debug)]
struct Instruction {
    direction: MapDirection,
    steps: usize,
    color: String,
}
impl Instruction {
    fn get_steps_from_hex(&self) -> i64 {
        i64::from_str_radix(&self.color[0..5], 16).unwrap()
    }

    fn get_direction_from_hex(&self) -> MapDirection {
        match self.color.chars().last().unwrap().to_digit(10).unwrap() {
            0 => MapDirection::Right,
            1 => MapDirection::Down,
            2 => MapDirection::Left,
            3 => MapDirection::Up,
            _ => panic!("Should not be here!"),
        }
    }
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let direction = value.chars().next().unwrap().into();
        let mut split = value.split(' ');
        let steps = split.nth(1).unwrap().parse().unwrap();
        let color_part = split.next().unwrap();
        let color = color_part[2..color_part.len() - 1].to_string();

        Self {
            direction,
            steps,
            color,
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut position = MapPoint::new(0, 0);
    let mut map = HashPointMap::default();
    // Parse
    let instructions: Vec<Instruction> = parse_file_into(input_file);

    for instruction in instructions {
        for _ in 0..instruction.steps {
            map.insert(position, '#');
            position.move_by_direction(&instruction.direction);
        }
    }

    // Solve
    let (min_x, max_x, min_y, max_y) = map.get_min_x_max_x_min_y_max_y();

    for y in min_y..=max_y {
        let mut fill = false;
        for x in min_x..=max_x {
            let has_hole = map.get_x_y(x, y).is_some();
            let is_wall = has_hole && map.get_x_y(x, y + 1).is_some();

            if is_wall {
                fill = !fill;
            }

            if fill {
                map.insert(MapPoint::new(x, y), '#');
            }
        }
    }

    // map.print_all('.');

    // Result
    println!("Result of part 1 is {}", map.len());
}

fn run2(input_file: &str) {
    // Preamble
    // Parse
    let instructions: Vec<Instruction> = parse_file_into(input_file);

    let mut position = MapPoint::new(0, 0);
    // let mut points = vec![position];
    let mut points = Vec::new();

    let mut steps_count = 0;

    for instruction in instructions {
        let direction = instruction.get_direction_from_hex();
        let steps = instruction.get_steps_from_hex();

        // let direction = instruction.direction;
        // let steps = instruction.steps as i64;
        steps_count += steps;
        position.move_by_direction_and_distance(direction, steps);
        points.push(position);
    }

    let rtn = calculate_area(&points) + steps_count / 2 + 1;

    let point_iter = points.into_iter().map(|p| (p.x as f64, p.y as f64));

    let polygon = Polygon::new(LineString::from_iter(point_iter), vec![]);

    let result = polygon.signed_area() + (steps_count as f64 / 2.0 + 1.0);

    // Result
    println!("Result of part 2 is {}", result);
}

fn calculate_area(points: &[MapPoint]) -> i64 {
    let mut xy = 0;
    let mut yx = 0;
    for [point1, point2] in points.array_windows() {
        xy += point1.x * point2.y;
        yx += point1.y * point2.x;
    }
    (xy - yx) / 2
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
