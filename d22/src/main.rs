use std::collections::{HashMap, HashSet, VecDeque};

use utils::{get_input_path, parse_file_into, vector3::Vector3};

#[derive(Debug)]
struct Brick {
    start: Vector3,
    end: Vector3,
}
impl Brick {
    fn min_z(&self) -> i64 {
        self.start.z.min(self.end.z)
    }

    fn get_coords_with_z(&self, z: i64) -> Vec<Vector3> {
        let mut rtn = Vec::new();
        for y in self.start.y..=self.end.y {
            for x in self.start.x..=self.end.x {
                rtn.push(Vector3 { x, y, z });
            }
        }
        rtn
    }

    fn get_coords_from_z(&self, from_z: i64) -> Vec<Vector3> {
        let mut rtn = Vec::new();
        let min_z = self.min_z();
        for z in self.start.z..=self.end.z {
            for y in self.start.y..=self.end.y {
                for x in self.start.x..=self.end.x {
                    rtn.push(Vector3 {
                        x,
                        y,
                        z: z - min_z + from_z,
                    });
                }
            }
        }
        rtn
    }
}

impl From<String> for Brick {
    fn from(value: String) -> Self {
        let mut start_end = value.split("~");
        let mut start_split = start_end.next().unwrap().split(',');
        let x_start = start_split.next().unwrap().parse().unwrap();
        let y_start = start_split.next().unwrap().parse().unwrap();
        let z_start = start_split.next().unwrap().parse().unwrap();

        let start = Vector3::new(x_start, y_start, z_start);

        let mut end_split = start_end.next().unwrap().split(',');
        let x_end = end_split.next().unwrap().parse().unwrap();
        let y_end = end_split.next().unwrap().parse().unwrap();
        let z_end = end_split.next().unwrap().parse().unwrap();

        let end = Vector3::new(x_end, y_end, z_end);

        Self { start, end }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut map: HashMap<Vector3, usize> = HashMap::new();

    // Parse
    let mut bricks: Vec<Brick> = parse_file_into(input_file);
    bricks.sort_unstable_by(|l, r| l.min_z().cmp(&r.min_z()));
    let mut support_map: Vec<HashSet<usize>> = Vec::with_capacity(bricks.len());

    for (idx, brick) in bricks.iter().enumerate() {
        support_map.push(HashSet::new());
        let min_z = brick.min_z();
        for z in (0..min_z).rev() {
            let coords: Vec<Vector3> = brick.get_coords_with_z(z);
            let blocks: Vec<&Vector3> = coords.iter().filter(|c| map.contains_key(c)).collect();

            for block in blocks.iter() {
                let get_support_from = map.get(&block).unwrap();
                let sme = support_map.get_mut(*get_support_from).unwrap();
                sme.insert(idx);
            }

            if !blocks.is_empty() {
                map.extend(brick.get_coords_from_z(z + 1).into_iter().map(|c| (c, idx)));
                break;
            } else if z == 0 {
                map.extend(brick.get_coords_from_z(1).into_iter().map(|c| (c, idx)));
                break;
            }
        }
    }

    // print_from_x(&map);
    // println!();
    // print_from_y(&map);

    // Solve
    let mut result: usize = 0;
    let mut supported_by_list = Vec::new();
    for i in 0..bricks.len() {
        let count = support_map.iter().filter(|v| v.contains(&i)).count();
        supported_by_list.push(count);
    }
    'outer: for i in 0..bricks.len() {
        let needs_to_support = support_map.get(i).unwrap();
        for nts in needs_to_support {
            if supported_by_list[*nts] < 2 {
                continue 'outer;
            }
        }
        result += 1;
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut map: HashMap<Vector3, usize> = HashMap::new();
    // Parse
    let mut bricks: Vec<Brick> = parse_file_into(input_file);
    bricks.sort_unstable_by(|l, r| l.min_z().cmp(&r.min_z()));
    let mut support_map: Vec<HashSet<usize>> = Vec::with_capacity(bricks.len());

    for (idx, brick) in bricks.iter_mut().enumerate() {
        support_map.push(HashSet::new());
        let min_z = brick.min_z();
        for z in (0..min_z).rev() {
            let coords: Vec<Vector3> = brick.get_coords_with_z(z);
            let blocks: Vec<&Vector3> = coords.iter().filter(|c| map.contains_key(c)).collect();

            for block in blocks.iter() {
                let get_support_from = map.get(&block).unwrap();
                let sme = support_map.get_mut(*get_support_from).unwrap();
                sme.insert(idx);
            }

            if !blocks.is_empty() {
                map.extend(brick.get_coords_from_z(z + 1).into_iter().map(|c| (c, idx)));
                let diff = brick.start.z - (z + 1);
                brick.start.z -= diff;
                brick.end.z -= diff;
                break;
            } else if z == 0 {
                map.extend(brick.get_coords_from_z(1).into_iter().map(|c| (c, idx)));
                let diff = brick.start.z - (1);
                brick.start.z -= diff;
                brick.end.z -= diff;
                break;
            }
        }
    }

    let mut result = 0;

    for start in 0..bricks.len() {
        let mut map_t = map.clone();

        let mut queue: VecDeque<usize> = VecDeque::from([start]);

        while let Some(bv) = queue.pop_front() {
            map_t.retain(|_, v| *v != bv);

            's: for &support in support_map[bv].iter() {
                let z = bricks[support].min_z() - 1;
                let coords = bricks[support].get_coords_with_z(z);
                for coord in coords {
                    if map_t.contains_key(&coord) {
                        continue 's;
                    }
                }
                result += 1;
                queue.push_back(support);
            }
        }
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

// fn print_from_x(map: &HashMap<Vector3, usize>) {
//     println!("Print from x view");
//     let mut max_x = 0;
//     let mut max_y = 0;
//     let mut max_z = 0;

//     for v in map.iter() {
//         max_x = v.0.x.max(max_x);
//         max_y = v.0.y.max(max_y);
//         max_z = v.0.z.max(max_z);
//     }

//     for z in (0..=max_z).rev() {
//         'x: for x in 0..=max_x {
//             for y in 0..=max_y {
//                 if let Some(value) = map.get(&Vector3::new(x, y, z)) {
//                     print!("{}", value % 10);
//                     continue 'x;
//                 }
//             }
//             print!(".");
//         }
//         println!();
//     }
// }

// fn print_from_y(map: &HashMap<Vector3, usize>) {
//     println!("Print from y view");
//     let mut max_x = 0;
//     let mut max_y = 0;
//     let mut max_z = 0;

//     for v in map.iter() {
//         max_x = v.0.x.max(max_x);
//         max_y = v.0.y.max(max_y);
//         max_z = v.0.z.max(max_z);
//     }

//     for z in (0..=max_z).rev() {
//         'y: for y in 0..=max_y {
//             for x in 0..=max_x {
//                 if let Some(value) = map.get(&Vector3::new(x, y, z)) {
//                     print!("{}", value % 10);
//                     continue 'y;
//                 }
//             }
//             print!(".");
//         }
//         println!();
//     }
// }
