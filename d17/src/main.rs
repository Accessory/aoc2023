use std::collections::BinaryHeap;

use fnv::FnvHashSet;
use utils::{
    get_input_path, grid::Grid, grid_direction::GridDirection, grid_point::GridPoint,
    parse_into_usize_vector_vector_by_char,
};

const MAX_MOVES: usize = 9;
const MUST_MOVE: usize = 3;
const CAN_TURN_AFTER: usize = MAX_MOVES - MUST_MOVE;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GliderState {
    position: GridPoint,
    direction: GridDirection,
    heat_loss: usize,
    moves_left: usize,
    // visists: Vec<(GridPoint, GridDirection)>,
}

impl PartialOrd for GliderState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GliderState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl GliderState {
    pub fn hashing_values(&self) -> (GridPoint, GridDirection, usize) {
        (self.position, self.direction, self.moves_left)
    }

    fn generate_next_positions(
        &self,
        max_x: usize,
        max_y: usize,
    ) -> Vec<(GridPoint, GridDirection)> {
        let mut rtn = Vec::new();

        if self.moves_left <= CAN_TURN_AFTER {
            let next_directions = match self.direction {
                GridDirection::Up => [GridDirection::Left, GridDirection::Right],
                GridDirection::Right => [GridDirection::Up, GridDirection::Down],
                GridDirection::Down => [GridDirection::Left, GridDirection::Right],
                GridDirection::Left => [GridDirection::Up, GridDirection::Down],
            };

            for direction in next_directions {
                if let Some(next) = self
                    .position
                    .next_by_direction_with_check(&direction, max_x, max_y)
                {
                    rtn.push((next, direction));
                }
            }
        }

        if self.moves_left != 0 {
            if let Some(next) =
                self.position
                    .next_by_direction_with_check(&self.direction, max_x, max_y)
            {
                rtn.push((next, self.direction));
            }
        }

        rtn
    }
}

fn run(input_file: &str) {
    // Preamble
    // Parse
    let grid: Grid<usize> = parse_into_usize_vector_vector_by_char(input_file).into();

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    let goal = GridPoint::new(max_x - 1, max_y - 1);

    // Solve
    let mut seen: FnvHashSet<(GridPoint, GridDirection, usize)> = FnvHashSet::default();
    let start = [
        GliderState {
            position: GridPoint { x: 0, y: 0 },
            direction: GridDirection::Right,
            heat_loss: 0,
            moves_left: 3,
            // visists: Vec::new(),
        },
        GliderState {
            position: GridPoint { x: 0, y: 0 },
            direction: GridDirection::Down,
            heat_loss: 0,
            moves_left: 3,
            // visists: Vec::new(),
        },
    ];

    let mut queue: BinaryHeap<GliderState> = BinaryHeap::from(start);

    let mut result = None;

    while let Some(state) = queue.pop() {
        if state.position == goal {
            result = Some(state);
            break;
        }

        // if !seen.insert(state.hashing_values()) {
        //     continue;
        // }

        if !seen.insert(state.hashing_values()) {
            continue;
        }

        // Next gen
        let next_points = state.generate_next_positions(max_x, max_y);

        // let mut visits = state.visists;
        // visits.push((state.position, state.direction));

        for (next, direction) in next_points {
            let heat_loss = grid.get_from_point(&next).unwrap();
            let moves_left = if state.direction == direction {
                state.moves_left - 1
            } else {
                2
            };

            queue.push(GliderState {
                position: next,
                direction: direction,
                heat_loss: state.heat_loss + heat_loss,
                moves_left: moves_left,
                // visists: visits.clone(),
            })
        }
    }

    // print_data(&grid, result.clone());

    // Result
    println!(
        "Result of part 1 is {}",
        result.expect("No solution found.").heat_loss
    );
}

fn run2(input_file: &str) {
    // Preamble
    // Parse
    let grid: Grid<usize> = parse_into_usize_vector_vector_by_char(input_file).into();

    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    let goal = GridPoint::new(max_x - 1, max_y - 1);

    // Solve
    let mut seen: FnvHashSet<(GridPoint, GridDirection, usize)> = FnvHashSet::default();
    let start = [
        GliderState {
            position: GridPoint { x: 0, y: 0 },
            direction: GridDirection::Right,
            heat_loss: 0,
            moves_left: MAX_MOVES,
            // visists: Vec::new(),
        },
        GliderState {
            position: GridPoint { x: 0, y: 0 },
            direction: GridDirection::Down,
            heat_loss: 0,
            moves_left: MAX_MOVES,
            // visists: Vec::new(),
        },
    ];

    let mut queue: BinaryHeap<GliderState> = BinaryHeap::from(start);

    let mut result: Option<GliderState> = None;

    while let Some(state) = queue.pop() {
        if state.position == goal && state.moves_left <= CAN_TURN_AFTER {
            result = Some(state);
            // continue;
            break;
        }

        if !seen.insert(state.hashing_values()) {
            continue;
        }

        // Next gen
        let next_points = state.generate_next_positions(max_x, max_y);

        // let mut visits = state.visists;
        // visits.push((state.position, state.direction));

        for (next, direction) in next_points {
            let heat_loss = grid.get_from_point(&next).unwrap();
            let moves_left = if state.direction == direction {
                state.moves_left - 1
            } else {
                MAX_MOVES
            };

            queue.push(GliderState {
                position: next,
                direction: direction,
                heat_loss: state.heat_loss + heat_loss,
                moves_left: moves_left,
                // visists: visits.clone(),
            })
        }
    }

    // print_data(&grid, result.clone());

    // Result
    println!(
        "Result of part 2 is {}",
        result.expect("No solution found.").heat_loss
    );
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
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_3() {
        let input_path = get_test_input_2_path(file!());
        run2(input_path.to_str().unwrap());
    }
}

// fn print_data(grid: &Grid<usize>, result: Option<GliderState>) {
//     let max_x = grid.get_max_x();
//     let max_y = grid.get_max_y();
//     let gs = result.unwrap();
//     for y in 0..max_y {
//         for x in 0..max_x {
//             if let Some(visited) = gs.visists.iter().find(|i| i.0.x == x && i.0.y == y) {
//                 match visited.1 {
//                     GridDirection::Up => print!("{}", '^'),
//                     GridDirection::Right => print!("{}", '>'),
//                     GridDirection::Down => print!("{}", 'v'),
//                     GridDirection::Left => print!("{}", '<'),
//                 }
//             } else {
//                 print!("{}", grid.data[y][x]);
//             }
//         }
//         println!();
//     }
// }
